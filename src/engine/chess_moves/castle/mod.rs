use crate::engine::{
    check::{is_check, is_under_attack},
    directions::squares::*,
    piece::{Color, Piece, BLACK_KING, BLACK_ROOK, WHITE_KING, WHITE_ROOK},
    position::Position,
};

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

pub fn white_queenside_castle(position: &Position) -> Option<Position> {
    castle(position, WHITE_QUEENSIDE)
}

pub fn white_kingside_castle(position: &Position) -> Option<Position> {
    castle(position, WHITE_KINGSIDE)
}

pub fn black_queenside_castle(position: &Position) -> Option<Position> {
    castle(position, BLACK_QUEENSIDE)
}
pub fn black_kingside_castle(position: &Position) -> Option<Position> {
    castle(position, BLACK_KINGSIDE)
}

fn castle(position: &Position, castle: Castle) -> Option<Position> {
    if !is_empty_path(position, castle.empty_path_squares) {
        return None;
    }
    if !(position.is_occupied_by_piece(castle.rook_from, castle.rook)
        && position.is_occupied_by_piece(castle.king_from, castle.king))
    {
        return None;
    }

    if !is_save_passage(position, castle.empty_path_squares, castle.color) {
        return None;
    }

    if is_check(position, castle.color) {
        return None;
    }

    let new_position = position
        .remove_piece(castle.king_from)
        .remove_piece(castle.rook_from)
        .put_piece(castle.king, castle.king_to)
        .put_piece(castle.rook, castle.rook_to)
        .toggle_player()
        .reset_en_passant()
        .disallow_castle_for_color(castle.color)
        .set_target(castle.king_to);
    if !is_check(&new_position, castle.color) {
        return Some(new_position);
    }
    None
}

fn is_save_passage(position: &Position, sqares: &[u32], color: Color) -> bool {
    for square in sqares {
        if is_under_attack(position, *square, color) {
            return false;
        }
    }
    return true;
}

fn is_empty_path(position: &Position, sqares: &[u32]) -> bool {
    for square in sqares {
        if position.is_occupied(*square) {
            return false;
        }
    }
    true
}

pub const WHITE_KING_STARTING_POSITION: u32 = E1;
pub const BLACK_KING_STARTING_POSITION: u32 = E8;
pub struct Castle {
    color: Color,
    king: Piece,
    rook: Piece,
    pub(crate) king_from: u32,
    king_to: u32,
    pub(crate) rook_from: u32,
    rook_to: u32,
    empty_path_squares: &'static [u32],
}

// White Kingside Castling
pub const WHITE_KINGSIDE: Castle = Castle {
    color: Color::White,
    king_from: E1,
    king_to: G1,
    rook_from: H1,
    rook_to: F1,
    empty_path_squares: &[F1, G1],
    king: WHITE_KING,
    rook: WHITE_ROOK,
};

// White Queenside Castling
pub const WHITE_QUEENSIDE: Castle = Castle {
    color: Color::White,
    king_from: E1,
    king_to: C1,
    rook_from: A1,
    rook_to: D1,
    empty_path_squares: &[B1, C1, D1],
    king: WHITE_KING,
    rook: WHITE_ROOK,
};

// Black Kingside Castling
pub const BLACK_KINGSIDE: Castle = Castle {
    color: Color::Black,
    king_from: E8,
    king_to: G8,
    rook_from: H8,
    rook_to: F8,
    empty_path_squares: &[F8, G8],
    king: BLACK_KING,
    rook: BLACK_ROOK,
};

// Black Queenside Castling
pub const BLACK_QUEENSIDE: Castle = Castle {
    color: Color::Black,
    king_from: E8,
    king_to: C8,
    rook_from: A8,
    rook_to: D8,
    empty_path_squares: &[B8, C8, D8],
    king: BLACK_KING,
    rook: BLACK_ROOK,
};
#[cfg(test)]
mod tests;
