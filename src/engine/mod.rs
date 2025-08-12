use std::collections::HashMap;

use crate::engine::{
    chess_moves::configurations::{MovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG},
    position::Position,
};

pub fn is_valid_drag_square(position: &Position, square: u32) -> bool {
    match position.get_piece_at(square) {
        Some(piece) => piece.color == position.get_player(),
        None => false,
    }
}

pub fn get_valid_drop_positions(position: &Position, from: u32) -> HashMap<u32, Position> {
    let mut positions: HashMap<u32, Position> = HashMap::new();
    if let Some(piece) = position.get_piece_at(from) {
        let config = match piece.color {
            piece::Color::Black => BLACK_MOVE_CONFIG,
            piece::Color::White => WHITE_MOVE_CONFIG,
        };
        let moves_fn: MovesFn = match piece.typ {
            piece::Typ::King => config.king_fn,
            piece::Typ::Queen => config.queen_fn,
            piece::Typ::Rook => config.rook_fn,
            piece::Typ::Pawn => config.pawn_fn,
            piece::Typ::Knight => config.knight_fn,
            piece::Typ::Bishop => config.bishop_fn,
        };
        for position in moves_fn(position, piece, from) {
            if let Some(target) = position.get_target() {
                positions.insert(target, position);
            }
        }
    }
    positions
}
pub fn get_next_positions(position: &Position) -> Vec<Position> {
    let positions: Vec<Position> = Vec::new();
    positions
}

pub fn new_game() -> Position {
    Position::new_starting_position()
}

mod check;
mod chess_moves;
mod directions;
mod evaluation;
mod minimax;
pub mod piece;
pub mod position;
#[cfg(test)]
mod tests;
