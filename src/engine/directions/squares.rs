use std::cmp::Ordering;
pub const WHITE_KING_STARTING_POSITION: Square = E1;
pub const BLACK_KING_STARTING_POSITION: Square = E8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(u32);
impl Square {
    pub fn new(index: u32) -> Option<Self> {
        if index < 64 {
            Some(Square(index))
        } else {
            None
        }
    }

    pub const fn new_unchecked(index: u32) -> Self {
        Square(index)
    }
    pub const fn as_index(self) -> u32 {
        self.0
    }
}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Square {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
#[allow(unused_variables)]
macro_rules! define_squares {
    ($($name:ident = $value:expr),* $(,)?) => {
            $(#[allow(dead_code)]
                pub const $name: Square = Square($value);)*
    };
}

define_squares! {
    A1 = 0, B1 = 1, C1 = 2, D1 = 3, E1 = 4, F1 = 5, G1 = 6, H1 = 7,
    A2 = 8, B2 = 9, C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
}
