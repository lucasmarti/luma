use once_cell::sync::Lazy;

use crate::engine::{directions::squares::Square, piece::*, position::Position};

type PieceSquareRow = [f32; 8];

const WHITE_PAWN_ROW_1: PieceSquareRow = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
const WHITE_PAWN_ROW_2: PieceSquareRow = [0.5, 1.0, 1.0, -2.0, -2.0, 1.0, 1.0, 0.5];
const WHITE_PAWN_ROW_3: PieceSquareRow = [0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5];
const WHITE_PAWN_ROW_4: PieceSquareRow = [0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 0.0];
const WHITE_PAWN_ROW_5: PieceSquareRow = [0.5, 0.5, 1.0, 2.5, 2.5, 1.0, 0.5, 0.5];
const WHITE_PAWN_ROW_6: PieceSquareRow = [1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0];
const WHITE_PAWN_ROW_7: PieceSquareRow = [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0];
const WHITE_PAWN_ROW_8: PieceSquareRow = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

const KNIGHT_ROW_8: PieceSquareRow = [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0];
const KNIGHT_ROW_7: PieceSquareRow = [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0];
const KNIGHT_ROW_6: PieceSquareRow = [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0];
const KNIGHT_ROW_5: PieceSquareRow = [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0];
const KNIGHT_ROW_4: PieceSquareRow = [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0];
const KNIGHT_ROW_3: PieceSquareRow = [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0];
const KNIGHT_ROW_2: PieceSquareRow = [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0];
const KNIGHT_ROW_1: PieceSquareRow = [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0];

const BISHOP_ROW_8: PieceSquareRow = [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0];
const BISHOP_ROW_7: PieceSquareRow = [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0];
const BISHOP_ROW_6: PieceSquareRow = [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0];
const BISHOP_ROW_5: PieceSquareRow = [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0];
const BISHOP_ROW_4: PieceSquareRow = [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0];
const BISHOP_ROW_3: PieceSquareRow = [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0];
const BISHOP_ROW_2: PieceSquareRow = [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0];
const BISHOP_ROW_1: PieceSquareRow = [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0];

const ROOK_ROW_8: PieceSquareRow = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
const ROOK_ROW_7: PieceSquareRow = [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5];
const ROOK_ROW_6: PieceSquareRow = [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5];
const ROOK_ROW_5: PieceSquareRow = [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5];
const ROOK_ROW_4: PieceSquareRow = [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5];
const ROOK_ROW_3: PieceSquareRow = [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5];
const ROOK_ROW_2: PieceSquareRow = [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5];
const ROOK_ROW_1: PieceSquareRow = [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0];

const QUEEN_ROW_8: PieceSquareRow = [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0];
const QUEEN_ROW_7: PieceSquareRow = [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0];
const QUEEN_ROW_6: PieceSquareRow = [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0];
const QUEEN_ROW_5: PieceSquareRow = [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5];
const QUEEN_ROW_4: PieceSquareRow = [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5];
const QUEEN_ROW_3: PieceSquareRow = [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0];
const QUEEN_ROW_2: PieceSquareRow = [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0];
const QUEEN_ROW_1: PieceSquareRow = [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0];

const WHITE_KING_ROW_8: PieceSquareRow = [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0];
const WHITE_KING_ROW_7: PieceSquareRow = [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0];
const WHITE_KING_ROW_6: PieceSquareRow = [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0];
const WHITE_KING_ROW_5: PieceSquareRow = [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0];
const WHITE_KING_ROW_4: PieceSquareRow = [-2.0, -3.0, -4.0, -4.0, -4.0, -4.0, -3.0, -2.0];
const WHITE_KING_ROW_3: PieceSquareRow = [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0];
const WHITE_KING_ROW_2: PieceSquareRow = [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0];
const WHITE_KING_ROW_1: PieceSquareRow = [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0];

static WHITE_PAWN_TABLE: Lazy<Vec<f32>> = Lazy::new(|| {
    [
        WHITE_PAWN_ROW_1,
        WHITE_PAWN_ROW_2,
        WHITE_PAWN_ROW_3,
        WHITE_PAWN_ROW_4,
        WHITE_PAWN_ROW_5,
        WHITE_PAWN_ROW_6,
        WHITE_PAWN_ROW_7,
        WHITE_PAWN_ROW_8,
    ]
    .concat()
});

static BLACK_PAWN_TABLE: Lazy<Vec<f32>> = Lazy::new(|| {
    [
        WHITE_PAWN_ROW_8,
        WHITE_PAWN_ROW_7,
        WHITE_PAWN_ROW_6,
        WHITE_PAWN_ROW_5,
        WHITE_PAWN_ROW_4,
        WHITE_PAWN_ROW_3,
        WHITE_PAWN_ROW_2,
        WHITE_PAWN_ROW_1,
    ]
    .concat()
});

