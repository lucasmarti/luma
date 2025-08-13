use rand::seq::IndexedRandom;
use std::collections::HashMap;

use crate::engine::{
    check::filter_checks,
    chess_moves::{
        castle::{BLACK_KING_STARTING_POSITION, WHITE_KING_STARTING_POSITION},
        configurations::{CastleMovesFn, MovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG},
        get_black_moves, get_white_moves,
    },
    piece::{Piece, BLACK_KING, WHITE_KING},
    position::Position,
};

pub fn get_next_move(position: &Position) -> Option<Position> {
    let positions = match position.get_player() {
        piece::Color::Black => get_black_moves(position),
        piece::Color::White => get_white_moves(position),
    };
    match positions.choose(&mut rand::rng()) {
        Some(position) => Some(*position),
        None => None,
    }
}

pub fn is_valid_drag_square(position: &Position, square: u32) -> bool {
    match position.get_piece_at(square) {
        Some(piece) => piece.color == position.get_player(),
        None => false,
    }
}

pub fn get_valid_drop_positions(position: &Position, from: u32) -> HashMap<u32, Position> {
    let mut map: HashMap<u32, Position> = HashMap::new();
    if let Some(piece) = position.get_piece_at(from) {
        let moves_fn = get_moves_fn(piece);
        let mut positions = moves_fn(position, piece, from);
        if let Some(castle_moves_fn) = get_castle_moves_fn(from, piece) {
            positions.extend(castle_moves_fn(position));
        }
        positions = filter_checks(positions, position.get_player());
        map = to_target_positions_map(positions);
    }
    map
}

fn to_target_positions_map(positions: Vec<Position>) -> HashMap<u32, Position> {
    let mut map: HashMap<u32, Position> = HashMap::new();
    for position in positions {
        if let Some(target) = position.get_target() {
            map.insert(target, position);
        }
    }
    map
}

fn get_castle_moves_fn(square: u32, piece: Piece) -> Option<CastleMovesFn> {
    if square == WHITE_KING_STARTING_POSITION && piece == WHITE_KING {
        Some(WHITE_MOVE_CONFIG.castle_move_fn)
    } else if square == BLACK_KING_STARTING_POSITION && piece == BLACK_KING {
        Some(BLACK_MOVE_CONFIG.castle_move_fn)
    } else {
        None
    }
}

fn get_moves_fn(piece: Piece) -> MovesFn {
    let config = match piece.color {
        piece::Color::Black => BLACK_MOVE_CONFIG,
        piece::Color::White => WHITE_MOVE_CONFIG,
    };
    let moves_fn: MovesFn = match piece.typ {
        piece::Typ::King => config.king_fn,
        piece::Typ::Queen => config.queen_fn,
        piece::Typ::Rook => config.rook_fn,
        piece::Typ::Pawn => config.pawn_fn,
        piece::Typ::Knight => config.knight_fn,
        piece::Typ::Bishop => config.bishop_fn,
    };
    moves_fn
}

mod check;
mod chess_moves;
mod directions;
mod evaluation;
mod minimax;
pub mod piece;
pub mod position;
#[cfg(test)]
mod tests;
