use crate::{
    chess_moves::ChessMove,
    directions,
    piece::{Color, Piece, BLACK_BISHOP, WHITE_BISHOP},
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
    let bishop = get_bishop(color);
    let mut moves: Vec<ChessMove> = Vec::new();
    moves.extend(explore(
        position,
        from,
        directions::all_up_left(from),
        bishop,
    ));
    moves.extend(explore(
        position,
        from,
        directions::all_up_right(from),
        bishop,
    ));
    moves.extend(explore(
        position,
        from,
        directions::all_down_left(from),
        bishop,
    ));
    moves.extend(explore(
        position,
        from,
        directions::all_down_right(from),
        bishop,
    ));
    moves
}

fn get_bishop(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_BISHOP,
        Color::White => WHITE_BISHOP,
    }
}
mod tests;
