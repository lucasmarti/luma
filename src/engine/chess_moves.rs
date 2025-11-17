use crate::engine::check::filter_checks;
use crate::engine::chess_moves::configurations::*;
use crate::engine::chess_moves::configurations::{MovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG};
use crate::engine::directions::squares::Square;
use crate::engine::piece::{Color, Piece};
use crate::engine::position::{CastlingType, Position};

pub(crate) mod castling;
mod common;
pub(crate) mod configurations;
pub mod pawn;

pub fn get_current_player_moves(position: &Position) -> Vec<Position> {
    match position.get_player() {
        Color::Black => get_black_moves(position),
        Color::White => get_white_moves(position),
    }
}

pub fn get_black_mobility(position: &Position) -> Vec<Position> {
    get_moves(position, BLACK_MOVE_CONFIG, true)
}

pub fn get_white_mobility(position: &Position) -> Vec<Position> {
    get_moves(position, WHITE_MOVE_CONFIG, true)
}

pub fn get_white_moves(position: &Position) -> Vec<Position> {
    get_moves(position, WHITE_MOVE_CONFIG, false)
}

pub fn get_black_moves(position: &Position) -> Vec<Position> {
    get_moves(position, BLACK_MOVE_CONFIG, false)
}

fn get_moves(position: &Position, config: Config, ignore_checks: bool) -> Vec<Position> {
    let mut new_positions: Vec<Position> = Vec::new();
    new_positions.extend(get_new_positions(position, config.bishop, config.bishop_fn));
    new_positions.extend(get_new_positions(position, config.king, config.king_fn));
    new_positions.extend(get_new_positions(position, config.queen, config.queen_fn));
    new_positions.extend(get_new_positions(position, config.rook, config.rook_fn));
    new_positions.extend(get_new_positions(position, config.knight, config.knight_fn));
    new_positions.extend(get_new_positions(position, config.pawn, config.pawn_fn));
    new_positions.extend((config.castling_move_fn)(position));

    if ignore_checks {
        new_positions
    } else {
        filter_checks(new_positions, position.get_player())
    }
}
fn get_new_positions(position: &Position, piece: Piece, get_moves_fn: MovesFn) -> Vec<Position> {
    let mut new_positions: Vec<Position> = Vec::new();
    for square in position.get_squares(piece).iter() {
        new_positions.extend(get_moves_fn(position, piece, square));
    }
    new_positions
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ChessMove {
    pub move_type: MoveType,
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub capture: Option<Piece>,
    pub pormotion: Option<Piece>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveType {
    Quiet,
    Capture,
    Promotion,
    PromotionCapture,
    EnPassant,
    Castling { castling_type: CastlingType },
}

#[cfg(test)]
mod tests;
