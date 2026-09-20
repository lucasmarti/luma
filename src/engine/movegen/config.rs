use crate::engine::{
    movegen::{
        directions::{self, DirectionFn},
        Square,
    },
    piece::*,
    position::Position,
    Mve,
};

pub const BOARD_MAX_DISTANCE: u32 = 7;

pub struct MoveConfig {
    pub max_distance: u32,
    pub directions: &'static [DirectionFn],
}

pub const ROOK_CONFIG: MoveConfig = MoveConfig {
    max_distance: BOARD_MAX_DISTANCE,
    directions: &[
        directions::up,
        directions::down,
        directions::left,
        directions::right,
    ],
};

pub const QUEEN_CONFIG: MoveConfig = MoveConfig {
    max_distance: BOARD_MAX_DISTANCE,
    directions: &[
        directions::up,
        directions::down,
        directions::left,
        directions::right,
        directions::up_left,
        directions::up_right,
        directions::down_left,
        directions::down_right,
    ],
};

pub const BISHOP_CONFIG: MoveConfig = MoveConfig {
    max_distance: BOARD_MAX_DISTANCE,
    directions: &[
        directions::up_left,
        directions::up_right,
        directions::down_left,
        directions::down_right,
    ],
};

pub const KING_CONFIG: MoveConfig = MoveConfig {
    max_distance: 1,
    directions: &[
        directions::left,
        directions::right,
        directions::up,
        directions::down,
        directions::up_left,
        directions::up_right,
        directions::down_left,
        directions::down_right,
    ],
};

pub const KNIGHT_CONFIG: MoveConfig = MoveConfig {
    max_distance: 1,
    directions: &[
        directions::right_right_down,
        directions::right_right_up,
        directions::left_left_up,
        directions::left_left_down,
        directions::up_up_left,
        directions::up_up_right,
        directions::down_down_left,
        directions::down_down_right,
    ],
};

pub type CastlingMovesFn = fn(position: &Position) -> Vec<Mve>;
pub type MovesFn = fn(position: &Position, piece: Piece, square: Square) -> Vec<Mve>;
pub const HORIZONTAL_VERTICAL_DIRECTIONS: [DirectionFn; 4] = [
    directions::up,
    directions::down,
    directions::left,
    directions::right,
];

pub const DIAGONAL_DIRECTIONS: [DirectionFn; 4] = [
    directions::up_left,
    directions::up_right,
    directions::down_left,
    directions::down_right,
];
