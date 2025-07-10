use crate::{
    chess_moves::ChessMove,
    directions::{self, DirectionFn},
    piece::{Color, Piece, BLACK_KING, WHITE_KING},
    position::Position,
    possible_moves::common::get_single_step_moves,
};

const KING_DIRECTIONS: [DirectionFn; 8] = [
    directions::left,
    directions::right,
    directions::up,
    directions::down,
    directions::up_left,
    directions::up_right,
    directions::down_left,
    directions::down_right,
];

pub fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::Black)
}
pub fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::White)
}

fn get_possible_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let king = get_king(color);
    get_single_step_moves(position, from, color, &KING_DIRECTIONS, king)
}

fn get_king(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_KING,
        Color::White => WHITE_KING,
    }
}
mod tests;
