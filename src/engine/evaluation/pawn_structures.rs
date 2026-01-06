use crate::engine::{
    directions::{self, squares::Square, DirectionFn},
    piece::Piece,
    position::Position,
};
const PAWN_IN_FRONT_SCORE: f32 = -3.0;
const ISOLATED_PAWN_SCORE: f32 = -4.0;
const PASSED_PAWN_SCORE: f32 = 15.0;

pub fn count_black(position: &Position) -> f32 {
    let mut score: f32 = 0.0;
    score += get_doubled_pawns(position, Piece::BlackPawn);
    score += get_isolated_pawns(position, Piece::BlackPawn);
    score += get_passed_pawns(position, Piece::BlackPawn);
    score
}
pub fn count_white(position: &Position) -> f32 {
    let mut score: f32 = 0.0;
    score += get_doubled_pawns(position, Piece::WhitePawn);
    score += get_isolated_pawns(position, Piece::WhitePawn);
    score += get_passed_pawns(position, Piece::WhitePawn);
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
fn has_piece_in_direction(
    position: &Position,
    square: Square,
    direction_fn: DirectionFn,
    piece: Piece,
) -> bool {
    if let Some(square) = direction_fn(square) {
        if position.is_occupied_by_piece(square, piece) {
            return true;
        } else {
            return has_piece_in_direction(position, square, direction_fn, piece);
        }
    }
    false
}

pub fn get_passed_pawns(position: &Position, pawn: Piece) -> f32 {
    let mut score = 0.0;
    let opponent_pawn = match pawn.get_color() {
        crate::engine::piece::Color::Black => Piece::WhitePawn,
        crate::engine::piece::Color::White => Piece::BlackPawn,
    };

    let direction_fn = match pawn.get_color() {
        crate::engine::piece::Color::Black => directions::down,
        crate::engine::piece::Color::White => directions::up,
    };

    for square in position.get_squares(pawn).iter() {
        let mut has_pawn_in_front_on_left_column = false;
        let mut has_pawn_in_front_on_right_column = false;

        if let Some(left) = directions::left(square) {
            has_pawn_in_front_on_left_column =
                has_piece_in_direction(position, left, direction_fn, opponent_pawn);
        }
        if let Some(right) = directions::right(square) {
            has_pawn_in_front_on_right_column =
                has_piece_in_direction(position, right, direction_fn, opponent_pawn);
        }
        let has_pawn_in_front_on_column =
            has_piece_in_direction(position, square, direction_fn, opponent_pawn);
        if !has_pawn_in_front_on_column
            && !has_pawn_in_front_on_left_column
            && !has_pawn_in_front_on_right_column
        {
            score += PASSED_PAWN_SCORE;
        }
    }
    score
}
