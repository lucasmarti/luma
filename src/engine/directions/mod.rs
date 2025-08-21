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

pub type DirectionFn = fn(u32) -> Option<u32>;
pub type RowFn = fn(u32) -> bool;

pub fn up(index: u32) -> Option<u32> {
    if !is_in_last_row(index) {
        Some(index + ONE_ROW)
    } else {
        None
    }
}

pub fn down(index: u32) -> Option<u32> {
    if !is_in_first_row(index) {
        Some(index - ONE_ROW)
    } else {
        None
    }
}

pub fn left(index: u32) -> Option<u32> {
    if !is_in_first_column(index) {
        Some(index - ONE_COLUMN)
    } else {
        None
    }
}

pub fn right(index: u32) -> Option<u32> {
    if !is_in_last_column(index) {
        Some(index + ONE_COLUMN)
    } else {
        None
    }
}

pub fn up_right(index: u32) -> Option<u32> {
    if !is_in_last_row(index) && !is_in_last_column(index) {
        Some(index + ONE_ROW + ONE_COLUMN)
    } else {
        None
    }
}
pub fn up_left(index: u32) -> Option<u32> {
    if !is_in_last_row(index) && !is_in_first_column(index) {
        Some(index + ONE_ROW - ONE_COLUMN)
    } else {
        None
    }
}
pub fn down_right(index: u32) -> Option<u32> {
    if !is_in_first_row(index) && !is_in_last_column(index) {
        Some(index - ONE_ROW + ONE_COLUMN)
    } else {
        None
    }
}
pub fn down_left(index: u32) -> Option<u32> {
    if !is_in_first_row(index) && !is_in_first_column(index) {
        Some(index - ONE_ROW - ONE_COLUMN)
    } else {
        None
    }
}

pub fn up_up_right(index: u32) -> Option<u32> {
    if !is_in_last_or_second_last_row(index) && !is_in_last_column(index) {
        Some(index + TWO_ROWS + ONE_COLUMN)
    } else {
        None
    }
}

pub fn up_up_left(index: u32) -> Option<u32> {
    if !is_in_last_or_second_last_row(index) && !is_in_first_column(index) {
        Some(index + TWO_ROWS - ONE_COLUMN)
    } else {
        None
    }
}

pub fn down_down_right(index: u32) -> Option<u32> {
    if !is_in_first_or_second_row(index) && !is_in_last_column(index) {
        Some(index - TWO_ROWS + ONE_COLUMN)
    } else {
        None
    }
}

pub fn down_down_left(index: u32) -> Option<u32> {
    if !is_in_first_or_second_row(index) && !is_in_first_column(index) {
        Some(index - TWO_ROWS - ONE_COLUMN)
    } else {
        None
    }
}

pub fn right_right_up(index: u32) -> Option<u32> {
    if !is_in_last_row(index) && !is_in_last_or_second_last_column(index) {
        Some(index + ONE_ROW + TWO_COLUMNS)
    } else {
        None
    }
}

pub fn right_right_down(index: u32) -> Option<u32> {
    if !is_in_first_row(index) && !is_in_last_or_second_last_column(index) {
        Some(index - ONE_ROW + TWO_COLUMNS)
    } else {
        None
    }
}

pub fn left_left_up(index: u32) -> Option<u32> {
    if !is_in_last_row(index) && !is_in_first_or_second_column(index) {
        Some(index + ONE_ROW - TWO_COLUMNS)
    } else {
        None
    }
}

pub fn left_left_down(index: u32) -> Option<u32> {
    if !is_in_first_row(index) && !is_in_first_or_second_column(index) {
        Some(index - ONE_ROW - TWO_COLUMNS)
    } else {
        None
    }
}
pub fn is_in_last_or_second_last_row(index: u32) -> bool {
    index >= A7
}

pub fn is_in_first_or_second_row(index: u32) -> bool {
    index <= H2
}

fn is_in_first_or_second_column(index: u32) -> bool {
    (index % 8 == 0) || (index % 8 == 1)
}

pub fn is_in_last_or_second_last_column(index: u32) -> bool {
    (index % 8 == 7) || (index % 8 == 6)
}

fn is_in_first_column(index: u32) -> bool {
    index % 8 == 0
}

fn is_in_last_column(index: u32) -> bool {
    index % 8 == 7
}

pub fn is_in_first_row(index: u32) -> bool {
    index < A2
}

pub fn is_in_last_row(index: u32) -> bool {
    index >= A8
}

pub fn is_in_row_2(index: u32) -> bool {
    A2 <= index && index <= H2
}

pub fn is_in_row_4(index: u32) -> bool {
    A4 <= index && index <= H4
}

pub fn is_in_row_5(index: u32) -> bool {
    A5 <= index && index <= H5
}
pub fn is_in_row_7(index: u32) -> bool {
    A7 <= index && index <= H7
}

pub mod squares;
#[cfg(test)]
mod tests;
