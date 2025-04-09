use crate::position::Position;
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
pub fn evaluate(position: &Position) -> i32 {
    let white_score = KING_SCORE * position.black_king.count_ones()
        + QUEEN_SCORE * position.white_queen.count_ones()
        + ROOK_SCORE * position.white_rooks.count_ones()
        + BISHOP_SCORE * position.white_bishops.count_ones()
        + KNIGHT_SCORE * position.white_knights.count_ones()
        + PAWN_SCORE * position.white_pawns.count_ones();

    let black_score = KING_SCORE * position.black_king.count_ones()
        + QUEEN_SCORE * position.black_queen.count_ones()
        + ROOK_SCORE * position.black_rooks.count_ones()
        + BISHOP_SCORE * position.black_bishops.count_ones()
        + KNIGHT_SCORE * position.black_knights.count_ones()
        + PAWN_SCORE * position.black_pawns.count_ones();
    white_score as i32 - black_score as i32
}

#[cfg(test)]
mod tests;
