use std::ops::BitOr;

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
}
impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0.bitor(rhs.0))
    }
}

#[cfg(test)]
mod tests;
