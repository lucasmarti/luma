use crate::{
    bitboard::Bitboard,
    chess_moves::{ChessMove, Progress},
    directions,
    piece::{Color, Piece, BLACK_KNIGHT, WHITE_KNIGHT},
    position::Position,
    possible_moves::common::get_own_pieces,
};

pub fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::Black)
}
pub fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::White)
}

fn get_possible_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let knight = get_knight(color);
    let own_pieces: Bitboard = get_own_pieces(position, color);
    let mut moves: Vec<ChessMove> = Vec::new();
    for field in get_fields(from) {
        if !own_pieces.contains(field) {
            moves.push(ChessMove::Progress(Progress {
                from: from,
                to: field,
                piece: knight,
            }));
        }
    }
    moves
}

fn get_knight(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_KNIGHT,
        Color::White => WHITE_KNIGHT,
    }
}

fn get_fields(from: u32) -> Vec<u32> {
    let mut fields: Vec<u32> = Vec::new();
    if let Some(field) = directions::right_right_down(from) {
        fields.push(field);
    }
    if let Some(field) = directions::right_right_up(from) {
        fields.push(field);
    }
    if let Some(field) = directions::left_left_up(from) {
        fields.push(field);
    }
    if let Some(field) = directions::left_left_down(from) {
        fields.push(field);
    }
    if let Some(field) = directions::up_up_left(from) {
        fields.push(field);
    }
    if let Some(field) = directions::up_up_right(from) {
        fields.push(field);
    }
    if let Some(field) = directions::down_down_left(from) {
        fields.push(field);
    }
    if let Some(field) = directions::down_down_right(from) {
        fields.push(field);
    }
    fields
}
mod tests;
