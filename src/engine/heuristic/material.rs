use crate::engine::{piece::*, position::Position};

const KING_SCORE: u32 = 900;
const QUEEN_SCORE: u32 = 90;
const ROOK_SCORE: u32 = 50;
const BISHOP_SCORE: u32 = 30;
const KNIGHT_SCORE: u32 = 30;
const PAWN_SCORE: u32 = 10;

pub fn count_black(position: &Position) -> f32 {
    let black_score = KING_SCORE * position.count_pieces(BLACK_KING)
        + QUEEN_SCORE * position.count_pieces(BLACK_QUEEN)
        + ROOK_SCORE * position.count_pieces(BLACK_ROOK)
        + BISHOP_SCORE * position.count_pieces(BLACK_BISHOP)
        + KNIGHT_SCORE * position.count_pieces(BLACK_KNIGHT)
        + PAWN_SCORE * position.count_pieces(BLACK_PAWN);
    black_score as f32
}

pub fn count_white(position: &Position) -> f32 {
    let white_score = KING_SCORE * position.count_pieces(WHITE_KING)
        + QUEEN_SCORE * position.count_pieces(WHITE_QUEEN)
        + ROOK_SCORE * position.count_pieces(WHITE_ROOK)
        + BISHOP_SCORE * position.count_pieces(WHITE_BISHOP)
        + KNIGHT_SCORE * position.count_pieces(WHITE_KNIGHT)
        + PAWN_SCORE * position.count_pieces(WHITE_PAWN);
    white_score as f32
}
