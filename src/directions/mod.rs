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

const ONE_ROW: u32 = 8;
const TWO_ROWS: u32 = 16;
const ONE_COLUMN: u32 = 1;
const TWO_COLUMNS: u32 = 2;

pub const A1: u32 = 0;
pub const A2: u32 = 8;
pub const A3: u32 = 16;
pub const A4: u32 = 24;
pub const A5: u32 = 32;
pub const A6: u32 = 40;
pub const A7: u32 = 48;
pub const A8: u32 = 56;

pub const B1: u32 = 1;
pub const B2: u32 = 9;
pub const B3: u32 = 17;
pub const B4: u32 = 25;
pub const B5: u32 = 33;
pub const B6: u32 = 41;
pub const B7: u32 = 49;
pub const B8: u32 = 57;

pub const C1: u32 = 2;
pub const C2: u32 = 10;
pub const C3: u32 = 18;
pub const C4: u32 = 26;
pub const C5: u32 = 34;
pub const C6: u32 = 42;
pub const C7: u32 = 50;
pub const C8: u32 = 58;

pub const D1: u32 = 3;
pub const D2: u32 = 11;
pub const D3: u32 = 19;
pub const D4: u32 = 27;
pub const D5: u32 = 35;
pub const D6: u32 = 43;
pub const D7: u32 = 51;
pub const D8: u32 = 59;

pub const E1: u32 = 4;
pub const E2: u32 = 12;
pub const E3: u32 = 20;
pub const E4: u32 = 28;
pub const E5: u32 = 36;
pub const E6: u32 = 44;
pub const E7: u32 = 52;
pub const E8: u32 = 60;

pub const F1: u32 = 5;
pub const F2: u32 = 13;
pub const F3: u32 = 21;
pub const F4: u32 = 29;
pub const F5: u32 = 37;
pub const F6: u32 = 45;
pub const F7: u32 = 53;
pub const F8: u32 = 61;

pub const G1: u32 = 6;
pub const G2: u32 = 14;
pub const G3: u32 = 22;
pub const G4: u32 = 30;
pub const G5: u32 = 38;
pub const G6: u32 = 46;
pub const G7: u32 = 54;
pub const G8: u32 = 62;

pub const H1: u32 = 7;
pub const H2: u32 = 15;
pub const H3: u32 = 23;
pub const H4: u32 = 31;
pub const H5: u32 = 39;
pub const H6: u32 = 47;
pub const H7: u32 = 55;
pub const H8: u32 = 63;
#[derive(Copy, Clone)]
pub enum EnPassantField {
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
}

pub fn two_up(index: u32) -> Option<u32> {
    if let Some(one_up) = up(index) {
        return up(one_up);
    }
    None
}

pub fn two_down(index: u32) -> Option<u32> {
    if let Some(one_down) = down(index) {
        return down(one_down);
    }
    None
}
pub fn all_up(index: u32) -> Vec<u32> {
    let mut all: Vec<u32> = Vec::new();
    let mut optional = up(index);
    while let Some(current) = optional {
        all.push(current);
        optional = up(current);
    }
    all
}

pub fn all_left(index: u32) -> Vec<u32> {
    let mut all: Vec<u32> = Vec::new();
    let mut optional = left(index);
    while let Some(current) = optional {
        all.push(current);
        optional = left(current);
    }
    all
}

pub fn all_right(index: u32) -> Vec<u32> {
    let mut all: Vec<u32> = Vec::new();
    let mut optional = right(index);
    while let Some(current) = optional {
        all.push(current);
        optional = right(current);
    }
    all
}

pub fn all_down(index: u32) -> Vec<u32> {
    let mut all: Vec<u32> = Vec::new();
    let mut optional = down(index);
    while let Some(current) = optional {
        all.push(current);
        optional = down(current);
    }
    all
}

pub fn all_up_left(index: u32) -> Vec<u32> {
    let mut all: Vec<u32> = Vec::new();
    let mut optional = up_left(index);
    while let Some(current) = optional {
        all.push(current);
        optional = up_left(current);
    }
    all
}

pub fn all_up_right(index: u32) -> Vec<u32> {
    let mut all: Vec<u32> = Vec::new();
    let mut optional = up_right(index);
    while let Some(current) = optional {
        all.push(current);
        optional = up_right(current);
    }
    all
}

pub fn all_down_left(index: u32) -> Vec<u32> {
    let mut all: Vec<u32> = Vec::new();
    let mut optional = down_left(index);
    while let Some(current) = optional {
        all.push(current);
        optional = down_left(current);
    }
    all
}

pub fn all_down_right(index: u32) -> Vec<u32> {
    let mut all: Vec<u32> = Vec::new();
    let mut optional = down_right(index);
    while let Some(current) = optional {
        all.push(current);
        optional = down_right(current);
    }
    all
}

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
    index >= A2 && index <= H2
}

pub fn is_in_row_7(index: u32) -> bool {
    index >= A7 && index <= H7
}

#[cfg(test)]
mod tests;
