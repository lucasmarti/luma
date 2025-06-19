use crate::piece::Piece;

pub enum ChessMove {
    Promotion(Promotion),
    Progress(Progress),
    BlackKingsideCastle,
    BlackQueensideCastle,
    WhiteKingsideCastle,
    WhiteQueensideCastle,
}
pub struct Promotion {
    pub piece: Piece,
    pub from: u32,
    pub to: u32,
    pub new_piece: Piece,
}

pub struct Progress {
    pub piece: Piece,
    pub from: u32,
    pub to: u32,
}

pub struct BlackKingsideCastle;
pub struct BlackQueensideCastle;
pub struct WhiteKingsideCastle;
pub struct WhiteQueensideCastle;

#[cfg(test)]
mod tests;
