use crate::{
    chess_moves::ChessMove,
    directions::{self, DirectionFn},
    piece::Piece,
    position::Position,
    possible_moves::common::get_piece_moves,
};

const ROOK_DIRECTIONS: [DirectionFn; 4] = [
    directions::up,
    directions::down,
    directions::left,
    directions::right,
];

pub fn get_possible_moves(position: &Position, from: u32, piece: Piece) -> Vec<ChessMove> {
    get_piece_moves(position, from, &ROOK_DIRECTIONS, piece, u32::MAX)
}

mod tests;
