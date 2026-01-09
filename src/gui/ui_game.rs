use flo_canvas::{Draw, GraphicsContext};

use crate::{
    engine::{
        self, chess_moves::ChessMove, directions::squares::Square, piece::Piece, position::Position,
    },
    gui::{
        configuration::{BACKGROUND_COLOR, FIELD_SIZE, MENU_HEIGHT},
        state_machine::{GameState, SquareSelected},
        ui_board::{Orientation, UIBoard},
        ui_container::Container,
        ui_element::{CanvasCoordinate, UIElement, UIEvent},
        ui_layout::GameLayout,
        ui_menu::UIMenu,
    },
};

pub struct UIGame {
    ui_menu: UIMenu,
    ui_board: UIBoard,
    container: Container,
}
impl UIGame {
    pub fn new() -> Self {
        let container = Container::new(
            0.0,
            0.0,
            8.0 * FIELD_SIZE + 3.0 * MENU_HEIGHT,
            8.0 * FIELD_SIZE,
        );
        let layout: GameLayout = GameLayout::new(container);
        UIGame {
            container,
            ui_menu: UIMenu::new(layout.get_menu()),
            ui_board: UIBoard::new(layout.get_board()),
        }
    }

    pub fn update(
        &mut self,
        position: &Position,
        last_move: &Option<ChessMove>,
        state: &GameState,
    ) {
        self.reset_squares();
        for (square, piece) in position.get_all_pieces() {
            self.set_piece(square, piece);
        }
        if let Some(square) = engine::get_check_square(position) {
            self.ui_board.set_check_square(square);
        }
        if let Some(chess_move) = last_move {
            self.set_last_move_square(chess_move.from);
            self.set_last_move_square(chess_move.to);
        }
        if let GameState::Player(square_selected) = state {
            match square_selected {
                SquareSelected::From(data) => {
                    for chess_move in data.possible_moves_from.iter() {
                        self.set_drop_target_square(chess_move.to);
                    }
                    self.set_selected_square(data.from);
                }
                SquareSelected::Promotion(data) => {
                    if let Some(chess_move) = data.possible_promotion_moves.first() {
                        self.set_selected_square(chess_move.from);
                        self.set_drop_target_square(chess_move.to);
                        match position.get_player() {
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
            }
        }
    }

    pub fn reset_squares(&mut self) {
        self.ui_board.reset_squares();
    }

    pub fn turn_board(&mut self) {
        self.ui_board.turn_board();
    }
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.ui_board.set_orientation(orientation);
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
    pub fn disabled_promotion_buttons(&mut self) {
        self.set_black_promotion_buttons_disabled(true);
        self.set_white_promotion_buttons_disabled(true);
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
        gc.canvas_height(self.container.get_height());
        gc.center_region(
            self.container.x_horizontal_min,
            self.container.y_vertical_min,
            self.container.x_horizontal_max,
            self.container.y_vertical_max,
        );
        self.ui_menu.draw(gc);
        self.ui_board.draw(gc);
    }
}
