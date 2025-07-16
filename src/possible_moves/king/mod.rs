use crate::{
    check::{is_check, is_under_attack},
    directions::*,
    piece::{Color, Piece, BLACK_KING, BLACK_ROOK, WHITE_KING, WHITE_ROOK},
    position::Position,
    possible_moves::common::get_piece_moves,
};

pub const KING_DIRECTIONS: [DirectionFn; 8] = [
    left, right, up, down, up_left, up_right, down_left, down_right,
];

pub fn get_possible_moves(position: &Position, from: u32, piece: Piece) -> Vec<Position> {
    let mut new_positions = get_piece_moves(position, from, &KING_DIRECTIONS, piece, 1);
    new_positions.extend(get_castle_moves(position, piece.color));
    new_positions
}

struct Castle {
    color: Color,
    king: Piece,
    rook: Piece,
    king_from: u32,
    king_to: u32,
    rook_from: u32,
    rook_to: u32,
    empty_path_squares: &'static [u32],
}

// White Kingside Castling
const WHITE_KINGSIDE: Castle = Castle {
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
const WHITE_QUEENSIDE: Castle = Castle {
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
const BLACK_KINGSIDE: Castle = Castle {
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
const BLACK_QUEENSIDE: Castle = Castle {
    color: Color::Black,
    king_from: E8,
    king_to: C8,
    rook_from: A8,
    rook_to: D8,
    empty_path_squares: &[B8, C8, D8],
    king: BLACK_KING,
    rook: BLACK_ROOK,
};
fn get_castle_moves(position: &Position, color: Color) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    match color {
        Color::Black => {
            if position.is_black_queenside_castle_allowed() {
                if let Some(position) = get_castle(position, &BLACK_QUEENSIDE) {
                    positions.push(position);
                }
            }
            if position.is_black_kingside_castle_allowed() {
                if let Some(position) = get_castle(position, &BLACK_KINGSIDE) {
                    positions.push(position);
                }
            }
        }
        Color::White => {
            if position.is_white_queenside_castle_allowed() {
                if let Some(position) = get_castle(position, &WHITE_QUEENSIDE) {
                    positions.push(position);
                }
            }
            if position.is_white_kingside_castle_allowed() {
                if let Some(position) = get_castle(position, &WHITE_KINGSIDE) {
                    positions.push(position);
                }
            }
        }
    }
    positions
}

fn get_castle(position: &Position, castle: &Castle) -> Option<Position> {
    if !is_empty_path(position, castle.empty_path_squares) {
        return None;
    }

    if !is_save_passage(position, castle.empty_path_squares, castle.color) {
        return None;
    }
    println!(
        "Number of Black Kings {:?}",
        position.count_pieces(BLACK_KING)
    );
    println!("Color {:?},", castle.color);
    if is_check(position, castle.color) {
        return None;
    }

    let new_position = position
        .remove_piece(castle.king_from)
        .remove_piece(castle.rook_from)
        .put_piece(castle.king, castle.king_to)
        .put_piece(castle.rook, castle.rook_to);

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

mod tests;
