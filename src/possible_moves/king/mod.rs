use crate::{
    chess_moves::ChessMove,
    directions::*,
    piece::{Color, Piece},
    position::Position,
    possible_moves::common::get_piece_moves,
};

pub const KING_DIRECTIONS: [DirectionFn; 8] = [
    left, right, up, down, up_left, up_right, down_left, down_right,
];

pub fn get_possible_moves(position: &Position, from: u32, piece: Piece) -> Vec<ChessMove> {
    let mut moves = get_piece_moves(position, from, &KING_DIRECTIONS, piece, 1);
    moves.extend(get_castle_moves(position, from, piece.color));
    moves
}

fn get_castle_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let mut moves = Vec::new();

    // Only check castling if king is on starting square
    let king_start_square = match color {
        Color::White => E1,
        Color::Black => E8,
    };

    if from != king_start_square {
        return moves;
    }

    // Check kingside castling
    todo!();

    // Check queenside castling
    todo!();

    moves
}

mod tests;
