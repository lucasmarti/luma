use intbits::Bits;
use iterator::BitboardIterator;
use std::ops::{BitAnd, BitOr};

use crate::engine::directions::squares::Square;

#[derive(Clone, Copy, Debug, PartialEq, Default)]

pub struct Bitboard(u64);
impl Bitboard {
    pub fn from(square: Square) -> Bitboard {
        let index = square.as_index();
        if index > 63 {
            panic!("Index out of bound[0..63] found {:?}", index);
        }
        Bitboard(1 << index)
    }
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn from_vec(vec: Vec<Square>) -> Bitboard {
        let mut bitboard = Bitboard(0);
        for square in vec {
            bitboard.set_bit(square);
        }
        bitboard
    }

    pub fn set_bit(&mut self, square: Square) {
        self.0.set_bit(square.as_index(), true);
    }

    pub fn remove_bit(&mut self, square: Square) {
        self.0.set_bit(square.as_index(), false);
    }

    pub fn get_inner(&self) -> u64 {
        self.0
    }

    pub fn iter(&self) -> BitboardIterator<'_> {
        BitboardIterator {
            bitboard: self,
            index: 0,
        }
    }
    pub fn contains(&self, square: Square) -> bool {
        let index = square.as_index();
        if index > 63 {
            return false;
        }
        let bitboard = Bitboard::from(square);
        let intersection = *self & bitboard;
        intersection.count_ones() > 0
    }
}
impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0.bitor(rhs.0))
    }
}
impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0.bitand(rhs.0))
    }
}

mod iterator;
#[cfg(test)]
mod tests;
