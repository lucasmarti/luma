use crate::engine::{piece::*, position::Position};

const KING_SCORE: u32 = 900;
const QUEEN_SCORE: u32 = 90;
const ROOK_SCORE: u32 = 50;
const BISHOP_SCORE: u32 = 30;
const KNIGHT_SCORE: u32 = 30;
const PAWN_SCORE: u32 = 10;

pub fn count_black(position: &Position) -> f32 {
    count(position, Color::Black) as f32
}

pub fn count_white(position: &Position) -> f32 {
    count(position, Color::White) as f32
}

fn count(position: &Position, color: Color) -> f32 {
    let score = KING_SCORE * position.count_pieces(color, Typ::King)
        + QUEEN_SCORE * position.count_pieces(color, Typ::Queen)
        + ROOK_SCORE * position.count_pieces(color, Typ::Rook)
        + BISHOP_SCORE * position.count_pieces(color, Typ::Bishop)
        + KNIGHT_SCORE * position.count_pieces(color, Typ::Knight)
        + PAWN_SCORE * position.count_pieces(color, Typ::Pawn);
    score as f32
}
