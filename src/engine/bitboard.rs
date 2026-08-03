use intbits::Bits;
use iterator::BitboardIterator;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use crate::engine::movegen::Square;
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]

pub struct Bitboard(u64);
impl Bitboard {
    pub fn from(square: Square) -> Bitboard {
        let index = square.as_index();
        if index > 63 {
            panic!("Index out of bound[0..63] found {:?}", index);
        }
        Bitboard(1 << index)
    }
    pub const fn new(bits: u64) -> Bitboard {
        Self(bits)
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
    #[allow(dead_code)]
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
    pub const fn intersects(self, other: Bitboard) -> bool {
        (self.0 & other.0) != 0
    }
}
impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}
impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

mod iterator;
#[cfg(test)]
mod tests;
