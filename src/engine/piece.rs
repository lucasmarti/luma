use strum_macros::EnumCount;
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Piece {
    pub color: Color,
    pub typ: Typ,
}
pub const BLACK_KING: Piece = Piece::new(Color::Black, Typ::King);
pub const BLACK_QUEEN: Piece = Piece::new(Color::Black, Typ::Queen);
pub const BLACK_ROOK: Piece = Piece::new(Color::Black, Typ::Rook);
pub const BLACK_BISHOP: Piece = Piece::new(Color::Black, Typ::Bishop);
pub const BLACK_KNIGHT: Piece = Piece::new(Color::Black, Typ::Knight);
pub const BLACK_PAWN: Piece = Piece::new(Color::Black, Typ::Pawn);

pub const WHITE_KING: Piece = Piece::new(Color::White, Typ::King);
pub const WHITE_QUEEN: Piece = Piece::new(Color::White, Typ::Queen);
pub const WHITE_ROOK: Piece = Piece::new(Color::White, Typ::Rook);
pub const WHITE_BISHOP: Piece = Piece::new(Color::White, Typ::Bishop);
pub const WHITE_KNIGHT: Piece = Piece::new(Color::White, Typ::Knight);
pub const WHITE_PAWN: Piece = Piece::new(Color::White, Typ::Pawn);

impl Piece {
    pub const fn new(color: Color, typ: Typ) -> Self {
        Self { color, typ }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumCount, Hash, EnumIter)]
pub enum Typ {
    King,
    Queen,
    Rook,
    Pawn,
    Knight,
    Bishop,
}

impl Typ {
    pub fn idx(self) -> usize {
        self as usize
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, EnumCount, EnumIter)]
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
    pub fn idx(self) -> usize {
        self as usize
    }
}
