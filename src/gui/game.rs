use flo_canvas::{Draw, DrawingTarget};

use crate::{
    engine::{
        self,
        chess_moves::{self, ChessMove, MoveType},
        directions::squares::Square,
        position::Position,
    },
    gui::{
        state_machine::{
            self, FromSquareSelectedData, GameState, PlayerUIState, PromoteData,
            PromotionSquareSelectedData, SelectToSquareData, StateFunction,
        },
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
            state: GameState::Player(PlayerUIState::NoSquareSelected),
            position: Position::new_starting_position(),
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
                    StateFunction::SelectFromSquare(square) => self.select_from_square(square),
                    StateFunction::SelectToSquare(data) => self.select_to_square(data),
                    StateFunction::Promote(data) => self.promote(data),
                }
            }
        }
    }

    fn new_game_as(&mut self, color: engine::piece::Color) {
        self.position = Position::new_starting_position();
        self.update_ui();

        match color {
            engine::piece::Color::Black => {
                self.state = GameState::Computer;
                self.execute_computer_move();
            }
            engine::piece::Color::White => {
                self.state = GameState::Player(PlayerUIState::NoSquareSelected);
            }
        }
    }
    fn select_from_square(&mut self, square: Square) {
        if engine::is_valid_drag_square(&self.position, square) {
            self.state =
                GameState::Player(PlayerUIState::FromSquareSelected(FromSquareSelectedData {
                    square: square,
                    possible_moves: engine::get_possible_moves(&self.position, square),
                }));
        } else {
            self.state = GameState::Player(PlayerUIState::NoSquareSelected);
        }
        self.update_ui();
    }

    fn select_to_square(&mut self, data: SelectToSquareData) {
        match data.get_chess_move_for_to_square() {
            Some(chess_move) => match chess_move.move_type {
                MoveType::Promotion | MoveType::PromotionCapture => {
                    self.enable_promotion_buttons(data);
                }
                _ => {
                    self.execute_player_move(chess_move);
                }
            },
            None => self.state = GameState::Player(PlayerUIState::NoSquareSelected),
        }
        self.draw();
    }

    fn promote(&mut self, data: PromoteData) {
        match data.get_chess_move_for_selected_promotion_piece() {
            Some(chess_move) => self.execute_player_move(chess_move),
            None => panic!("Irgendwas ist nicht sauber implementiert"),
        }
    }

    fn enable_promotion_buttons(&mut self, data: SelectToSquareData) {
        self.state = GameState::Player(PlayerUIState::PromotionSquareSelected(
            PromotionSquareSelectedData::from(data),
        ));
        self.update_ui();
    }

    fn execute_computer_move(&mut self) {
        match engine::get_next_move(&self.position) {
            Some(conmputer_position) => {
                self.position = conmputer_position;
                self.state = GameState::Player(PlayerUIState::NoSquareSelected);
            }
            None => self.state = GameState::GameOver,
        }
        self.update_ui();
    }

    fn execute_player_move(&mut self, chess_move: ChessMove) {
        if let Some(position) = engine::execute_move(&self.position, chess_move) {
            self.position = position;
        }
        self.update_ui();
        self.state = GameState::Computer;
        self.execute_computer_move();
    }
}
