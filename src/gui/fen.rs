use std::collections::HashMap;

use crate::engine::piece::{
    Piece, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
    WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};
use crate::gui::algebraic_notation::Coordinate;
use crate::gui::algebraic_notation::{Column, Row};

pub fn parse(fen: &str) -> HashMap<Coordinate, Piece> {
    let mut pieces: HashMap<Coordinate, Piece> = HashMap::new();

    let lines: Vec<_> = fen.split('/').collect();
    for (row_index, line) in lines.iter().rev().enumerate() {
        let mut column_index = 1;
        for c in line.chars() {
            match c.to_digit(10) {
                Some(shift) => column_index = column_index + shift,
                None => {
                    insert_piece(row_index as u32, column_index, c, &mut pieces);
                    column_index = column_index + 1;
                }
            }
        }
    }
    pieces
}

fn insert_piece(
    row_index: u32,
    column_index: u32,
    c: char,
    pieces: &mut HashMap<Coordinate, Piece>,
) {
    let piece = get_piece(c);
    let row = Row::from(row_index + 1);
    let column = Column::from(column_index);
    if let Some(piece) = piece {
        if let Some(row) = row {
            if let Some(column) = column {
                pieces.insert(Coordinate { row, column }, piece);
            }
        }
    }
}

fn get_piece(c: char) -> Option<Piece> {
    match c {
        'K' => Some(WHITE_KING),
        'Q' => Some(WHITE_QUEEN),
        'R' => Some(WHITE_ROOK),
        'P' => Some(WHITE_PAWN),
        'N' => Some(WHITE_KNIGHT),
        'B' => Some(WHITE_BISHOP),
        'k' => Some(BLACK_KING),
        'q' => Some(BLACK_QUEEN),
        'r' => Some(BLACK_ROOK),
        'p' => Some(BLACK_PAWN),
        'n' => Some(BLACK_KNIGHT),
        'b' => Some(BLACK_BISHOP),
        _ => None,
    }
}
