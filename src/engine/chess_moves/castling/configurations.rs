use crate::engine::{
    directions::squares::*,
    piece::{Color, Piece, BLACK_KING, BLACK_ROOK, WHITE_KING, WHITE_ROOK},
    position::CastlingType,
};
pub struct CastlingConfiguration {
    pub(crate) color: Color,
    pub(crate) king: Piece,
    pub(crate) rook: Piece,
    pub(crate) king_from: u32,
    pub(crate) king_to: u32,
    pub(crate) rook_from: u32,
    pub(crate) rook_to: u32,
    pub(crate) empty_path_squares: &'static [u32],
    pub(crate) castling_type: CastlingType,
}

// White Kingside Castling
pub const WHITE_KINGSIDE: CastlingConfiguration = CastlingConfiguration {
    color: Color::White,
    king_from: E1,
    king_to: G1,
    rook_from: H1,
    rook_to: F1,
    empty_path_squares: &[F1, G1],
    king: WHITE_KING,
    rook: WHITE_ROOK,
    castling_type: CastlingType::WhiteKingside,
};

// White Queenside Castling
pub const WHITE_QUEENSIDE: CastlingConfiguration = CastlingConfiguration {
    color: Color::White,
    king_from: E1,
    king_to: C1,
    rook_from: A1,
    rook_to: D1,
    empty_path_squares: &[B1, C1, D1],
    king: WHITE_KING,
    rook: WHITE_ROOK,
    castling_type: CastlingType::WhiteQueenside,
};

// Black Kingside Castling
pub const BLACK_KINGSIDE: CastlingConfiguration = CastlingConfiguration {
    color: Color::Black,
    king_from: E8,
    king_to: G8,
    rook_from: H8,
    rook_to: F8,
    empty_path_squares: &[F8, G8],
    king: BLACK_KING,
    rook: BLACK_ROOK,
    castling_type: CastlingType::BlackKingside,
};

// Black Queenside Castling
pub const BLACK_QUEENSIDE: CastlingConfiguration = CastlingConfiguration {
    color: Color::Black,
    king_from: E8,
    king_to: C8,
    rook_from: A8,
    rook_to: D8,
    empty_path_squares: &[B8, C8, D8],
    king: BLACK_KING,
    rook: BLACK_ROOK,
    castling_type: CastlingType::BlackQueenside,
};
