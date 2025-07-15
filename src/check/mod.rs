use crate::{
    bitboard::{self, Bitboard},
    directions::{self, DirectionFn},
    piece::{Color, Typ, *},
    position::Position,
    possible_moves::{king::KING_DIRECTIONS, knight::KNIGHT_DIRECTIONS},
};

const HORIZONTAL_VERTICAL_DIRECTIONS: [DirectionFn; 4] = [
    directions::up,
    directions::down,
    directions::left,
    directions::right,
];

const DIAGONAL_DIRECTIONS: [DirectionFn; 4] = [
    directions::up_left,
    directions::up_right,
    directions::down_left,
    directions::down_right,
];

#[cfg(test)]
mod tests;
