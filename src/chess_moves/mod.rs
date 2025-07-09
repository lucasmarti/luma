use crate::piece::Piece;
#[derive(PartialEq)]
pub enum ChessMove {
    Promotion(Promotion),
    Progress(Progress),
    EnPassant(EnPassant),
    BlackKingsideCastle,
    BlackQueensideCastle,
    WhiteKingsideCastle,
    WhiteQueensideCastle,
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

pub struct BlackKingsideCastle;
pub struct BlackQueensideCastle;
pub struct WhiteKingsideCastle;
pub struct WhiteQueensideCastle;

#[cfg(test)]
mod tests;
