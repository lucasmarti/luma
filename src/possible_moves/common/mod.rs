use crate::{
    bitboard::Bitboard,
    chess_moves::{ChessMove, Progress},
    piece::{Color, Piece},
    position::Position,
};

pub fn explore(position: &Position, from: u32, path: Vec<u32>, piece: Piece) -> Vec<ChessMove> {
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

fn get_own_pieces(position: &Position, color: Color) -> Bitboard {
    match color {
        Color::Black => position.get_black(),
        Color::White => position.get_white(),
    }
}

fn get_opponent_pieces(position: &Position, color: Color) -> Bitboard {
    match color {
        Color::White => position.get_black(),
        Color::Black => position.get_white(),
    }
}

mod tests;
