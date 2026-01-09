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

pub fn slide(position: &Position, from: Square, path: Vec<Square>, piece: Piece) -> Vec<ChessMove> {
    let mut new_chess_moves: Vec<ChessMove> = Vec::new();

    for field in path {
        if position.is_occupied_by_color(field, piece.get_color()) {
            // collision with own
            return new_chess_moves;
        } else if position.is_occupied_by_color(field, piece.get_color().get_opponent_color()) {
            // capture
            new_chess_moves.push(progess(position, piece, from, field));
            return new_chess_moves;
        } else {
            // empty field
            new_chess_moves.push(progess(position, piece, from, field));
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

pub fn progess(position: &Position, piece: Piece, from: Square, to: Square) -> ChessMove {
    let tuple = match position.get_piece_at(to) {
        Some(piece) => (MoveType::Capture, Some(piece)),
        None => (MoveType::Quiet, None),
    };
    let mut new_position = position
        .remove_piece(from)
        .remove_piece(to)
        .put_piece(piece, to)
        .toggle_player();
    new_position = set_en_passant_if_necessary(new_position, piece, from, to);
    new_position = remove_castling_rights_if_necessary(new_position, from);
    ChessMove {
        move_type: tuple.0,
        piece,
        from,
        to,
        capture: tuple.1,
        pormotion: None,
        position: new_position,
    }
}
#[cfg(test)]
mod tests;
