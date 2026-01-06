use crate::engine::{
    check::is_check,
    chess_moves::get_current_player_moves,
    directions::squares::Square,
    piece::{
        Color,
        Piece::{self},
    },
    position::Position,
    search_algorithms::get_best_move,
};

pub fn get_next_move(position: &Position) -> MoveOrEnd {
    match get_best_move(*position) {
        Some(position) => MoveOrEnd::Move(position),
        None => {
            if is_check(position, position.get_player()) {
                MoveOrEnd::GameEnd(GameEnd::Victory(position.get_player().get_opponent_color()))
            } else {
                MoveOrEnd::GameEnd(GameEnd::Draw)
            }
        }
    }
}

pub fn get_possible_moves(position: &Position) -> Result<Vec<Position>, GameEnd> {
    let positions: Vec<Position> = get_current_player_moves(position);

    if positions.is_empty() {
        if is_check(position, position.get_player()) {
            Err(GameEnd::Victory(position.get_player().get_opponent_color()))
        } else {
            Err(GameEnd::Draw)
        }
    } else {
        Ok(positions)
    }
}

pub fn get_check_square(position: &Position) -> Option<Square> {
    if is_check(position, position.get_player()) {
        let king = match position.get_player() {
            piece::Color::Black => Piece::BlackKing,
            piece::Color::White => Piece::WhiteKing,
        };
        if let Some(square) = position.get_squares(king).iter().next() {
            return Some(square);
        }
    }
    None
}
#[derive(Debug)]
pub enum MoveOrEnd {
    Move(Position),
    GameEnd(GameEnd),
}
#[derive(Debug)]
pub enum GameEnd {
    Draw,
    Victory(Color),
}
mod check;
pub mod chess_moves;
pub mod directions;
mod evaluation;
pub mod piece;
pub mod position;
pub mod search_algorithms;
#[cfg(test)]
mod tests;
