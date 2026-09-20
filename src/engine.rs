mod bitboard;
mod chess_move;
mod evaluation;
mod movegen;
mod piece;
mod position;
mod search_algorithms;

use crate::engine::movegen::generate_moves;
use crate::engine::movegen::is_check;
use crate::engine::search_algorithms::search_best_move;
pub use chess_move::MoveType;
pub use movegen::get_check_square;
pub use movegen::Square;
pub use piece::*;
pub use position::Mve;
pub use position::Position;

pub fn get_next_move(position: &Position) -> MoveOrEnd {
    match search_best_move(*position) {
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

pub fn get_possible_moves(position: &Position) -> Result<Vec<Mve>, GameEnd> {
    let chess_moves: Vec<Mve> = generate_moves(position, position.get_player());

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
//#[cfg(test)]
//mod tests;
