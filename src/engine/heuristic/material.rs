use crate::engine::{piece::*, position::Position};

const KING_SCORE: u32 = 900;
const QUEEN_SCORE: u32 = 90;
const ROOK_SCORE: u32 = 50;
const BISHOP_SCORE: u32 = 30;
const KNIGHT_SCORE: u32 = 30;
const PAWN_SCORE: u32 = 10;

pub fn count_black(position: &Position) -> f32 {
    let black_score = KING_SCORE * position.count_pieces(Piece::BlackKing)
        + QUEEN_SCORE * position.count_pieces(Piece::BlackQueen)
        + ROOK_SCORE * position.count_pieces(Piece::BlackRook)
        + BISHOP_SCORE * position.count_pieces(Piece::BlackBishop)
        + KNIGHT_SCORE * position.count_pieces(Piece::BlackKnight)
        + PAWN_SCORE * position.count_pieces(Piece::BlackPawn);
    black_score as f32
}

pub fn count_white(position: &Position) -> f32 {
    let white_score = KING_SCORE * position.count_pieces(Piece::WhiteKing)
        + QUEEN_SCORE * position.count_pieces(Piece::WhiteQueen)
        + ROOK_SCORE * position.count_pieces(Piece::WhiteRook)
        + BISHOP_SCORE * position.count_pieces(Piece::WhiteBishop)
        + KNIGHT_SCORE * position.count_pieces(Piece::WhiteKnight)
        + PAWN_SCORE * position.count_pieces(Piece::WhitePawn);
    white_score as f32
}
