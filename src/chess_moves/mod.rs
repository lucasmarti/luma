use crate::{check::is_check, directions::*, piece::*, position::Position};
pub fn progess(position: &Position, piece: Piece, from: u32, to: u32) -> Position {
    position
        .remove_piece(from)
        .remove_piece(to)
        .put_piece(piece, to)
}

pub fn en_passant(position: &Position, piece: Piece, from: u32, to: u32, capture: u32) -> Position {
    position
        .remove_piece(from)
        .remove_piece(capture)
        .remove_piece(to)
        .put_piece(piece, to)
}

pub fn promote(position: &Position, from: u32, to: u32, new_piece: Piece) -> Position {
    position
        .remove_piece(from)
        .remove_piece(to)
        .put_piece(new_piece, to)
}

pub fn white_queenside_castle(position: &Position) -> Position {
    position
        .remove_piece(E1)
        .put_piece(WHITE_KING, C1)
        .remove_piece(A1)
        .put_piece(WHITE_ROOK, D1)
}

pub fn white_kingside_castle(position: &Position) -> Position {
    position
        .remove_piece(E1)
        .put_piece(WHITE_KING, G1)
        .remove_piece(H1)
        .put_piece(WHITE_ROOK, F1)
}

pub fn black_queenside_castle(position: &Position) -> Position {
    position
        .remove_piece(E8)
        .put_piece(BLACK_KING, C8)
        .remove_piece(A8)
        .put_piece(BLACK_ROOK, D8)
}
pub fn black_kingside_castle(position: &Position) -> Position {
    position
        .remove_piece(E8)
        .put_piece(BLACK_KING, G8)
        .remove_piece(H8)
        .put_piece(BLACK_ROOK, F8)
}

#[cfg(test)]
mod tests;
