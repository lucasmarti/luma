#[derive(Clone, Copy, PartialEq)]

pub struct Piece {
    pub typ: Typ,
    pub color: Color,
}
#[derive(Clone, Copy, PartialEq)]
pub enum Typ {
    King,
    Queen,
    Rook,
    Pawn,
    Knight,
    Bishop,
}
#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

pub const BLACK_KING: Piece = Piece {
    color: Color::Black,
    typ: Typ::King,
};

pub const BLACK_QUEEN: Piece = Piece {
    color: Color::Black,
    typ: Typ::Queen,
};

pub const BLACK_KNIGHT: Piece = Piece {
    color: Color::Black,
    typ: Typ::Knight,
};
pub const BLACK_ROOK: Piece = Piece {
    color: Color::Black,
    typ: Typ::Rook,
};

pub const BLACK_PAWN: Piece = Piece {
    color: Color::Black,
    typ: Typ::Pawn,
};

pub const BLACK_BISHOP: Piece = Piece {
    color: Color::Black,
    typ: Typ::Bishop,
};

pub const WHITE_KING: Piece = Piece {
    color: Color::White,
    typ: Typ::King,
};

pub const WHITE_QUEEN: Piece = Piece {
    color: Color::White,
    typ: Typ::Queen,
};

pub const WHITE_KNIGHT: Piece = Piece {
    color: Color::White,
    typ: Typ::Knight,
};
pub const WHITE_ROOK: Piece = Piece {
    color: Color::White,
    typ: Typ::Rook,
};

pub const WHITE_PAWN: Piece = Piece {
    color: Color::White,
    typ: Typ::Pawn,
};

pub const WHITE_BISHOP: Piece = Piece {
    color: Color::White,
    typ: Typ::Bishop,
};
