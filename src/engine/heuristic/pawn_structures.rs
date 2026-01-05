use crate::engine::{
    directions::{self},
    piece::Piece,
    position::Position,
};
const PAWN_IN_FRONT_SCORE: f32 = -15.0;
const ISOLATED_PAWN_SCORE: f32 = -10.0;
const BACKWARD_SCORE: f32 = -12.0;
const TBD: f32 = 0.0;

pub fn count_black(position: &Position) -> f32 {
    let mut score: f32 = 0.0;
    score += get_doubled_pawns(position, Piece::BlackPawn);
    score += get_isolated_pawns(position, Piece::BlackPawn);
    score
}
pub fn count_white(position: &Position) -> f32 {
    let mut score: f32 = 0.0;
    score += get_doubled_pawns(position, Piece::WhitePawn);
    score += get_isolated_pawns(position, Piece::WhitePawn);
    score
}

pub fn get_doubled_pawns(position: &Position, piece: Piece) -> f32 {
    let mut score = 0.0;
    for mut square in position.get_squares(piece).iter() {
        let mut has_pawn_in_front = false;

        while let Some(front_square) = directions::up(square) {
            if position.is_occupied_by_piece(front_square, piece) {
                has_pawn_in_front = true;
                break;
            }
            square = front_square;
        }
        if has_pawn_in_front {
            score += PAWN_IN_FRONT_SCORE;
        }
    }
    score
}
pub fn get_isolated_pawns(position: &Position, pawn: Piece) -> f32 {
    let mut score: f32 = 0.0;
    let mut columns: u32 = 0;
    for square in position.get_squares(pawn).iter() {
        columns |= 1 << (directions::get_column(square))
    }

    for col in 1..9 {
        let left_column_has_pawn = (columns & (1 << (col - 1))) != 0;
        let center_column_has_pawn = (columns & (1 << col)) != 0;
        let right_column_has_pawn = (columns & (1 << (col + 1))) != 0;
        if !left_column_has_pawn && center_column_has_pawn && !right_column_has_pawn {
            score += ISOLATED_PAWN_SCORE;
        }
    }
    score
}
fn is_backward() -> f32 {
    BACKWARD_SCORE
}
fn is_passed() -> f32 {
    TBD
}
