use crate::{
    directions::*,
    piece::{Color, Piece, Typ, BLACK_KING, BLACK_ROOK, WHITE_KING, WHITE_ROOK},
    position::{self, bitboard::Bitboard},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    white_king: Bitboard,
    white_queen: Bitboard,
    white_rooks: Bitboard,
    white_bishops: Bitboard,
    white_knights: Bitboard,
    white_pawns: Bitboard,
    white_kingside_castle_allowed: bool,
    white_queenside_castle_allowed: bool,
    black_king: Bitboard,
    black_queen: Bitboard,
    black_rooks: Bitboard,
    black_bishops: Bitboard,
    black_knights: Bitboard,
    black_pawns: Bitboard,
    black_kingside_castle_allowed: bool,
    black_queenside_castle_allowed: bool,
    pub en_passant: Option<u32>,
}
impl Position {
    pub fn new_starting_position() -> Position {
        Position {
            white_king: Bitboard::from(E1),
            white_queen: Bitboard::from(D1),
            white_rooks: Bitboard::from_vec(vec![A1, H1]),
            white_bishops: Bitboard::from_vec(vec![C1, F1]),
            white_knights: Bitboard::from_vec(vec![B1, G1]),
            white_pawns: Bitboard::from_vec(vec![A2, B2, C2, D2, E2, F2, G2, H2]),
            black_king: Bitboard::from(E8),
            black_queen: Bitboard::from(D8),
            black_rooks: Bitboard::from_vec(vec![A8, H8]),
            black_bishops: Bitboard::from_vec(vec![C8, F8]),
            black_knights: Bitboard::from_vec(vec![B8, G8]),
            black_pawns: Bitboard::from_vec(vec![A7, B7, C7, D7, E7, F7, G7, H7]),
            ..Default::default()
        }
    }
    pub fn is_white_kingside_castle_allowed(&self) -> bool {
        self.white_kingside_castle_allowed
    }
    pub fn is_white_queenside_castle_allowed(&self) -> bool {
        self.white_queenside_castle_allowed
    }
    pub fn is_black_kingside_castle_allowed(&self) -> bool {
        self.black_kingside_castle_allowed
    }
    pub fn is_black_queenside_castle_allowed(&self) -> bool {
        self.black_queenside_castle_allowed
    }

    pub fn disallow_white_kingside_castle(mut self) -> Position {
        self.white_kingside_castle_allowed = false;
        self
    }

    pub fn disallow_white_queenside_castle(mut self) -> Position {
        self.white_queenside_castle_allowed = false;
        self
    }
    pub fn disallow_black_kingside_castle(mut self) -> Position {
        self.black_kingside_castle_allowed = false;
        self
    }

    pub fn disallow_black_queenside_castle(mut self) -> Position {
        self.black_queenside_castle_allowed = false;
        self
    }
    pub fn is_occupied(&self, index: u32) -> bool {
        self.get_all().contains(index)
    }

    pub fn is_occupied_by_color(&self, index: u32, color: Color) -> bool {
        match color {
            Color::Black => self.get_black().contains(index),
            Color::White => self.get_white().contains(index),
        }
    }
    pub fn is_occupied_by_piece(&self, index: u32, piece: Piece) -> bool {
        self.get_squares(piece).contains(index)
    }

    pub fn count_pieces(&self, piece: Piece) -> u32 {
        self.get_squares(piece).count_ones()
    }

    pub fn get_king_square(&self, color: Color) -> u32 {
        for square in self
            .get_squares(Piece {
                typ: Typ::King,
                color: color,
            })
            .iter()
        {
            return square;
        }
        panic!("No King found");
    }

    fn get_black(&self) -> Bitboard {
        self.black_king
            | self.black_queen
            | self.black_rooks
            | self.black_knights
            | self.black_pawns
            | self.black_bishops
    }

    fn get_white(&self) -> Bitboard {
        self.white_king
            | self.white_queen
            | self.white_rooks
            | self.white_knights
            | self.white_pawns
            | self.white_bishops
    }

    fn get_all(&self) -> Bitboard {
        self.get_black() | self.get_white()
    }

    pub fn get_squares(&self, piece: Piece) -> Bitboard {
        match piece.color {
            Color::Black => match piece.typ {
                Typ::King => self.black_king,
                Typ::Queen => self.black_queen,
                Typ::Rook => self.black_rooks,
                Typ::Pawn => self.black_pawns,
                Typ::Knight => self.black_knights,
                Typ::Bishop => self.black_bishops,
            },
            Color::White => match piece.typ {
                Typ::King => self.white_king,
                Typ::Queen => self.white_queen,
                Typ::Rook => self.white_rooks,
                Typ::Pawn => self.white_pawns,
                Typ::Knight => self.white_knights,
                Typ::Bishop => self.white_bishops,
            },
        }
    }

    pub fn put_piece(mut self, piece: Piece, index: u32) -> Position {
        match piece.color {
            Color::Black => match piece.typ {
                Typ::King => self.black_king.set_bit(index),
                Typ::Queen => self.black_queen.set_bit(index),
                Typ::Rook => self.black_rooks.set_bit(index),
                Typ::Pawn => self.black_pawns.set_bit(index),
                Typ::Knight => self.black_knights.set_bit(index),
                Typ::Bishop => self.black_bishops.set_bit(index),
            },
            Color::White => match piece.typ {
                Typ::King => self.white_king.set_bit(index),
                Typ::Queen => self.white_queen.set_bit(index),
                Typ::Rook => self.white_rooks.set_bit(index),
                Typ::Pawn => self.white_pawns.set_bit(index),
                Typ::Knight => self.white_knights.set_bit(index),
                Typ::Bishop => self.white_bishops.set_bit(index),
            },
        }
        self
    }

    pub fn remove_piece(mut self, index: u32) -> Position {
        self.black_king.remove_bit(index);
        self.black_queen.remove_bit(index);
        self.black_rooks.remove_bit(index);
        self.black_pawns.remove_bit(index);
        self.black_knights.remove_bit(index);
        self.black_bishops.remove_bit(index);
        self.white_king.remove_bit(index);
        self.white_queen.remove_bit(index);
        self.white_rooks.remove_bit(index);
        self.white_pawns.remove_bit(index);
        self.white_knights.remove_bit(index);
        self.white_bishops.remove_bit(index);
        self
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            white_king: Default::default(),
            white_queen: Default::default(),
            white_rooks: Default::default(),
            white_bishops: Default::default(),
            white_knights: Default::default(),
            white_pawns: Default::default(),
            black_king: Default::default(),
            black_queen: Default::default(),
            black_rooks: Default::default(),
            black_bishops: Default::default(),
            black_knights: Default::default(),
            black_pawns: Default::default(),
            white_kingside_castle_allowed: true,
            white_queenside_castle_allowed: true,
            black_kingside_castle_allowed: true,
            black_queenside_castle_allowed: true,
            en_passant: None,
        }
    }
}
mod bitboard;
pub mod print;

#[cfg(test)]
mod tests;
