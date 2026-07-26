use strum::EnumCount;
use strum_macros::EnumCount;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
    pub const fn new(color: Color, typ: Typ) -> Self {
        match (color, typ) {
            (Color::Black, Typ::King) => Piece::BlackKing,
            (Color::Black, Typ::Queen) => Piece::BlackQueen,
            (Color::Black, Typ::Rook) => Piece::BlackRook,
            (Color::Black, Typ::Bishop) => Piece::BlackBishop,
            (Color::Black, Typ::Knight) => Piece::BlackKnight,
            (Color::Black, Typ::Pawn) => Piece::BlackPawn,

            (Color::White, Typ::King) => Piece::WhiteKing,
            (Color::White, Typ::Queen) => Piece::WhiteQueen,
            (Color::White, Typ::Rook) => Piece::WhiteRook,
            (Color::White, Typ::Bishop) => Piece::WhiteBishop,
            (Color::White, Typ::Knight) => Piece::WhiteKnight,
            (Color::White, Typ::Pawn) => Piece::WhitePawn,
        }
    }
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

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumCount)]
pub enum Typ {
    King,
    Queen,
    Rook,
    Pawn,
    Knight,
    Bishop,
}

impl Typ {
    pub const ALL: [Typ; Typ::COUNT] = [
        Typ::King,
        Typ::Queen,
        Typ::Rook,
        Typ::Pawn,
        Typ::Knight,
        Typ::Bishop,
    ];
    pub fn idx(self) -> usize {
        self as usize
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, EnumCount)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub const ALL: [Color; Color::COUNT] = [Color::Black, Color::White];

    pub fn get_opponent_color(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
    pub fn idx(self) -> usize {
        self as usize
    }
}
