use crate::engine::{
    chess_moves::{get_black_moves, get_white_moves},
    piece::{self, Piece},
    position::{self, Position},
};

const QUEEN_FACTOR: f32 = 4.5;
const KING_FACTOR: f32 = 1.5;
const ROOK_FACTOR: f32 = 2.5;
const KNIGHT_FACTOR: f32 = 3.5;
const BISHOP_FACTOR: f32 = 3.5;

pub fn count_black(position: &Position) -> f32 {
    0.0
}
