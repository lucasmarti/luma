use crate::{
    check::{is_check, is_under_attack},
    chess_moves::*,
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

fn get_castle_moves(position: &Position, color: Color) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    match color {
        Color::Black => {
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
        }
        Color::White => {
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
        }
    }
    positions
}

mod tests;
