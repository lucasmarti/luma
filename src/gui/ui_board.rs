use std::collections::HashMap;

use flo_canvas::Draw;

use crate::{
    engine::{directions::squares::Square, piece::Piece},
    gui::{
        configuration::{BLACK_FIELD_COLOR, FIELD_SIZE, WHITE_FIELD_COLOR},
        ui_element::{CanvasCoordinate, UIElement, UIEvent},
        ui_layout::{BoardLayout, Container},
        ui_piece::get_icon,
        ui_square::UISquare,
    },
};
#[derive(Debug)]
pub enum Orientation {
    WhiteUp,
    WhiteDown,
}
pub struct UIBoard {
    squares: HashMap<u32, UISquare>,
    orientation: Orientation,
    layout: BoardLayout,
}
impl UIBoard {
    pub fn reset_squares(&mut self) {
        for square in self.squares.values_mut() {
            square.reset();
        }
    }
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
        self.layout_squares();
    }
    pub fn turn_board(&mut self) {
        match self.orientation {
            Orientation::WhiteDown => self.orientation = Orientation::WhiteUp,
            Orientation::WhiteUp => self.orientation = Orientation::WhiteDown,
        }
        self.layout_squares();
    }
    pub fn set_selected_square(&mut self, square: Square) {
        if let Some(ui_square) = self.squares.get_mut(&square.as_index()) {
            ui_square.set_selected(true);
        }
    }
    pub fn set_drop_target_square(&mut self, square: Square) {
        if let Some(ui_square) = self.squares.get_mut(&square.as_index()) {
            ui_square.set_drop_target(true);
        }
    }
    pub fn set_last_move_square(&mut self, square: Square) {
        if let Some(ui_square) = self.squares.get_mut(&square.as_index()) {
            ui_square.set_last_move(true);
        }
    }

    fn layout_squares(&mut self) {
        for x in 0..8 {
            for y in 0..8 {
                let index = match self.orientation {
                    Orientation::WhiteUp => x + 8 * y,
                    Orientation::WhiteDown => 63 - (x + 8 * y),
                };
                if let Some(square) = self.squares.get_mut(&index) {
                    square.set_container(self.layout.get(y + 1, x + 1));
                }
            }
        }
    }
    pub fn new(container: Container) -> Self {
        let mut board = UIBoard {
            squares: HashMap::new(),
            orientation: Orientation::WhiteUp,
            layout: BoardLayout::new(container),
        };
        for x in 0..8 {
            for y in 0..8 {
                let color = if (x + y) % 2 == 1 {
                    WHITE_FIELD_COLOR
                } else {
                    BLACK_FIELD_COLOR
                };
                let index = x * 8 + y;
                board
                    .squares
                    .insert(index, UISquare::new(Square::new_unchecked(index), color));
            }
        }
        board.layout_squares();
        board
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        if let Some(square) = self.squares.get_mut(&square.as_index()) {
            square.set_icon(get_icon(piece));
        }
    }
}
impl UIElement for UIBoard {
    fn draw(&self, gc: &mut Vec<Draw>) {
        for ui_square in self.squares.values().cloned() {
            ui_square.draw(gc);
        }
    }
    fn dispatch_event(&self, canvas_coordinate: CanvasCoordinate) -> Option<UIEvent> {
        for square in self.squares.values() {
            if let Some(event) = square.dispatch_event(canvas_coordinate) {
                return Some(event);
            }
        }
        None
    }
}
