use flo_canvas::{Draw, GraphicsContext};

use crate::{
    engine::{
        self,
        directions::squares::Square,
        piece::Piece,
        position::{self, Position},
    },
    gui::{
        configuration::{BACKGROUND_COLOR, FIELD_SIZE, MENU_HEIGHT},
        state_machine::GameState,
        ui_board::UIBoard,
        ui_element::{CanvasCoordinate, UIElement, UIEvent},
        ui_menu::UIMenu,
    },
};

pub struct UIGame {
    ui_menu: UIMenu,
    ui_board: UIBoard,
}
impl UIGame {
    pub fn new() -> Self {
        UIGame {
            ui_menu: UIMenu::new(),
            ui_board: UIBoard::new(),
        }
    }

    pub fn update(&mut self, position: &Position, state: &GameState) {
        self.reset_squares();
        for (square, piece) in position.get_all_pieces() {
            self.set_piece(square, piece);
        }
        match state {
            GameState::Player(player_uistate) => match player_uistate {
                super::state_machine::PlayerUIState::FromSquareSelected(data) => {
                    for chess_move in data.possible_moves.iter() {
                        self.set_drop_target_square(chess_move.to.clone());
                    }
                    self.set_selected_square(data.square);
                }
                super::state_machine::PlayerUIState::PromotionSquareSelected(data) => {
                    if let Some(chess_move) = data.promotion_moves.first() {
                        self.set_selected_square(chess_move.from);
                        self.set_drop_target_square(chess_move.to);
                        match chess_move.piece.get_color() {
                            crate::engine::piece::Color::Black => {
                                self.set_black_promotion_buttons_disabled(false)
                            }
                            crate::engine::piece::Color::White => {
                                self.set_white_promotion_buttons_disabled(false)
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        //        if let Some(square) = engine::get_check_square(position){
        //            self.ui_board.
        //        }
    }

    pub fn reset_squares(&mut self) {
        self.ui_board.reset_squares();
    }

    pub fn turn_board(&mut self) {
        self.ui_board.turn_board();
    }
    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.ui_board.set_piece(square, piece);
    }

    pub fn set_selected_square(&mut self, square: Square) {
        self.ui_board.set_selected_square(square);
    }
    pub fn set_drop_target_square(&mut self, square: Square) {
        self.ui_board.set_drop_target_square(square);
    }
    pub fn set_last_move_square(&mut self, square: Square) {
        self.ui_board.set_last_move_square(square);
    }
    pub fn set_black_promotion_buttons_disabled(&mut self, disabled: bool) {
        self.ui_menu.set_black_promotion_buttons_disabled(disabled);
    }
    pub fn set_white_promotion_buttons_disabled(&mut self, disabled: bool) {
        self.ui_menu.set_white_promotion_buttons_disabled(disabled);
    }
}
impl UIElement for UIGame {
    fn dispatch_event(&self, canvas_coordinate: CanvasCoordinate) -> Option<UIEvent> {
        if let Some(event) = self.ui_menu.dispatch_event(canvas_coordinate) {
            return Some(event);
        }
        if let Some(event) = self.ui_board.dispatch_event(canvas_coordinate) {
            return Some(event);
        }
        None
    }

    fn draw(&self, gc: &mut Vec<Draw>) {
        gc.clear_canvas(BACKGROUND_COLOR);
        gc.canvas_height(8.0 * FIELD_SIZE + 2.0 * MENU_HEIGHT);
        gc.center_region(
            0.0,
            0.0,
            8.0 * FIELD_SIZE,
            8.0 * FIELD_SIZE + 2.0 * MENU_HEIGHT,
        );
        self.ui_menu.draw(gc);
        self.ui_board.draw(gc);
    }
}
