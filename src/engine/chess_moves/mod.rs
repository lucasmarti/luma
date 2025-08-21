use crate::engine::check::filter_checks;
use crate::engine::chess_moves::configurations::*;
use crate::engine::chess_moves::configurations::{MovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG};
use crate::engine::piece::{Color, Piece};
use crate::engine::position::Position;

pub(crate) mod castle;
mod common;
pub(crate) mod configurations;
pub mod pawn;

pub fn get_current_player_moves(position: &Position) -> Vec<Position> {
    match position.get_player() {
        Color::Black => get_black_moves(position),
        Color::White => get_white_moves(position),
    }
}
pub fn get_white_moves(position: &Position) -> Vec<Position> {
    get_moves(position, WHITE_MOVE_CONFIG)
}

pub fn get_black_moves(position: &Position) -> Vec<Position> {
    get_moves(position, BLACK_MOVE_CONFIG)
}

fn get_moves(position: &Position, config: Config) -> Vec<Position> {
    let mut new_positions: Vec<Position> = Vec::new();
    new_positions.extend(get_new_positions(position, config.bishop, config.bishop_fn));
    new_positions.extend(get_new_positions(position, config.king, config.king_fn));
    new_positions.extend(get_new_positions(position, config.queen, config.queen_fn));
    new_positions.extend(get_new_positions(position, config.rook, config.rook_fn));
    new_positions.extend(get_new_positions(position, config.knight, config.knight_fn));
    new_positions.extend(get_new_positions(position, config.pawn, config.pawn_fn));
    new_positions.extend((config.castle_move_fn)(position));

    filter_checks(new_positions, position.get_player())
}
fn get_new_positions(position: &Position, piece: Piece, get_moves_fn: MovesFn) -> Vec<Position> {
    let mut new_positions: Vec<Position> = Vec::new();
    for square in position.get_squares(piece).iter() {
        new_positions.extend(get_moves_fn(position, piece, square));
    }
    new_positions
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChessMove {
    BlackKingsideCastle,
    BlackQueensideCastle,
    WhiteKingsideCastle,
    WhiteQueensideCastle,
    Progress(Progress),
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Progress {
    piece: Piece,
    from: u32,
    to: u32,
}

#[cfg(test)]
mod tests;
