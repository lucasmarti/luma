use crate::{
    bitboard::Bitboard,
    piece::{self, Color, Piece, Type},
};
#[derive(Clone, Copy)]
pub struct Position {
    pub white_king: Bitboard,
    pub white_queen: Bitboard,
    pub white_rooks: Bitboard,
    pub white_bishops: Bitboard,
    pub white_knights: Bitboard,
    pub white_pawns: Bitboard,
    pub black_king: Bitboard,
    pub black_queen: Bitboard,
    pub black_rooks: Bitboard,
    pub black_bishops: Bitboard,
    pub black_knights: Bitboard,
    pub black_pawns: Bitboard,
}
impl Position {
    pub fn new_starting_position() -> Position {
        Position {
            white_king: Bitboard::from(4),
            white_queen: Bitboard::from(3),
            white_rooks: Bitboard::from_vec(vec![0, 7]),
            white_bishops: Bitboard::from_vec(vec![2, 5]),
            white_knights: Bitboard::from_vec(vec![1, 6]),
            white_pawns: Bitboard::from_vec(vec![8, 9, 10, 11, 12, 13, 14, 15]),
            black_king: Bitboard::from(60),
            black_queen: Bitboard::from(59),
            black_rooks: Bitboard::from_vec(vec![56, 63]),
            black_bishops: Bitboard::from_vec(vec![58, 61]),
            black_knights: Bitboard::from_vec(vec![57, 62]),
            black_pawns: Bitboard::from_vec(vec![48, 49, 50, 51, 52, 53, 54, 55]),
        }
    }

    pub fn get_black(&self) -> Bitboard {
        self.black_king
            | self.black_queen
            | self.black_rooks
            | self.black_knights
            | self.black_pawns
            | self.black_bishops
    }

    pub fn get_white(&self) -> Bitboard {
        self.white_king
            | self.white_queen
            | self.white_rooks
            | self.white_knights
            | self.white_pawns
            | self.white_bishops
    }

    pub fn get_all(&self) -> Bitboard {
        self.get_black() | self.get_white()
    }

    pub fn move_piece(mut self, piece: Piece, from_index: u32, to_index: u32) -> Position {
        self.remove_piece(from_index)
            .remove_piece(to_index)
            .put_piece(piece, to_index)
    }

    fn put_piece(mut self, piece: Piece, index: u32) -> Position {
        match piece.color {
            Color::BLACK => match piece.type_ {
                Type::KING => self.black_king.set_bit(index),
                Type::QUEEN => self.black_queen.set_bit(index),
                Type::ROOK => self.black_rooks.set_bit(index),
                Type::PAWN => self.black_pawns.set_bit(index),
                Type::KNIGHT => self.black_knights.set_bit(index),
                Type::BISHOP => self.black_bishops.set_bit(index),
            },
            Color::WHITE => match piece.type_ {
                Type::KING => self.white_king.set_bit(index),
                Type::QUEEN => self.white_queen.set_bit(index),
                Type::ROOK => self.white_rooks.set_bit(index),
                Type::PAWN => self.white_pawns.set_bit(index),
                Type::KNIGHT => self.white_knights.set_bit(index),
                Type::BISHOP => self.white_bishops.set_bit(index),
            },
        }
        self
    }

    fn remove_piece(mut self, index: u32) -> Position {
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
        }
    }
}
mod print;

#[cfg(test)]
mod tests;
