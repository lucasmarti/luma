use crate::{
    directions::{self, DirectionFn},
    piece::Piece,
    position::Position,
    possible_moves::common::get_piece_moves,
};

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

pub fn get_possible_moves(position: &Position, from: u32, piece: Piece) -> Vec<Position> {
    get_piece_moves(position, from, &QUEEN_DIRECTIONS, piece, u32::MAX)
}
mod tests;
