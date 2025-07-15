use crate::{check::is_check, directions::*, piece::*, position::Position};
#[derive(PartialEq)]
pub enum ChessMove {
    Promotion(Promotion),
    Progress(Progress),
    EnPassant(EnPassant),
    BlackKingsideCastle(BlackKingsideCastle),
    BlackQueensideCastle(BlackQueensideCastle),
    WhiteKingsideCastle(WhiteKingsideCastle),
    WhiteQueensideCastle(WhiteQueensideCastle),
}
#[derive(PartialEq)]
pub struct Promotion {
    pub piece: Piece,
    pub from: u32,
    pub to: u32,
    pub new_piece: Piece,
}

#[derive(PartialEq)]
pub struct Progress {
    pub piece: Piece,
    pub from: u32,
    pub to: u32,
}

#[derive(PartialEq)]
pub struct EnPassant {
    pub piece: Piece,
    pub from: u32,
    pub to: u32,
    pub capture: u32,
}

#[derive(PartialEq)]
pub struct BlackKingsideCastle;
#[derive(PartialEq)]

pub struct BlackQueensideCastle;

#[derive(PartialEq)]
pub struct WhiteKingsideCastle;

#[derive(PartialEq)]
pub struct WhiteQueensideCastle;

pub fn move_piece(position: &Position, possible_moves: ChessMove) -> Position {
    match possible_moves {
        ChessMove::Progress(progress) => progress.execute(position),
        ChessMove::Promotion(promotion) => promotion.execute(position),
        ChessMove::EnPassant(en_passant) => en_passant.execute(position),
        ChessMove::BlackKingsideCastle(black_kingside_castle) => {
            black_kingside_castle.execute(position)
        }
        ChessMove::BlackQueensideCastle(black_queenside_castle) => {
            black_queenside_castle.execute(position)
        }
        ChessMove::WhiteKingsideCastle(white_kingside_castle) => {
            white_kingside_castle.execute(position)
        }
        ChessMove::WhiteQueensideCastle(white_queenside_castle) => {
            white_queenside_castle.execute(position)
        }
    }
}

pub fn progess(position: &Position, piece: Piece, from: u32, to: u32) -> Option<Position> {
    let new_position = position
        .remove_piece(from)
        .remove_piece(to)
        .put_piece(piece, to);

    if is_check(&new_position, piece.color) {
        None
    } else {
        Some(new_position)
    }
}

impl Progress {
    pub fn execute(self, position: &Position) -> Position {
        position
            .remove_piece(self.from)
            .remove_piece(self.to)
            .put_piece(self.piece, self.to)
    }
}

impl Promotion {
    pub fn execute(self, position: &Position) -> Position {
        position
            .remove_piece(self.from)
            .remove_piece(self.to)
            .put_piece(self.new_piece, self.to)
    }
}
impl EnPassant {
    fn execute(self, position: &Position) -> Position {
        position
            .remove_piece(self.from)
            .remove_piece(self.capture)
            .remove_piece(self.to)
            .put_piece(self.piece, self.to)
    }
}
impl WhiteQueensideCastle {
    pub fn execute(self, position: &Position) -> Position {
        position
            .remove_piece(E1)
            .put_piece(WHITE_KING, C1)
            .remove_piece(A1)
            .put_piece(WHITE_ROOK, D1)
    }
}
impl WhiteKingsideCastle {
    pub fn execute(self, position: &Position) -> Position {
        position
            .remove_piece(E1)
            .put_piece(WHITE_KING, G1)
            .remove_piece(H1)
            .put_piece(WHITE_ROOK, F1)
    }
}
impl BlackQueensideCastle {
    pub fn execute(self, position: &Position) -> Position {
        position
            .remove_piece(E8)
            .put_piece(BLACK_KING, C8)
            .remove_piece(A8)
            .put_piece(BLACK_ROOK, D8)
    }
}
impl BlackKingsideCastle {
    pub fn execute(self, position: &Position) -> Position {
        position
            .remove_piece(E8)
            .put_piece(BLACK_KING, G8)
            .remove_piece(H8)
            .put_piece(BLACK_ROOK, F8)
    }
}

#[cfg(test)]
mod tests;
