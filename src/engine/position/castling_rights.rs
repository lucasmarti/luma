use std::ops::{Index, IndexMut};

use strum::EnumCount;
use strum_macros::EnumCount;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CastlingRights {
    pub castling_rights: [bool; CastlingType::COUNT],
}
impl Index<CastlingType> for CastlingRights {
    type Output = bool;

    fn index(&self, castling_type: CastlingType) -> &Self::Output {
        &self.castling_rights[castling_type.idx()]
    }
}
impl IndexMut<CastlingType> for CastlingRights {
    fn index_mut(&mut self, castling_type: CastlingType) -> &mut Self::Output {
        &mut self.castling_rights[castling_type.idx()]
    }
}
impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            castling_rights: [true; CastlingType::COUNT],
        }
    }
}
#[repr(u8)]
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, EnumCount)]
pub enum CastlingType {
    BlackQueenside,
    BlackKingside,
    WhiteQueenside,
    WhiteKingside,
}
impl CastlingType {
    fn idx(&self) -> usize {
        *self as usize
    }
}
