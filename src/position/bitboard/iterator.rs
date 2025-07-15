use super::Bitboard;
use intbits::Bits;

pub struct BitboardIterator<'a> {
    pub(crate) bitboard: &'a Bitboard,
    pub(crate) index: u32,
}
impl<'a> Iterator for BitboardIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 64 {
            if self.bitboard.0.bit(self.index) {
                let result = Some(self.index);
                self.index += 1;
                return result;
            } else {
                self.index += 1;
            }
        }
        None
    }
}
