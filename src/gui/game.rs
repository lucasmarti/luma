use flo_canvas::{Draw, DrawingTarget};

use crate::{
    engine::{
        self,
        chess_moves::{ChessMove, MoveType},
        directions::squares::Square,
        position::Position,
    },
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
                            possible_moves,
                            last_move: None,
                        }));
                        self.update_ui();
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
                last_move: None,
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
                    last_move: None,
                }));
                self.update_ui();
            }
        }
    }

    fn promote(&mut self, data: PromoteFunctionData) {
        self.ui.disabled_promotion_buttons();
        if let Some(promotion_move) = data.possible_promotion_moves.iter().find(|promotion_move| {
            (promotion_move.move_type == MoveType::Promotion
                || promotion_move.move_type == MoveType::PromotionCapture)
                && promotion_move.pormotion == Some(data.piece)
        }) {
            self.position = promotion_move.position;
            self.state = GameState::Computer;
            self.execute_computer_move();
        }
    }

    fn enable_promotion_buttons(&mut self, possible_promotion_moves: Vec<ChessMove>) {
        self.state = GameState::Player(SquareSelected::Promotion(
            PromotionSquareSelectedData::from(possible_promotion_moves),
        ));
        self.update_ui();
    }

    fn execute_computer_move(&mut self) {
        match engine::get_next_move(&self.position) {
            engine::MoveOrEnd::Move(chess_move) => {
                self.position = chess_move.position;
                match engine::get_possible_moves(&self.position) {
                    Ok(possible_moves) => {
                        self.state = GameState::Player(SquareSelected::No(NoSquareSelectedData {
                            possible_moves,
                            last_move: Some(chess_move),
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

fn get_selected_moves_from(possible_moves: Vec<ChessMove>, from: Square) -> Vec<ChessMove> {
    possible_moves
        .into_iter()
        .filter(|ui_move| from == ui_move.from)
        .collect()
}

fn get_selected_moves_to(
    possible_moves: Vec<ChessMove>,
    to: Square,
) -> Option<SimpleMoveOrPromotions> {
    let mut moves_from_to: Vec<ChessMove> = possible_moves
        .into_iter()
        .filter(|ui_move| to == ui_move.to)
        .collect();
    if moves_from_to.len() == 4 {
        Some(SimpleMoveOrPromotions::Promotions(moves_from_to))
    } else {
        moves_from_to.pop().map(SimpleMoveOrPromotions::SimpleMove)
    }
}

enum SimpleMoveOrPromotions {
    SimpleMove(ChessMove),
    Promotions(Vec<ChessMove>),
}
