use crate::engine::{
    chess_moves::{get_black_mobility, get_white_mobility},
    piece::{self},
    position::Position,
};

const QUEEN_FACTOR: f32 = 4.5;
const KING_FACTOR: f32 = 1.5;
const ROOK_FACTOR: f32 = 2.5;
const KNIGHT_FACTOR: f32 = 3.5;
const BISHOP_FACTOR: f32 = 3.5;
const PAWN_FACTOR: f32 = 0.0;

pub fn count_black(position: &Position) -> f32 {
    get_score(get_black_mobility(position))
}
pub fn count_white(position: &Position) -> f32 {
    get_score(get_white_mobility(position))
}

fn get_score(positions: Vec<Position>) -> f32 {
    let mut score: f32 = 0.0;
    for position in positions {
        if let Some(chess_move) = position.get_last_move() {
            let factor = match chess_move.piece.get_type() {
                piece::Typ::King => KING_FACTOR,
                piece::Typ::Queen => QUEEN_FACTOR,
                piece::Typ::Rook => ROOK_FACTOR,
                piece::Typ::Pawn => PAWN_FACTOR,
                piece::Typ::Knight => KNIGHT_FACTOR,
                piece::Typ::Bishop => BISHOP_FACTOR,
            };
            score += factor;
        } else {
            panic!("Position without ChessMove in heuristic.");
        }
    }
    score
}
