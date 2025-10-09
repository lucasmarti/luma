use crate::engine::{
    chess_moves::{
        castling::remove_castling_rights_if_necessary,
        configurations::{
            BISHOP_DIRECTIONS, BISHOP_MAX_DISTANCE, KING_DIRECTIONS, KING_MAX_DISTANCE,
            KNIGHT_DIRECTIONS, KNIGHT_MAX_DISTANCE, QUEEN_DIRECTIONS, QUEEN_MAX_DISTANCE,
            ROOK_DIRECTIONS, ROOK_MAX_DISTANCE,
        },
        pawn::set_en_passant_if_necessary,
        ChessMove, MoveType,
    },
    directions::{squares::Square, DirectionFn},
    piece::Piece,
    position::Position,
};

pub fn slide(position: &Position, from: Square, path: Vec<Square>, piece: Piece) -> Vec<Position> {
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
    square: Square,
) -> Vec<Position> {
    get_moves_for_piece_at_square(position, &KING_DIRECTIONS, piece, KING_MAX_DISTANCE, square)
}

pub fn get_moves_for_queen_at_square(
    position: &Position,
    piece: Piece,
    square: Square,
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
    square: Square,
) -> Vec<Position> {
    get_moves_for_piece_at_square(position, &ROOK_DIRECTIONS, piece, ROOK_MAX_DISTANCE, square)
}

pub fn get_moves_for_knight_at_square(
    position: &Position,
    piece: Piece,
    square: Square,
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
    square: Square,
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
    from: Square,
) -> Vec<Position> {
    let mut new_positions: Vec<Position> = Vec::new();
    for direction_fn in directions {
        let path = generate_path_with_limit(from, *direction_fn, max_distance);
        new_positions.extend(slide(position, from, path, piece));
    }
    new_positions
}
fn generate_path_with_limit(
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

pub fn progess(position: &Position, piece: Piece, from: Square, to: Square) -> Position {
    let tuple = match position.get_piece_at(to) {
        Some(piece) => (MoveType::Capture, Some(piece)),
        None => (MoveType::Quiet, None),
    };
    let chess_move = ChessMove {
        move_type: tuple.0,
        piece,
        from,
        to,
        capture: tuple.1,
        pormotion: None,
    };

    let mut new_position = position
        .remove_piece(from)
        .remove_piece(to)
        .put_piece(piece, to)
        .toggle_player()
        .set_chess_move(chess_move);
    new_position = set_en_passant_if_necessary(new_position, piece, from, to);
    new_position = remove_castling_rights_if_necessary(new_position, from);
    new_position
}
#[cfg(test)]
mod tests;
