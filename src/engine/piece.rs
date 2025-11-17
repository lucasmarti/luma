#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub enum Piece {
    BlackQueen,
    BlackKing,
    BlackBishop,
    BlackRook,
    BlackPawn,
    BlackKnight,
    WhiteQueen,
    WhiteKing,
    WhiteBishop,
    WhiteRook,
    WhitePawn,
    WhiteKnight,
}
impl Piece {
    pub fn get_color(&self) -> Color {
        match self {
            Piece::BlackQueen => Color::Black,
            Piece::BlackKing => Color::Black,
            Piece::BlackBishop => Color::Black,
            Piece::BlackRook => Color::Black,
            Piece::BlackPawn => Color::Black,
            Piece::BlackKnight => Color::Black,
            Piece::WhiteQueen => Color::White,
            Piece::WhiteKing => Color::White,
            Piece::WhiteBishop => Color::White,
            Piece::WhiteRook => Color::White,
            Piece::WhitePawn => Color::White,
            Piece::WhiteKnight => Color::White,
        }
    }
    pub fn get_type(&self) -> Typ {
        match self {
            Piece::BlackQueen => Typ::Queen,
            Piece::BlackKing => Typ::King,
            Piece::BlackBishop => Typ::Bishop,
            Piece::BlackRook => Typ::Rook,
            Piece::BlackPawn => Typ::Pawn,
            Piece::BlackKnight => Typ::Knight,
            Piece::WhiteQueen => Typ::Queen,
            Piece::WhiteKing => Typ::King,
            Piece::WhiteBishop => Typ::Bishop,
            Piece::WhiteRook => Typ::Rook,
            Piece::WhitePawn => Typ::Pawn,
            Piece::WhiteKnight => Typ::Knight,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Typ {
    King,
    Queen,
    Rook,
    Pawn,
    Knight,
    Bishop,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn get_opponent_color(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
