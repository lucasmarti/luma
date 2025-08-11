use crate::{chess_moves::*, position::Position};

pub fn disallow_castle_if_necessary(position: Position, from: u32) -> Position {
    let mut new_position = position;
    if from == WHITE_QUEENSIDE.king_from || from == WHITE_QUEENSIDE.rook_from {
        new_position = new_position.disallow_white_queenside_castle();
    }
    if from == WHITE_KINGSIDE.king_from || from == WHITE_KINGSIDE.rook_from {
        new_position = new_position.disallow_white_kingside_castle();
    }
    if from == BLACK_QUEENSIDE.king_from || from == BLACK_QUEENSIDE.rook_from {
        new_position = new_position.disallow_black_queenside_castle();
    }
    if from == BLACK_KINGSIDE.king_from || from == BLACK_KINGSIDE.rook_from {
        new_position = new_position.disallow_black_kingside_castle();
    }
    new_position
}

pub fn get_black_castle_moves(position: &Position) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    if position.is_black_queenside_castle_allowed() {
        if let Some(position) = black_queenside_castle(position) {
            positions.push(position);
        }
    }
    if position.is_black_kingside_castle_allowed() {
        if let Some(position) = black_kingside_castle(position) {
            positions.push(position);
        }
    }
    positions
}

pub fn get_white_castle_moves(position: &Position) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    if position.is_white_queenside_castle_allowed() {
        if let Some(position) = white_queenside_castle(position) {
            positions.push(position);
        }
    }
    if position.is_white_kingside_castle_allowed() {
        if let Some(position) = white_kingside_castle(position) {
            positions.push(position);
        }
    }

    positions
}

mod tests;
