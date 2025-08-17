use crate::engine::heuristic::*;
use crate::engine::piece::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
    WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};
use crate::engine::position::*;

#[test]
fn test_equal_material() {
    let position = Position::default()
        .put_piece(WHITE_KING, 1)
        .put_piece(WHITE_BISHOP, 1)
        .put_piece(WHITE_PAWN, 1)
        .put_piece(WHITE_ROOK, 1)
        .put_piece(WHITE_KNIGHT, 1)
        .put_piece(WHITE_QUEEN, 1)
        .put_piece(BLACK_KING, 1)
        .put_piece(BLACK_BISHOP, 1)
        .put_piece(BLACK_KNIGHT, 1)
        .put_piece(BLACK_PAWN, 1)
        .put_piece(BLACK_ROOK, 1)
        .put_piece(BLACK_QUEEN, 1);
    assert_eq!(heuristic(&position), 0);
}

#[test]
fn test_white_queen_missing() {
    let position = Position::default()
        .put_piece(WHITE_KING, 1)
        .put_piece(WHITE_BISHOP, 1)
        .put_piece(WHITE_PAWN, 1)
        .put_piece(WHITE_ROOK, 1)
        .put_piece(WHITE_KNIGHT, 1)
        .put_piece(BLACK_QUEEN, 1)
        .put_piece(BLACK_KING, 1)
        .put_piece(BLACK_BISHOP, 1)
        .put_piece(BLACK_KNIGHT, 1)
        .put_piece(BLACK_PAWN, 1)
        .put_piece(BLACK_ROOK, 1);
    assert_eq!(heuristic(&position), -9);
}

#[test]
fn test_black_queen_missing() {
    let position = Position::default()
        .put_piece(WHITE_KING, 1)
        .put_piece(WHITE_BISHOP, 1)
        .put_piece(WHITE_PAWN, 1)
        .put_piece(WHITE_ROOK, 1)
        .put_piece(WHITE_KNIGHT, 1)
        .put_piece(WHITE_QUEEN, 1)
        .put_piece(BLACK_KING, 1)
        .put_piece(BLACK_BISHOP, 1)
        .put_piece(BLACK_KNIGHT, 1)
        .put_piece(BLACK_PAWN, 1)
        .put_piece(BLACK_ROOK, 1);
    assert_eq!(heuristic(&position), 9);
}
