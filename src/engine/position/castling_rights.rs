use std::ops::BitOr;

use strum_macros::EnumCount;

use crate::engine::movegen::{A1, A8, E1, E8, H1, H8};
use crate::engine::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const BLACK_KINGSIDE: Self = Self(1 << 0);
    pub const BLACK_QUEENSIDE: Self = Self(1 << 1);
    pub const WHITE_KINGSIDE: Self = Self(1 << 2);
    pub const WHITE_QUEENSIDE: Self = Self(1 << 3);
    pub const NONE: Self = Self(0);
    pub fn from(square: Square) -> Self {
        match square {
            A1 => CastlingRights::WHITE_QUEENSIDE,
            E1 => CastlingRights::WHITE_QUEENSIDE | CastlingRights::WHITE_KINGSIDE,
            H1 => CastlingRights::WHITE_KINGSIDE,
            A8 => CastlingRights::BLACK_QUEENSIDE,
            E8 => CastlingRights::BLACK_QUEENSIDE | CastlingRights::BLACK_KINGSIDE,
            H8 => CastlingRights::BLACK_KINGSIDE,
            _ => CastlingRights::NONE,
        }
    }
    pub fn all() -> Self {
        Self::BLACK_KINGSIDE | Self::BLACK_QUEENSIDE | Self::WHITE_KINGSIDE | Self::WHITE_QUEENSIDE
    }

    pub fn add(&mut self, rights: Self) {
        self.0 |= rights.0;
    }

    pub fn remove(&mut self, rights: Self) {
        self.0 &= !rights.0;
    }

    pub fn has(&self, rights: Self) -> bool {
        (self.0 & rights.0) == rights.0
    }
}

impl BitOr for CastlingRights {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(test)]
mod tests;
