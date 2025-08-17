use crate::engine::{
    chess_moves::{
        castle::disallow_castle_if_necessary,
        configurations::{
            BISHOP_DIRECTIONS, BISHOP_MAX_DISTANCE, KING_DIRECTIONS, KING_MAX_DISTANCE,
            KNIGHT_DIRECTIONS, KNIGHT_MAX_DISTANCE, QUEEN_DIRECTIONS, QUEEN_MAX_DISTANCE,
            ROOK_DIRECTIONS, ROOK_MAX_DISTANCE,
        },
        pawn::set_en_passant_if_necessary,
    },
    directions::DirectionFn,
    piece::Piece,
    position::Position,
};

pub fn slide(position: &Position, from: u32, path: Vec<u32>, piece: Piece) -> Vec<Position> {
    let mut new_positions: Vec<Position> = Vec::new();

    for field in path {
        if position.is_occupied_by_color(field, piece.color) {
            // collision with own
            return new_positions;
        } else if position.is_occupied_by_color(field, piece.color.get_opponent_color()) {
            // capture
            new_positions.push(progess(position, piece, from, field));
            return new_positions;
        } else {
            // empty field
            new_positions.push(progess(position, piece, from, field));
        }
    }
    new_positions
}

pub fn get_moves_for_king_at_square(
    position: &Position,
    piece: Piece,
    square: u32,
) -> Vec<Position> {
    get_moves_for_piece_at_square(position, &KING_DIRECTIONS, piece, KING_MAX_DISTANCE, square)
}

pub fn get_moves_for_queen_at_square(
    position: &Position,
    piece: Piece,
    square: u32,
) -> Vec<Position> {
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
    square: u32,
) -> Vec<Position> {
    get_moves_for_piece_at_square(position, &ROOK_DIRECTIONS, piece, ROOK_MAX_DISTANCE, square)
}

pub fn get_moves_for_knight_at_square(
    position: &Position,
    piece: Piece,
    square: u32,
) -> Vec<Position> {
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
    square: u32,
) -> Vec<Position> {
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
    from: u32,
) -> Vec<Position> {
    let mut new_positions: Vec<Position> = Vec::new();
    for direction_fn in directions {
        let path = generate_path_with_limit(from, *direction_fn, max_distance);
        new_positions.extend(slide(position, from, path, piece));
    }
    new_positions
}
fn generate_path_with_limit(from: u32, direction_fn: DirectionFn, max_distance: u32) -> Vec<u32> {
    let mut path: Vec<u32> = Vec::new();
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

pub fn progess(position: &Position, piece: Piece, from: u32, to: u32) -> Position {
    let mut new_position = position
        .remove_piece(from)
        .remove_piece(to)
        .put_piece(piece, to)
        .toggle_player()
        .set_target(to);
    new_position = set_en_passant_if_necessary(new_position, piece, from, to);
    new_position = disallow_castle_if_necessary(new_position, from);
    new_position
}
#[cfg(test)]
mod tests;
