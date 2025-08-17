use crate::engine::{piece::*, position::Position};
const KING_SCORE: u32 = 200;
const QUEEN_SCORE: u32 = 9;
const ROOK_SCORE: u32 = 5;
const BISHOP_SCORE: u32 = 3;
const KNIGHT_SCORE: u32 = 3;
const PAWN_SCORE: u32 = 1;

/*
f(p) = 200(K-K')
       + 9(Q-Q')
       + 5(R-R')
       + 3(B-B' + N-N')
       + 1(P-P')
*/
pub fn heuristic(position: &Position) -> i32 {
    let white_score = KING_SCORE * position.count_pieces(WHITE_KING)
        + QUEEN_SCORE * position.count_pieces(WHITE_QUEEN)
        + ROOK_SCORE * position.count_pieces(WHITE_ROOK)
        + BISHOP_SCORE * position.count_pieces(WHITE_BISHOP)
        + KNIGHT_SCORE * position.count_pieces(WHITE_KNIGHT)
        + PAWN_SCORE * position.count_pieces(WHITE_PAWN);

    let black_score = KING_SCORE * position.count_pieces(BLACK_KING)
        + QUEEN_SCORE * position.count_pieces(BLACK_QUEEN)
        + ROOK_SCORE * position.count_pieces(BLACK_ROOK)
        + BISHOP_SCORE * position.count_pieces(BLACK_BISHOP)
        + KNIGHT_SCORE * position.count_pieces(BLACK_KNIGHT)
        + PAWN_SCORE * position.count_pieces(BLACK_PAWN);
    white_score as i32 - black_score as i32
}

#[cfg(test)]
mod tests;
