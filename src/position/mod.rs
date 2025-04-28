use crate::bitboard::Bitboard;

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
