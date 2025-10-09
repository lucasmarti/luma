use rand::seq::IndexedRandom;

use crate::engine::{
    check::{filter_checks, is_check},
    chess_moves::{
        configurations::{CastleMovesFn, MovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG},
        get_current_player_moves,
    },
    directions::squares::{Square, BLACK_KING_STARTING_POSITION, WHITE_KING_STARTING_POSITION},
    minimax::chess_impl::get_best_move,
    piece::{Piece, BLACK_KING, WHITE_KING},
    position::Position,
};

pub fn get_next_move(position: &Position) -> Option<Position> {
    get_best_move(*position)
}

pub fn get_check(position: &Position) -> Option<Square> {
    if is_check(position, position.get_player()) {
        let king = match position.get_player() {
            piece::Color::Black => BLACK_KING,
            piece::Color::White => WHITE_KING,
        };
        return get_first_piece(position, king);
    }
    None
}
fn get_first_piece(position: &Position, piece: Piece) -> Option<Square> {
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

pub fn is_valid_drag_square(position: &Position, square: Square) -> bool {
    match position.get_piece_at(square) {
        Some(piece) => piece.color == position.get_player(),
        None => false,
    }
}

pub fn get_valid_drop_positions(position: &Position, from: Square) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    if let Some(piece) = position.get_piece_at(from) {
        let moves_fn = get_moves_fn(piece);
        positions.extend(moves_fn(position, piece, from));
        if let Some(castle_moves_fn) = get_castle_moves_fn(from, piece) {
            positions.extend(castle_moves_fn(position));
        }
        positions = filter_checks(positions, position.get_player());
    }
    positions
}

fn get_castle_moves_fn(square: Square, piece: Piece) -> Option<CastleMovesFn> {
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
pub mod directions;
mod heuristic;
mod minimax;
pub mod piece;
pub mod position;
#[cfg(test)]
mod tests;
