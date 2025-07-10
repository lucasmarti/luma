use crate::{
    bitboard::Bitboard,
    chess_moves::{ChessMove, Progress},
    directions::DirectionFn,
    piece::{Color, Piece},
    position::Position,
};

pub fn slide(position: &Position, from: u32, path: Vec<u32>, piece: Piece) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    let own_pieces: Bitboard = get_own_pieces(position, piece.color);
    let opponent_pieces: Bitboard = get_opponent_pieces(position, piece.color);

    for field in path {
        if own_pieces.contains(field) {
            // collision with own
            return moves;
        } else if opponent_pieces.contains(field) {
            // capture
            moves.push(ChessMove::Progress(Progress {
                from: from,
                to: field,
                piece: piece,
            }));
            return moves;
        } else {
            // empty field
            moves.push(ChessMove::Progress(Progress {
                from: from,
                to: field,
                piece: piece,
            }));
        }
    }
    moves
}

pub fn get_own_pieces(position: &Position, color: Color) -> Bitboard {
    match color {
        Color::Black => position.get_black(),
        Color::White => position.get_white(),
    }
}

pub fn get_opponent_pieces(position: &Position, color: Color) -> Bitboard {
    match color {
        Color::White => position.get_black(),
        Color::Black => position.get_white(),
    }
}

pub fn get_single_step_moves(
    position: &Position,
    from: u32,
    color: Color,
    directions: &[DirectionFn],
    piece: Piece,
) -> Vec<ChessMove> {
    let own_pieces: Bitboard = get_own_pieces(position, color);
    let mut moves: Vec<ChessMove> = Vec::new();

    for direction_fn in directions {
        if let Some(field) = direction_fn(from) {
            if !own_pieces.contains(field) {
                moves.push(ChessMove::Progress(Progress {
                    from,
                    to: field,
                    piece,
                }));
            }
        }
    }

    moves
}

pub fn get_sliding_moves(
    position: &Position,
    from: u32,
    color: Color,
    direction_fns: &[DirectionFn],
    piece: Piece,
) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();

    for direction_fn in direction_fns {
        let path = generate_path_in_direction(from, *direction_fn);
        moves.extend(slide(position, from, path, piece));
    }

    moves
}

fn generate_path_in_direction(from: u32, direction_fn: DirectionFn) -> Vec<u32> {
    let mut path: Vec<u32> = Vec::new();
    let mut current_pos = from;

    while let Some(next_pos) = direction_fn(current_pos) {
        path.push(next_pos);
        current_pos = next_pos;
    }

    path
}

mod tests;
