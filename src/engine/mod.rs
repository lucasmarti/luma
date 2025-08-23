use rand::seq::IndexedRandom;
use std::collections::HashMap;

use crate::engine::{
    check::{filter_checks, is_check},
    chess_moves::{
        castle::{BLACK_KING_STARTING_POSITION, WHITE_KING_STARTING_POSITION},
        configurations::{CastleMovesFn, MovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG},
        get_current_player_moves,
        pawn::get_pawn_moves,
    },
    minimax::chess_impl::get_best_move,
    piece::{Piece, BLACK_KING, BLACK_PAWN, WHITE_KING, WHITE_PAWN},
    position::Position,
};

pub fn get_next_move(position: &Position) -> Option<Position> {
    get_best_move(*position)
}

pub fn get_check(position: &Position) -> Option<u32> {
    if is_check(position, position.get_player()) {
        let king = match position.get_player() {
            piece::Color::Black => BLACK_KING,
            piece::Color::White => WHITE_KING,
        };
        return get_first_piece(position, king);
    }
    None
}
fn get_first_piece(position: &Position, piece: Piece) -> Option<u32> {
    for square in position.get_squares(piece).iter() {
        return Some(square);
    }
    None
}

#[allow(dead_code)]
fn get_random_move(position: &Position) -> Option<Position> {
    let positions = get_current_player_moves(position);
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

pub fn promote(position: &Position, from: u32, to: u32, piece: Piece) -> Option<Position> {
    let possible_positions = match position.get_player() {
        piece::Color::Black => get_pawn_moves(position, BLACK_PAWN, from),
        piece::Color::White => get_pawn_moves(position, WHITE_PAWN, from),
    };
    for new_position in possible_positions {
        if new_position.is_occupied_by_piece(to, piece) {
            return Some(new_position);
        }
    }
    return None;
}

fn to_target_positions_map(positions: Vec<Position>) -> HashMap<u32, Position> {
    let mut map: HashMap<u32, Position> = HashMap::new();
    for position in positions {
        if let Some(target) = position.get_to_square() {
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
mod heuristic;
mod minimax;
pub mod piece;
pub mod position;
#[cfg(test)]
mod tests;
