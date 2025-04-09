use std::ops::BitOr;
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
            bitboard = bitboard.set_bit(index);
        }
        bitboard
    }

    fn set_bit(self, index: u32) -> Bitboard {
        if index > 63 {
            panic!("Index out of bound[0..63] found {:?}", index);
        }
        self | Bitboard::from(index)
    }

    pub fn get_inner(&self) -> u64 {
        self.0
    }
}
impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0.bitor(rhs.0))
    }
}

#[cfg(test)]
mod tests;
