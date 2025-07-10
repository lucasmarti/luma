use crate::{
    chess_moves::ChessMove,
    directions::{self, DirectionFn},
    piece::{Color, Piece, BLACK_KNIGHT, WHITE_KNIGHT},
    position::Position,
    possible_moves::common::get_single_step_moves,
};

pub fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::Black)
}
pub fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::White)
}

const KNIGHT_DIRECTIONS: [DirectionFn; 8] = [
    directions::right_right_down,
    directions::right_right_up,
    directions::left_left_up,
    directions::left_left_down,
    directions::up_up_left,
    directions::up_up_right,
    directions::down_down_left,
    directions::down_down_right,
];

fn get_possible_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let knight = get_knight(color);
    get_single_step_moves(position, from, color, &KNIGHT_DIRECTIONS, knight)
}

fn get_knight(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_KNIGHT,
        Color::White => WHITE_KNIGHT,
    }
}
mod tests;
