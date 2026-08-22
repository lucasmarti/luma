use std::assert_ne;
#[cfg(debug_assertions)]
use std::debug_assert_ne;

use crate::engine::{
    chess_move::{ChessMove, MoveType},
    movegen::{
        config::{
            BISHOP_DIRECTIONS, BISHOP_MAX_DISTANCE, KING_DIRECTIONS, KING_MAX_DISTANCE,
            KNIGHT_DIRECTIONS, KNIGHT_MAX_DISTANCE, QUEEN_DIRECTIONS, QUEEN_MAX_DISTANCE,
            ROOK_DIRECTIONS, ROOK_MAX_DISTANCE,
        },
        directions::DirectionFn,
        Square,
    },
    piece::Piece,
    position::{CastlingRights, Mve, Position},
    Typ,
};

pub fn slide(position: &Position, from: Square, path: Vec<Square>, piece: Piece) -> Vec<ChessMove> {
    let mut new_chess_moves: Vec<ChessMove> = Vec::new();

    for field in path {
        if position.is_occupied_by_color(field, piece.color) {
            // collision with own
            return new_chess_moves;
        } else if position.is_occupied_by_color(field, piece.color.get_opponent_color()) {
            // capture
            new_chess_moves.push(progress(position, piece, from, field));
            return new_chess_moves;
        } else {
            // empty field
            new_chess_moves.push(progress(position, piece, from, field));
        }
    }
    new_chess_moves
}

pub fn get_moves_for_king_at_square(
    position: &Position,
    piece: Piece,
    square: Square,
) -> Vec<ChessMove> {
    get_moves_for_piece_at_square(position, &KING_DIRECTIONS, piece, KING_MAX_DISTANCE, square)
}

pub fn get_moves_for_queen_at_square(
    position: &Position,
    piece: Piece,
    square: Square,
) -> Vec<ChessMove> {
    get_moves_for_piece_at_square(
        position,
        &QUEEN_DIRECTIONS,
        piece,
        QUEEN_MAX_DISTANCE,
        square,
    )
}

pub fn get_moves_for_rook_at_square(
    position: &Position,
    piece: Piece,
    square: Square,
) -> Vec<ChessMove> {
    get_moves_for_piece_at_square(position, &ROOK_DIRECTIONS, piece, ROOK_MAX_DISTANCE, square)
}

pub fn get_moves_for_knight_at_square(
    position: &Position,
    piece: Piece,
    square: Square,
) -> Vec<ChessMove> {
    get_moves_for_piece_at_square(
        position,
        &KNIGHT_DIRECTIONS,
        piece,
        KNIGHT_MAX_DISTANCE,
        square,
    )
}

pub fn get_moves_for_bishop_at_square(
    position: &Position,
    piece: Piece,
    square: Square,
) -> Vec<ChessMove> {
    get_moves_for_piece_at_square(
        position,
        &BISHOP_DIRECTIONS,
        piece,
        BISHOP_MAX_DISTANCE,
        square,
    )
}

fn get_moves_for_piece_at_square(
    position: &Position,
    directions: &[DirectionFn],
    piece: Piece,
    max_distance: u32,
    from: Square,
) -> Vec<ChessMove> {
    let mut new_chess_moves: Vec<ChessMove> = Vec::new();
    for direction_fn in directions {
        let path = generate_path_with_limit(from, *direction_fn, max_distance);
        new_chess_moves.extend(slide(position, from, path, piece));
    }
    new_chess_moves
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

pub fn progress(position: &Position, piece: Piece, from: Square, to: Square) -> ChessMove {
    debug_assert_ne!(piece.typ, Typ::Pawn);

    let capture = position.get_piece_at(to);
    let move_type = match capture {
        Some(_) => MoveType::Capture,
        None => MoveType::Quiet,
    };
    let mut castling_rights = CastlingRights::from(from);
    if let Some(capture) = capture {
        if capture.typ == Typ::Rook {
            castling_rights = castling_rights | CastlingRights::from(to);
        }
    }
    let mut new_pos = *position;
    new_pos.remove_piece(from);
    new_pos.remove_piece(to);
    new_pos.put_piece(piece, to);
    new_pos.set_en_passant(None);
    new_pos.toggle_player();
    new_pos.remove_castling_right(castling_rights);
    let mve = Mve {
        piece,
        from,
        to,
        move_type: move_type,
    };
    ChessMove {
        move_type: move_type,
        piece,
        from,
        to,
        capture: position.get_piece_at(to),
        pormotion: None,
        position: new_pos,
        mve,
    }
}
#[cfg(test)]
mod tests;
