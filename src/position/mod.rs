use crate::bitboard::Bitboard;

pub struct Position {
    pub white_king: Bitboard,
    pub white_queen: Bitboard,
    pub white_rooks: Bitboard,
    pub white_bishops: Bitboard,
    pub white_knights: Bitboard,
    pub white_pawns: Bitboard,
    pub white_kingside_castel_is_possible: bool,
    pub white_queenside_castel_is_possible: bool,
    pub black_king: Bitboard,
    pub black_queen: Bitboard,
    pub black_rooks: Bitboard,
    pub black_bishops: Bitboard,
    pub black_knights: Bitboard,
    pub black_pawns: Bitboard,
    pub black_kingside_castel_is_possible: bool,
    pub black_queenside_castel_is_possible: bool,

    pub current_player: Color,
}

pub enum Color {
    White,
    Black,
}
