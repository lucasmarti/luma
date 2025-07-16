use crate::{
    chess_moves::*,
    directions::DirectionFn,
    piece::{Color, Piece},
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

pub fn get_piece_moves(
    position: &Position,
    from: u32,
    directions: &[DirectionFn],
    piece: Piece,
    max_distance: u32,
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

mod tests;
