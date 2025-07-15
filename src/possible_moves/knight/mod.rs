use crate::{
    chess_moves::ChessMove,
    directions::{self, DirectionFn},
    piece::Piece,
    position::Position,
    possible_moves::common::get_piece_moves,
};

pub const KNIGHT_DIRECTIONS: [DirectionFn; 8] = [
    directions::right_right_down,
    directions::right_right_up,
    directions::left_left_up,
    directions::left_left_down,
    directions::up_up_left,
    directions::up_up_right,
    directions::down_down_left,
    directions::down_down_right,
];

pub fn get_possible_moves(position: &Position, from: u32, piece: Piece) -> Vec<ChessMove> {
    get_piece_moves(position, from, &KNIGHT_DIRECTIONS, piece, 1)
}
mod tests;
