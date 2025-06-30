use crate::{
    bitboard::Bitboard,
    chess_moves::{ChessMove, Progress},
    directions::{self, left},
    piece::{Color, Piece, BLACK_KING, WHITE_KING},
    position::Position,
    possible_moves::common::{get_opponent_pieces, get_own_pieces},
};

pub fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::Black)
}
pub fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::White)
}

fn get_possible_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let king = get_king(color);
    let own_pieces: Bitboard = get_own_pieces(position, color);
    let mut moves: Vec<ChessMove> = Vec::new();
    for neighbour in get_neighbours(from) {
        if !own_pieces.contains(neighbour) {
            moves.push(ChessMove::Progress(Progress {
                from: from,
                to: neighbour,
                piece: king,
            }));
        }
    }
    moves
}

fn get_king(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_KING,
        Color::White => WHITE_KING,
    }
}

fn get_neighbours(index: u32) -> Vec<u32> {
    let mut neighbours: Vec<u32> = Vec::new();
    if let Some(neighbour) = directions::left(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::right(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::up(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::down(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::up_left(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::up_right(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::down_left(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::down_right(index) {
        neighbours.push(neighbour);
    }
    neighbours
}
mod tests;
