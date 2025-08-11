use crate::{
    directions::{self, DirectionFn},
    piece::{
        Piece, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
        WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    position::Position,
    possible_moves::{
        castle::{get_black_castle_moves, get_white_castle_moves},
        common::{
            get_moves_for_bishop_at_square, get_moves_for_king_at_square,
            get_moves_for_knight_at_square, get_moves_for_queen_at_square,
            get_moves_for_rook_at_square,
        },
        pawn::get_pawn_moves,
    },
};

pub const ROOK_MAX_DISTANCE: u32 = u32::MAX;
pub const ROOK_DIRECTIONS: [DirectionFn; 4] = [
    directions::up,
    directions::down,
    directions::left,
    directions::right,
];
pub const QUEEN_MAX_DISTANCE: u32 = u32::MAX;
pub const QUEEN_DIRECTIONS: [DirectionFn; 8] = [
    directions::up,
    directions::down,
    directions::left,
    directions::right,
    directions::up_left,
    directions::up_right,
    directions::down_left,
    directions::down_right,
];
pub const BISHOP_MAX_DISTANCE: u32 = u32::MAX;
pub const BISHOP_DIRECTIONS: [DirectionFn; 4] = [
    directions::up_left,
    directions::up_right,
    directions::down_left,
    directions::down_right,
];
pub const KING_MAX_DISTANCE: u32 = 1;
pub const KING_DIRECTIONS: [DirectionFn; 8] = [
    directions::left,
    directions::right,
    directions::up,
    directions::down,
    directions::up_left,
    directions::up_right,
    directions::down_left,
    directions::down_right,
];
pub const KNIGHT_MAX_DISTANCE: u32 = 1;
pub const KNIGHT_DIRECTIONS: [DirectionFn; 8] = [
    directions::right_right_down,
    directions::right_right_up,
    directions::left_left_up,
    directions::left_left_down,
    directions::up_up_left,
    directions::up_up_right,
    directions::down_down_left,
    directions::down_down_right,
];

pub type CastleMovesFn = fn(position: &Position) -> Vec<Position>;
pub type MovesFn = fn(position: &Position, piece: Piece, square: u32) -> Vec<Position>;
pub struct Config {
    pub king: Piece,
    pub queen: Piece,
    pub rook: Piece,
    pub knight: Piece,
    pub bishop: Piece,
    pub pawn: Piece,
    pub king_fn: MovesFn,
    pub queen_fn: MovesFn,
    pub rook_fn: MovesFn,
    pub knight_fn: MovesFn,
    pub bishop_fn: MovesFn,
    pub pawn_fn: MovesFn,
    pub castle_move_fn: CastleMovesFn,
}

pub const WHITE_MOVE_CONFIG: Config = Config {
    king: WHITE_KING,
    queen: WHITE_QUEEN,
    rook: WHITE_ROOK,
    knight: WHITE_KNIGHT,
    bishop: WHITE_BISHOP,
    pawn: WHITE_PAWN,
    king_fn: get_moves_for_king_at_square,
    queen_fn: get_moves_for_queen_at_square,
    rook_fn: get_moves_for_rook_at_square,
    knight_fn: get_moves_for_knight_at_square,
    bishop_fn: get_moves_for_bishop_at_square,
    castle_move_fn: get_white_castle_moves,
    pawn_fn: get_pawn_moves,
};

pub const BLACK_MOVE_CONFIG: Config = Config {
    king: BLACK_KING,
    queen: BLACK_QUEEN,
    rook: BLACK_ROOK,
    knight: BLACK_KNIGHT,
    bishop: BLACK_BISHOP,
    pawn: BLACK_PAWN,
    king_fn: get_moves_for_king_at_square,
    queen_fn: get_moves_for_queen_at_square,
    rook_fn: get_moves_for_rook_at_square,
    knight_fn: get_moves_for_knight_at_square,
    bishop_fn: get_moves_for_bishop_at_square,
    castle_move_fn: get_black_castle_moves,
    pawn_fn: get_pawn_moves,
};

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
