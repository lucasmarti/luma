mod bitboard;
mod chess_move;
mod evaluation;
mod movegen;
mod piece;
mod position;
mod search_algorithms;

use chess_move::ChessMove;
pub use chess_move::MoveType;
pub use movegen::get_check_square;
pub use movegen::Square;
pub use piece::*;
pub use position::Mve;
pub use position::Position;

use crate::engine::movegen::get_moves_by_color;
use crate::engine::movegen::is_check;
use crate::engine::movegen::MoveMode;
use crate::engine::search_algorithms::search_best_move;

pub fn get_next_move(position: &Position) -> MoveOrEnd {
    match search_best_move(*position) {
        Some(chess_move) => MoveOrEnd::Move(chess_move.into()),
        None => {
            if is_check(position, position.get_player()) {
                MoveOrEnd::GameEnd(GameEnd::Victory)
            } else {
                MoveOrEnd::GameEnd(GameEnd::Draw)
            }
        }
    }
}

pub fn get_possible_moves(position: &Position) -> Result<Vec<Mve>, GameEnd> {
    let chess_moves: Vec<ChessMove> =
        get_moves_by_color(position, position.get_player(), MoveMode::Legal);
    let mves: Vec<Mve> = chess_moves.into_iter().map(Into::into).collect();

    if mves.is_empty() {
        if is_check(position, position.get_player()) {
            Err(GameEnd::Victory)
        } else {
            Err(GameEnd::Draw)
        }
    } else {
        Ok(mves)
    }
}
#[derive(Debug)]
pub enum MoveOrEnd {
    Move(Mve),
    GameEnd(GameEnd),
}
#[derive(Debug)]
pub enum GameEnd {
    Draw,
    Victory,
}
#[cfg(test)]
mod tests;