static KNIGHT_TABLE: Lazy<Vec<f32>> = Lazy::new(|| {
    [
        KNIGHT_ROW_1,
        KNIGHT_ROW_2,
        KNIGHT_ROW_3,
        KNIGHT_ROW_4,
        KNIGHT_ROW_5,
        KNIGHT_ROW_6,
        KNIGHT_ROW_7,
        KNIGHT_ROW_8,
    ]
    .concat()
});

static QUEEN_TABLE: Lazy<Vec<f32>> = Lazy::new(|| {
    [
        QUEEN_ROW_1,
        QUEEN_ROW_2,
        QUEEN_ROW_3,
        QUEEN_ROW_4,
        QUEEN_ROW_5,
        QUEEN_ROW_6,
        QUEEN_ROW_7,
        QUEEN_ROW_8,
    ]
    .concat()
});

static BISHOP_TABLE: Lazy<Vec<f32>> = Lazy::new(|| {
    [
        BISHOP_ROW_1,
        BISHOP_ROW_2,
        BISHOP_ROW_3,
        BISHOP_ROW_4,
        BISHOP_ROW_5,
        BISHOP_ROW_6,
        BISHOP_ROW_7,
        BISHOP_ROW_8,
    ]
    .concat()
});

static ROOK_TABLE: Lazy<Vec<f32>> = Lazy::new(|| {
    [
        ROOK_ROW_1, ROOK_ROW_2, ROOK_ROW_3, ROOK_ROW_4, ROOK_ROW_5, ROOK_ROW_6, ROOK_ROW_7,
        ROOK_ROW_8,
    ]
    .concat()
});

static WHITE_KING_TABLE: Lazy<Vec<f32>> = Lazy::new(|| {
    [
        WHITE_KING_ROW_1,
        WHITE_KING_ROW_2,
        WHITE_KING_ROW_3,
        WHITE_KING_ROW_4,
        WHITE_KING_ROW_5,
        WHITE_KING_ROW_6,
        WHITE_KING_ROW_7,
        WHITE_KING_ROW_8,
    ]
    .concat()
});

static BLACK_KING_TABLE: Lazy<Vec<f32>> = Lazy::new(|| {
    [
        WHITE_KING_ROW_8,
        WHITE_KING_ROW_7,
        WHITE_KING_ROW_6,
        WHITE_KING_ROW_5,
        WHITE_KING_ROW_4,
        WHITE_KING_ROW_3,
        WHITE_KING_ROW_2,
        WHITE_KING_ROW_1,
    ]
    .concat()
});

fn get_table(piece: Piece) -> &'static once_cell::sync::Lazy<Vec<f32>> {
    match piece.typ {
        crate::engine::piece::Typ::Knight => &KNIGHT_TABLE,
        crate::engine::piece::Typ::Bishop => &BISHOP_TABLE,
        crate::engine::piece::Typ::Queen => &QUEEN_TABLE,
        crate::engine::piece::Typ::Rook => &ROOK_TABLE,
        crate::engine::piece::Typ::Pawn => match piece.color {
            crate::engine::piece::Color::Black => &BLACK_PAWN_TABLE,
            crate::engine::piece::Color::White => &WHITE_PAWN_TABLE,
        },
        crate::engine::piece::Typ::King => match piece.color {
            crate::engine::piece::Color::Black => &BLACK_KING_TABLE,
            crate::engine::piece::Color::White => &WHITE_KING_TABLE,
        },
    }
}
fn get_value(piece: Piece, square: Square) -> f32 {
    get_table(piece)[square.as_index() as usize]
}
fn get_score_for_pieces(position: &Position, pieces: [Piece; 6]) -> f32 {
    let mut total_score: f32 = 0.0;
    for piece in pieces {
        let mut piece_score: f32 = 0.0;
        for square in position.get_squares(piece).iter() {
            piece_score = piece_score + get_value(piece, square);
        }
        total_score = total_score + piece_score;
    }
    total_score
}

pub fn count_black(position: &Position) -> f32 {
    get_score_for_pieces(position, BLACK_PIECES)
}

pub fn count_white(position: &Position) -> f32 {
    get_score_for_pieces(position, WHITE_PIECES)
}

const WHITE_PIECES: [Piece; 6] = [
    WHITE_QUEEN,
    WHITE_KING,
    WHITE_BISHOP,
    WHITE_KNIGHT,
    WHITE_PAWN,
    WHITE_ROOK,
];

const BLACK_PIECES: [Piece; 6] = [
    BLACK_QUEEN,
    BLACK_KING,
    BLACK_BISHOP,
    BLACK_KNIGHT,
    BLACK_PAWN,
    BLACK_ROOK,
];
