use crate::{
    chess_moves::ChessMove,
    directions,
    piece::{Color, Piece, BLACK_ROOK, WHITE_ROOK},
    position::Position,
    possible_moves::common::explore,
};

pub fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::White)
}

pub fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::Black)
}
fn get_possible_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let rook = get_rook(color);
    let mut moves: Vec<ChessMove> = Vec::new();
    moves.extend(explore(position, from, directions::all_up(from), rook));
    moves.extend(explore(position, from, directions::all_left(from), rook));
    moves.extend(explore(position, from, directions::all_down(from), rook));
    moves.extend(explore(position, from, directions::all_right(from), rook));
    moves
}

fn get_rook(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_ROOK,
        Color::White => WHITE_ROOK,
    }
}

mod tests;
