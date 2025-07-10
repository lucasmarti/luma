use crate::{
    chess_moves::ChessMove,
    directions::{self, DirectionFn},
    piece::{Color, Piece, BLACK_ROOK, WHITE_ROOK},
    position::Position,
    possible_moves::common::get_sliding_moves,
};

pub fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::White)
}

pub fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::Black)
}
const ROOK_DIRECTIONS: [DirectionFn; 4] = [
    directions::up,
    directions::down,
    directions::left,
    directions::right,
];

fn get_possible_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let rook = get_rook(color);
    get_sliding_moves(position, from, color, &ROOK_DIRECTIONS, rook)
}

fn get_rook(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_ROOK,
        Color::White => WHITE_ROOK,
    }
}

mod tests;
