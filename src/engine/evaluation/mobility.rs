use crate::engine::{
    chess_moves::{get_black_mobility, get_white_mobility, ChessMove},
    piece::{self},
    position::Position,
};

const QUEEN_FACTOR: f32 = 0.25; // ~2.25 Punkte pro Feld
const ROOK_FACTOR: f32 = 0.20; // ~1.0 Punkte pro Feld
const BISHOP_FACTOR: f32 = 0.10; // ~0.3 Punkte pro Feld
const KNIGHT_FACTOR: f32 = 0.15; // ~0.45 Punkte pro Feld
const KING_FACTOR: f32 = 0.05; // König-Aktivität ist später wichtig
const PAWN_FACTOR: f32 = 0.0;

pub fn count_black(position: &Position) -> f32 {
    get_score(get_black_mobility(position))
}
pub fn count_white(position: &Position) -> f32 {
    get_score(get_white_mobility(position))
}

fn get_score(chess_moves: Vec<ChessMove>) -> f32 {
    let mut score: f32 = 0.0;
    for chess_move in chess_moves {
        let factor = match chess_move.piece.get_type() {
            piece::Typ::King => KING_FACTOR,
            piece::Typ::Queen => QUEEN_FACTOR,
            piece::Typ::Rook => ROOK_FACTOR,
            piece::Typ::Pawn => PAWN_FACTOR,
            piece::Typ::Knight => KNIGHT_FACTOR,
            piece::Typ::Bishop => BISHOP_FACTOR,
        };
        score += factor;
    }
    score
}
