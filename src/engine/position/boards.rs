use std::ops::{Index, IndexMut};

use strum::EnumCount;

use crate::engine::{
    bitboard::Bitboard,
    piece::{Color, Typ},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct Boards {
    pub boards: [[Bitboard; Typ::COUNT]; Color::COUNT],
}

impl Index<(Color, Typ)> for Boards {
    type Output = Bitboard;

    fn index(&self, (color, typ): (Color, Typ)) -> &Self::Output {
        &self.boards[color.idx()][typ.idx()]
    }
}
impl IndexMut<(Color, Typ)> for Boards {
    fn index_mut(&mut self, (color, typ): (Color, Typ)) -> &mut Self::Output {
        &mut self.boards[color.idx()][typ.idx()]
    }
}
