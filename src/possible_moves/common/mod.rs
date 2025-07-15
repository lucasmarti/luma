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

pub fn get_piece_moves(
    position: &Position,
    from: u32,
    directions: &[DirectionFn],
    piece: Piece,
    max_distance: u32,
) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();

    for direction_fn in directions {
        let path = generate_path_with_limit(from, *direction_fn, max_distance);
        moves.extend(slide(position, from, path, piece));
    }

    moves
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
