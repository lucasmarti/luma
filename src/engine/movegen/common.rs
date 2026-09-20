#[cfg(debug_assertions)]
use std::debug_assert_ne;

use crate::engine::{
    chess_move::MoveType,
    movegen::{config::MoveConfig, directions::DirectionFn, Square},
    piece::Piece,
    position::{Mve, Position},
    Typ,
};

pub fn slide(
    position: &Position,
    from: Square,
    path: Vec<Square>,
    piece: Piece,
    moves: &mut Vec<Mve>,
) {
    for field in path {
        if position.is_occupied_by_color(field, piece.color) {
            // collision with own
            return;
        } else if position.is_occupied_by_color(field, piece.color.get_opponent_color()) {
            // capture
            moves.push(progress(position, piece, from, field));
            return;
        } else {
            // empty field
            moves.push(progress(position, piece, from, field));
        }
    }
}
pub fn generate_moves_for_typ(
    position: &Position,
    piece: Piece,
    from: Square,
    config: &MoveConfig,
    moves: &mut Vec<Mve>,
) {
    for direction_fn in config.directions {
        let path = generate_path_with_limit(from, *direction_fn, config.max_distance);
        slide(position, from, path, piece, moves);
    }
}

pub fn generate_path_with_limit(
    from: Square,
    direction_fn: DirectionFn,
    max_distance: u32,
) -> Vec<Square> {
    let mut path: Vec<Square> = Vec::new();
    let mut current_pos = from;
    let mut distance = 0;

    while distance < max_distance {
        if let Some(next_pos) = direction_fn(current_pos) {
            path.push(next_pos);
            current_pos = next_pos;
            distance += 1;
        } else {
            break;
        }
    }

    path
}

pub fn progress(position: &Position, piece: Piece, from: Square, to: Square) -> Mve {
    debug_assert_ne!(piece.typ, Typ::Pawn);

    let capture = position.get_piece_at(to);
    let move_type = match capture {
        Some(_) => MoveType::Capture,
        None => MoveType::Quiet,
    };
    Mve {
        piece,
        from,
        to,
        move_type,
    }
}
//#[cfg(test)]
//mod tests;
