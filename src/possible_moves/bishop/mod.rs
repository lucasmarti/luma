use crate::{
    chess_moves::ChessMove,
    directions::{self, DirectionFn},
    piece::{Color, Piece, BLACK_BISHOP, WHITE_BISHOP},
    position::Position,
    possible_moves::common::get_sliding_moves,
};

pub fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::White)
}

pub fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::Black)
}

const BISHOP_DIRECTIONS: [DirectionFn; 4] = [
    directions::up_left,
    directions::up_right,
    directions::down_left,
    directions::down_right,
];

fn get_possible_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let bishop = get_bishop(color);
    get_sliding_moves(position, from, color, &BISHOP_DIRECTIONS, bishop)
}

fn get_bishop(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_BISHOP,
        Color::White => WHITE_BISHOP,
    }
}
mod tests;
