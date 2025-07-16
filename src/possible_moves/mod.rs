use crate::position::print::Print;
use crate::{
    check::is_check,
    piece::{
        self, Color, Piece, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN,
        BLACK_ROOK, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    position::Position,
};

pub mod bishop;
mod common;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

type GetPossibleMovesFn = fn(position: &Position, from: u32, piece: Piece) -> Vec<Position>;

const BLACK_TYPES: [(Piece, GetPossibleMovesFn); 6] = [
    (BLACK_KING, king::get_possible_moves),
    (BLACK_QUEEN, queen::get_possible_moves),
    (BLACK_BISHOP, bishop::get_possible_moves),
    (BLACK_KNIGHT, knight::get_possible_moves),
    (BLACK_PAWN, pawn::get_possible_moves),
    (BLACK_ROOK, rook::get_possible_moves),
];

const WHITE_TYPES: [(Piece, GetPossibleMovesFn); 6] = [
    (WHITE_KING, king::get_possible_moves),
    (WHITE_QUEEN, queen::get_possible_moves),
    (WHITE_BISHOP, bishop::get_possible_moves),
    (WHITE_KNIGHT, knight::get_possible_moves),
    (WHITE_PAWN, pawn::get_possible_moves),
    (WHITE_ROOK, rook::get_possible_moves),
];
pub fn get_all_moves(position: &Position, color: Color) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();

    for _type in get_types(color) {
        positions.extend(get_moves(position, _type.1, _type.0));
    }
    positions
}

fn get_types(color: Color) -> [(Piece, GetPossibleMovesFn); 6] {
    match color {
        Color::Black => BLACK_TYPES,
        Color::White => WHITE_TYPES,
    }
}

fn get_moves(
    position: &Position,
    get_possible_moves_fn: GetPossibleMovesFn,
    piece: Piece,
) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    for square in position.get_squares(piece).iter() {
        for position in get_possible_moves_fn(position, square, piece) {
            if !is_check(&position, piece.color) {
                positions.push(position);
            }
        }
    }
    println!(
        "Piece = {:?}, Number of Moves = {:?}",
        piece,
        positions.len()
    );
    positions
}

mod tests;
