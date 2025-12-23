use flo_canvas::{Draw, DrawingTarget};

use crate::{
    engine::{self, directions::squares::Square, position::Position},
    gui::{
        state_machine::{
            self, FromSquareSelectedData, GameState, NoSquareSelectedData, PromoteFunctionData,
            PromotionSquareSelectedData, SelectFromSquareFunctionData, SelectToSquareFunctionData,
            SquareSelected, StateFunction,
        },
        ui_board::Orientation,
        ui_element::{CanvasCoordinate, UIElement},
        ui_game::UIGame,
    },
};

pub struct Game {
    canvas: DrawingTarget,
    ui: UIGame,
    state: GameState,
    position: Position,
}
impl Game {
    pub fn new(canvas: DrawingTarget) -> Self {
        Game {
            canvas,
            ui: UIGame::new(),
            state: GameState::NoGame,
            position: Position::default(),
        }
    }
    pub fn draw(&mut self) {
        let mut gc: Vec<Draw> = Vec::new();
        self.ui.draw(&mut gc);
        self.canvas.draw(|graphics_context| {
            graphics_context.extend(gc);
        });
    }
    fn update_ui(&mut self) {
        self.ui.update(&self.position, &self.state);
        self.draw();
    }

    pub fn handle_click_event(&mut self, canvas_coordinate: CanvasCoordinate) {
        if let Some(event) = self.ui.dispatch_event(canvas_coordinate) {
            if let Some(state_function) = state_machine::get_function(event, &self.state) {
                match state_function {
                    StateFunction::NewGameAs(color) => self.new_game_as(color),
                    StateFunction::SelectFromSquare(data) => self.select_from_square(data),
                    StateFunction::SelectToSquare(data) => self.select_to_square(data),
                    StateFunction::Promote(data) => self.promote(data),
                    StateFunction::TurnBoard => self.turn_board(),
                }
            }
        }
    }

    fn new_game_as(&mut self, color: engine::piece::Color) {
        self.position = Position::new_starting_position();

        let orientation = match color {
            engine::piece::Color::Black => Orientation::WhiteDown,
            engine::piece::Color::White => Orientation::WhiteUp,
        };
        self.ui.set_orientation(orientation);
        self.update_ui();

        match color {
            engine::piece::Color::Black => {
                self.state = GameState::Computer;
                self.position = Position::new_starting_position();
                self.execute_computer_move();
            }
            engine::piece::Color::White => {
                self.position = Position::new_starting_position();
                match engine::get_possible_moves(&self.position) {
                    Ok(possible_moves) => {
                        self.state = GameState::Player(SquareSelected::No(NoSquareSelectedData {
                            possible_moves: map_possible_moves(possible_moves),
                        }));
                    }
                    Err(_) => self.state = GameState::NoGame,
                }
            }
        }
    }
    fn turn_board(&mut self) {
        self.ui.turn_board();
        self.update_ui();
    }
    fn select_from_square(&mut self, data: SelectFromSquareFunctionData) {
        let possible_moves_from = get_selected_moves_from(data.possible_moves.clone(), data.from);
        if possible_moves_from.is_empty() {
            self.state = GameState::Player(SquareSelected::No(NoSquareSelectedData {
                possible_moves: data.possible_moves.clone(),
            }));
        } else {
            self.state = GameState::Player(SquareSelected::From(FromSquareSelectedData {
                possible_moves: data.possible_moves.clone(),
                possible_moves_from,
                from: data.from,
            }));
        }
        self.update_ui();
    }

    fn select_to_square(&mut self, data: SelectToSquareFunctionData) {
        match get_selected_moves_to(data.possible_moves_from, data.to) {
            Some(SimpleMoveOrPromotions::SimpleMove(ui_move)) => {
                self.execute_player_move(ui_move.position);
            }
            Some(SimpleMoveOrPromotions::Promotions(possible_promotion_moves)) => {
                self.enable_promotion_buttons(possible_promotion_moves);
            }
            None => {
                self.state = GameState::Player(SquareSelected::No(NoSquareSelectedData {
                    possible_moves: data.possible_moves.clone(),
                }));
                self.update_ui();
            }
        }
    }

    fn promote(&mut self, data: PromoteFunctionData) {
        self.ui.disabled_promotion_buttons();
        if let Some(promotion_move) = data.possible_promotion_moves.iter().find(|promotion_move| {
            promotion_move.position.get_promotion_piece() == Some(data.piece)
        }) {
            self.position = promotion_move.position;
            self.state = GameState::Computer;
            self.execute_computer_move();
        }
    }

    fn enable_promotion_buttons(&mut self, possible_promotion_moves: Vec<UIMove>) {
        self.state = GameState::Player(SquareSelected::Promotion(
            PromotionSquareSelectedData::from(possible_promotion_moves),
        ));
        self.update_ui();
    }

    fn execute_computer_move(&mut self) {
        match engine::get_next_move(&self.position) {
            engine::MoveOrEnd::Move(position) => {
                self.position = position;
                match engine::get_possible_moves(&self.position) {
                    Ok(possible_moves) => {
                        self.state = GameState::Player(SquareSelected::No(NoSquareSelectedData {
                            possible_moves: map_possible_moves(possible_moves),
                        }));
                    }
                    Err(_) => {
                        self.state = GameState::GameOver;
                    }
                }
            }
            engine::MoveOrEnd::GameEnd(game_end) => {
                println!("{:?}", game_end);
                self.state = GameState::GameOver;
            }
        }
        self.update_ui();
    }

    fn execute_player_move(&mut self, position: Position) {
        self.position = position;
        self.state = GameState::Computer;
        self.update_ui();
        self.execute_computer_move();
    }
}

fn map_possible_moves(possible_moves: Vec<Position>) -> Vec<UIMove> {
    let mut ui_possible_moves: Vec<UIMove> = Vec::new();
    for possible_move in possible_moves {
        if let (Some(from), Some(to)) = (
            possible_move.get_from_square(),
            possible_move.get_to_square(),
        ) {
            ui_possible_moves.push(UIMove {
                from,
                to,
                position: possible_move,
            });
        }
    }
    ui_possible_moves
}

fn get_selected_moves_from(possible_moves: Vec<UIMove>, from: Square) -> Vec<UIMove> {
    possible_moves
        .into_iter()
        .filter(|ui_move| from == ui_move.from)
        .collect()
}

fn get_selected_moves_to(
    possible_moves: Vec<UIMove>,
    to: Square,
) -> Option<SimpleMoveOrPromotions> {
    let mut moves_from_to: Vec<UIMove> = possible_moves
        .into_iter()
        .filter(|ui_move| to == ui_move.to)
        .collect();
    if moves_from_to.len() == 4 {
        Some(SimpleMoveOrPromotions::Promotions(moves_from_to))
    } else {
        moves_from_to.pop().map(SimpleMoveOrPromotions::SimpleMove)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UIMove {
    pub from: Square,
    pub to: Square,
    pub position: Position,
}

enum SimpleMoveOrPromotions {
    SimpleMove(UIMove),
    Promotions(Vec<UIMove>),
}
