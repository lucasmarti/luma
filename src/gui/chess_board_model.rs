use std::collections::HashMap;

use crate::engine::{
    self,
    piece::Color,
    position::{self, Position},
};

#[derive(Clone)]
pub struct ChessBoardModel {
    pub position: Position,
    pub drop_targets: HashMap<u32, Position>,
    pub selected_square: Option<u32>,
    pub computer: Color,
}
impl ChessBoardModel {
    pub fn new() -> ChessBoardModel {
        ChessBoardModel {
            position: Position::new_starting_position(),
            drop_targets: HashMap::new(),
            selected_square: None,
            computer: Color::Black,
        }
    }

    pub fn handle_click_event(&mut self, square: u32) {
        if self.position.get_player() == self.computer {
            return;
        }
        if engine::is_valid_drag_square(&self.position, square) {
            self.selected_square = Some(square);
            self.drop_targets = engine::get_valid_drop_positions(&self.position, square);
        } else if let Some(new_position) = self.drop_targets.get(&square) {
            self.position = *new_position;

            if let Some(position) = engine::get_next_move(&new_position) {
                self.position = position;
            }
            self.drop_targets.clear();
            self.selected_square = None;
        } else {
            self.drop_targets.clear();
            self.selected_square = None;
        }
    }
}
