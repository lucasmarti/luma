use std::ops::{Index, IndexMut};

use strum::EnumCount;

use crate::engine::{bitboard::Bitboard, piece::Color};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct Occupancy {
    pub occupied: [Bitboard; Color::COUNT],
}

impl Index<Color> for Occupancy {
    type Output = Bitboard;

    fn index(&self, color: Color) -> &Self::Output {
        &self.occupied[color.idx()]
    }
}
impl IndexMut<Color> for Occupancy {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        &mut self.occupied[color.idx()]
    }
}
