use crate::engine::{
    chess_move::CastlingType,
    movegen::{Square, *},
    piece::{Color, *},
    position::CastlingRights,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct CastlingConfiguration {
    pub(crate) color: Color,
    pub(crate) king: Piece,
    pub(crate) rook: Piece,

    pub(crate) king_from: Square,
    pub(crate) king_to: Square,

    pub(crate) rook_from: Square,
    pub(crate) rook_to: Square,

    pub(crate) empty_path_squares: &'static [Square],
    pub(crate) king_path_squares: &'static [Square],

    pub(crate) castling_rights: CastlingRights,
    pub(crate) castling_type: CastlingType,
}

pub const WHITE_KINGSIDE: CastlingConfiguration = CastlingConfiguration {
    color: Color::White,
    king: WHITE_KING,
    rook: WHITE_ROOK,
    king_from: E1,
    king_to: G1,
    rook_from: H1,
    rook_to: F1,

    empty_path_squares: &[F1, G1],
    king_path_squares: &[F1, G1],

    castling_rights: CastlingRights::WHITE_KINGSIDE,
    castling_type: CastlingType::WhiteKingside,
};

pub const WHITE_QUEENSIDE: CastlingConfiguration = CastlingConfiguration {
    color: Color::White,
    king: WHITE_KING,
    rook: WHITE_ROOK,
    king_from: E1,
    king_to: C1,
    rook_from: A1,
    rook_to: D1,

    empty_path_squares: &[B1, C1, D1],
    king_path_squares: &[D1, C1],

    castling_rights: CastlingRights::WHITE_QUEENSIDE,
    castling_type: CastlingType::WhiteQueenside,
};

pub const BLACK_KINGSIDE: CastlingConfiguration = CastlingConfiguration {
    color: Color::Black,
    king: BLACK_KING,
    rook: BLACK_ROOK,
    king_from: E8,
    king_to: G8,
    rook_from: H8,
    rook_to: F8,

    empty_path_squares: &[F8, G8],
    king_path_squares: &[F8, G8],

    castling_rights: CastlingRights::BLACK_KINGSIDE,
    castling_type: CastlingType::BlackKingside,
};
pub const BLACK_QUEENSIDE: CastlingConfiguration = CastlingConfiguration {
    color: Color::Black,
    king: BLACK_KING,
    rook: BLACK_ROOK,
    king_from: E8,
    king_to: C8,
    rook_from: A8,
    rook_to: D8,

    empty_path_squares: &[B8, C8, D8],
    king_path_squares: &[D8, C8],

    castling_rights: CastlingRights::BLACK_QUEENSIDE,
    castling_type: CastlingType::BlackQueenside,
};
