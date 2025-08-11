use crate::engine::chess_moves::configurations::*;
use crate::engine::chess_moves::configurations::{MovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG};
use crate::engine::piece::Piece;
use crate::engine::position::Position;

pub(crate) mod castle;
mod common;
pub(crate) mod configurations;
pub mod pawn;

pub fn get_white_moves(position: &Position) -> Vec<Position> {
    get_moves(position, WHITE_MOVE_CONFIG)
}

pub fn get_black_moves(position: &Position) -> Vec<Position> {
    get_moves(position, BLACK_MOVE_CONFIG)
}

fn get_moves(position: &Position, config: Config) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    positions.extend(get_new_positions(position, config.bishop, config.bishop_fn));
    positions.extend(get_new_positions(position, config.king, config.king_fn));
    positions.extend(get_new_positions(position, config.queen, config.queen_fn));
    positions.extend(get_new_positions(position, config.rook, config.rook_fn));
    positions.extend(get_new_positions(position, config.knight, config.knight_fn));
    positions.extend(get_new_positions(position, config.pawn, config.pawn_fn));
    positions.extend((config.castle_move_fn)(position));
    positions
}

fn get_new_positions(position: &Position, piece: Piece, get_moves_fn: MovesFn) -> Vec<Position> {
    let mut new_positions: Vec<Position> = Vec::new();
    for square in position.get_squares(piece).iter() {
        new_positions.extend(get_moves_fn(position, piece, square));
    }
    new_positions
}
mod tests;
