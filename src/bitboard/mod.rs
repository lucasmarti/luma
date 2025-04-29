use intbits::Bits;
use iterator::BitboardIterator;
use std::ops::{BitAnd, BitOr};

#[derive(Clone, Copy, Debug)]

pub struct Bitboard(u64);
impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard { 0: 0 }
    }
    pub fn from(index: u32) -> Bitboard {
        if index > 63 {
            panic!("Index out of bound[0..63] found {:?}", index);
        }
        Bitboard { 0: 1 << index }
    }
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn from_vec(vec: Vec<u32>) -> Bitboard {
        let mut bitboard = Bitboard { 0: 0 };
        for index in vec {
            bitboard.set_bit(index);
        }
        bitboard
    }
    pub fn bit(self, index: u32) -> bool {
        self.0.bit(index)
    }

    pub fn set_bit(&mut self, index: u32) {
        self.0.set_bit(index, true);
    }

    pub fn remove_bit(&mut self, index: u32) {
        self.0.set_bit(index, false);
    }

    pub fn get_inner(&self) -> u64 {
        self.0
    }

    pub fn iter(&self) -> BitboardIterator {
        BitboardIterator {
            bitboard: self,
            index: 0,
        }
    }
    pub fn contains(&self, index: u32) -> bool {
        if index > 63 {
            return false;
        }
        let bitboard = Bitboard::from(index);
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
impl Default for Bitboard {
    fn default() -> Self {
        Self(Default::default())
    }
}

mod iterator;
#[cfg(test)]
mod tests;
