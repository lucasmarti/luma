use crate::{
    check::{is_check, is_under_attack},
    directions::{self, *},
    piece::*,
    position::Position,
    possible_moves::king::disallow_castle_if_necessary,
};
pub fn progess(position: &Position, piece: Piece, from: u32, to: u32) -> Position {
    let mut new_position = position
        .remove_piece(from)
        .remove_piece(to)
        .put_piece(piece, to)
        .toggle_player();
    new_position = set_en_passant_if_necessary(new_position, piece, from, to);
    new_position = disallow_castle_if_necessary(new_position, from);
    new_position
}

fn set_en_passant_if_necessary(position: Position, piece: Piece, from: u32, to: u32) -> Position {
    if is_pawn_two_rows_forward(piece, from, to) {
        return position.set_en_passant(to);
    }
    position
}

fn is_pawn_two_rows_forward(piece: Piece, from: u32, to: u32) -> bool {
    if !(piece.typ == Typ::Pawn) {
        return false;
    }
    match piece.color {
        Color::White => {
            if directions::is_in_row_2(from) && directions::is_in_row_4(to) {
                return true;
            }
        }
        Color::Black => {
            if directions::is_in_row_7(from) && directions::is_in_row_5(to) {
                return true;
            }
        }
    }
    return false;
}

pub fn en_passant(position: &Position, piece: Piece, from: u32, to: u32, capture: u32) -> Position {
    position
        .remove_piece(from)
        .remove_piece(capture)
        .remove_piece(to)
        .put_piece(piece, to)
        .toggle_player()
        .reset_en_passant()
}

pub fn promote(position: &Position, from: u32, to: u32, new_piece: Piece) -> Position {
    position
        .remove_piece(from)
        .remove_piece(to)
        .put_piece(new_piece, to)
        .toggle_player()
        .reset_en_passant()
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

    let mut new_position = position
        .remove_piece(castle.king_from)
        .remove_piece(castle.rook_from)
        .put_piece(castle.king, castle.king_to)
        .put_piece(castle.rook, castle.rook_to)
        .toggle_player()
        .reset_en_passant();
    match castle.color {
        Color::White => {
            new_position = new_position
                .disallow_white_kingside_castle()
                .disallow_white_queenside_castle();
        }
        Color::Black => {
            new_position = new_position
                .disallow_black_kingside_castle()
                .disallow_black_queenside_castle();
        }
    }
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
