mod evaluation;
mod movegen;
mod piece;
mod position;
mod search_algorithms;

pub use movegen::ChessMove;
pub use movegen::MoveType;
pub use movegen::Square;
pub use piece::*;
pub use position::Position;

use crate::engine::{
    movegen::get_current_player_moves, movegen::is_check, search_algorithms::get_best_move,
};
pub fn get_next_move(position: &Position) -> MoveOrEnd {
    match get_best_move(*position) {
        Some(chess_move) => MoveOrEnd::Move(chess_move),
        None => {
            if is_check(position, position.get_player()) {
                MoveOrEnd::GameEnd(GameEnd::Victory)
            } else {
                MoveOrEnd::GameEnd(GameEnd::Draw)
            }
        }
    }
}

pub fn get_possible_moves(position: &Position) -> Result<Vec<ChessMove>, GameEnd> {
    let chess_moves: Vec<ChessMove> = get_current_player_moves(position);

    if chess_moves.is_empty() {
        if is_check(position, position.get_player()) {
            Err(GameEnd::Victory)
        } else {
            Err(GameEnd::Draw)
        }
    } else {
        Ok(chess_moves)
    }
}

pub fn get_check_square(position: &Position) -> Option<Square> {
    is_check(position, position.get_player())
        .then(|| position.get_king_square(position.get_player()))
}
#[derive(Debug)]
pub enum MoveOrEnd {
    Move(ChessMove),
    GameEnd(GameEnd),
}
#[derive(Debug)]
pub enum GameEnd {
    Draw,
    Victory,
}

#[cfg(test)]
mod tests;
