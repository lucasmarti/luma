use crate::{
    chess_moves::ChessMove,
    directions::{self, DirectionFn},
    piece::{Color, Piece, BLACK_QUEEN, WHITE_QUEEN},
    position::Position,
    possible_moves::common::get_sliding_moves,
};

pub fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::White)
}

pub fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::Black)
}

const QUEEN_DIRECTIONS: [DirectionFn; 8] = [
    directions::up,
    directions::down,
    directions::left,
    directions::right,
    directions::up_left,
    directions::up_right,
    directions::down_left,
    directions::down_right,
];

fn get_possible_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let queen = get_queen(color);
    get_sliding_moves(position, from, color, &QUEEN_DIRECTIONS, queen)
}

fn get_queen(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_QUEEN,
        Color::White => WHITE_QUEEN,
    }
}
mod tests;
