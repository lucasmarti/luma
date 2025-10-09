/*
Index Positions
-----------------------
56 57 58 59 60 61 62 63
48 49 50 51 52 53 54 55
40 41 42 43 44 45 46 47
32 33 34 35 36 37 38 39
24 25 26 27 28 29 30 31
16 17 18 19 20 21 22 23
08 09 10 11 12 13 14 15
00 01 02 03 04 05 06 07

Directions
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ A _ B _ _ _
_ H 1 2 3 C _ _
_ _ 4   5 _ _ _
_ G 6 7 8 D _ _
_ _ F _ E _ _ _
_ _ _ _ _ _ _ _

1: up_left
2: up
3: up_right
4: right
5: down_right
6: down
7: down_left
8: left

A: up_up_left
B: up_up_right
C: right_right_up
D: right_right_down
E: down_down_right
F: down_down_left
G: left_left_down
H: left_left_up

*/

use crate::engine::directions::squares::*;

const ONE_ROW: u32 = 8;
const TWO_ROWS: u32 = 16;
const ONE_COLUMN: u32 = 1;
const TWO_COLUMNS: u32 = 2;

pub type DirectionFn = fn(Square) -> Option<Square>;
pub type RowFn = fn(Square) -> bool;

pub fn up(square: Square) -> Option<Square> {
    if !is_in_last_row(square) {
        Square::new(square.as_index() + ONE_ROW)
    } else {
        None
    }
}

pub fn down(square: Square) -> Option<Square> {
    let index = square.as_index();
    if !is_in_first_row(square) {
        Square::new(index - ONE_ROW)
    } else {
        None
    }
}

pub fn left(square: Square) -> Option<Square> {
    if !is_in_first_column(square) {
        Square::new(square.as_index() - ONE_COLUMN)
    } else {
        None
    }
}

pub fn right(square: Square) -> Option<Square> {
    if !is_in_last_column(square) {
        Square::new(square.as_index() + ONE_COLUMN)
    } else {
        None
    }
}

pub fn up_right(square: Square) -> Option<Square> {
    if !is_in_last_row(square) && !is_in_last_column(square) {
        Square::new(square.as_index() + ONE_ROW + ONE_COLUMN)
    } else {
        None
    }
}
pub fn up_left(square: Square) -> Option<Square> {
    if !is_in_last_row(square) && !is_in_first_column(square) {
        Square::new(square.as_index() + ONE_ROW - ONE_COLUMN)
    } else {
        None
    }
}
pub fn down_right(square: Square) -> Option<Square> {
    if !is_in_first_row(square) && !is_in_last_column(square) {
        Square::new(square.as_index() - ONE_ROW + ONE_COLUMN)
    } else {
        None
    }
}
pub fn down_left(square: Square) -> Option<Square> {
    if !is_in_first_row(square) && !is_in_first_column(square) {
        Square::new(square.as_index() - ONE_ROW - ONE_COLUMN)
    } else {
        None
    }
}

pub fn up_up_right(square: Square) -> Option<Square> {
    if !is_in_last_or_second_last_row(square) && !is_in_last_column(square) {
        Square::new(square.as_index() + TWO_ROWS + ONE_COLUMN)
    } else {
        None
    }
}

pub fn up_up_left(square: Square) -> Option<Square> {
    if !is_in_last_or_second_last_row(square) && !is_in_first_column(square) {
        Square::new(square.as_index() + TWO_ROWS - ONE_COLUMN)
    } else {
        None
    }
}

pub fn down_down_right(square: Square) -> Option<Square> {
    if !is_in_first_or_second_row(square) && !is_in_last_column(square) {
        Square::new(square.as_index() - TWO_ROWS + ONE_COLUMN)
    } else {
        None
    }
}

pub fn down_down_left(square: Square) -> Option<Square> {
    if !is_in_first_or_second_row(square) && !is_in_first_column(square) {
        Square::new(square.as_index() - TWO_ROWS - ONE_COLUMN)
    } else {
        None
    }
}

pub fn right_right_up(square: Square) -> Option<Square> {
    if !is_in_last_row(square) && !is_in_last_or_second_last_column(square) {
        Square::new(square.as_index() + ONE_ROW + TWO_COLUMNS)
    } else {
        None
    }
}

pub fn right_right_down(square: Square) -> Option<Square> {
    if !is_in_first_row(square) && !is_in_last_or_second_last_column(square) {
        Square::new(square.as_index() - ONE_ROW + TWO_COLUMNS)
    } else {
        None
    }
}

pub fn left_left_up(square: Square) -> Option<Square> {
    if !is_in_last_row(square) && !is_in_first_or_second_column(square) {
        Square::new(square.as_index() + ONE_ROW - TWO_COLUMNS)
    } else {
        None
    }
}

pub fn left_left_down(square: Square) -> Option<Square> {
    if !is_in_first_row(square) && !is_in_first_or_second_column(square) {
        Square::new(square.as_index() - ONE_ROW - TWO_COLUMNS)
    } else {
        None
    }
}
fn is_in_first_or_second_column(square: Square) -> bool {
    (square.as_index() % 8 == 0) || (square.as_index() % 8 == 1)
}

pub fn is_in_last_or_second_last_column(square: Square) -> bool {
    (square.as_index() % 8 == 7) || (square.as_index() % 8 == 6)
}

fn is_in_first_column(square: Square) -> bool {
    square.as_index() % 8 == 0
}

fn is_in_last_column(square: Square) -> bool {
    square.as_index() % 8 == 7
}

pub fn is_in_last_or_second_last_row(square: Square) -> bool {
    square >= A7
}

pub fn is_in_first_or_second_row(square: Square) -> bool {
    square <= H2
}

pub fn is_in_first_row(square: Square) -> bool {
    square < A2
}

pub fn is_in_last_row(square: Square) -> bool {
    square >= A8
}

pub fn is_in_row_2(square: Square) -> bool {
    A2 <= square && square <= H2
}

pub fn is_in_row_4(square: Square) -> bool {
    A4 <= square && square <= H4
}

pub fn is_in_row_5(square: Square) -> bool {
    A5 <= square && square <= H5
}
pub fn is_in_row_7(square: Square) -> bool {
    A7 <= square && square <= H7
}

pub mod squares;
#[cfg(test)]
mod tests;
