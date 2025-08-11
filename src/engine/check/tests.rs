use super::*;
use crate::engine::{directions::*, position::Position};

#[test]
fn test_starting_position_no_check() {
    let position = Position::new_starting_position();
    assert!(!is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_rook_check() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, E4);

    // Black rook on E4 should put white king on E1 in check
    assert!(is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_bishop_check() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_BISHOP, H4);

    // Black bishop on H4 should put white king on E1 in check
    assert!(is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_queen_check_diagonal() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_QUEEN, A5);

    // Black queen on A5 should put white king on E1 in check (diagonal)
    assert!(is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_queen_check_horizontal() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_QUEEN, E5);

    // Black queen on E5 should put white king on E1 in check (vertical)
    assert!(is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_knight_check() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_KNIGHT, D3);

    // Black knight on D3 should put white king on E1 in check
    assert!(is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_pawn_check_white() {
    let mut position = Position::default();
    position = position
        .put_piece(BLACK_KING, E4)
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_PAWN, D3);

    // White pawn on D3 should put black king on E4 in check
    assert!(is_check(&position, Color::Black));
    assert!(!is_check(&position, Color::White));
}

#[test]
fn test_pawn_check_black() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_PAWN, D2);

    // Black pawn on D2 should put white king on E1 in check
    assert!(is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_blocked_rook_no_check() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, E5)
        .put_piece(WHITE_PAWN, E2);

    // Black rook on E5 should NOT put white king on E1 in check (blocked by pawn)
    assert!(!is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_blocked_bishop_no_check() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_BISHOP, H4)
        .put_piece(WHITE_KNIGHT, F2);

    // Black bishop on H4 should NOT put white king on E1 in check (blocked by knight)
    assert!(!is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_multiple_checks() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, E5)
        .put_piece(BLACK_BISHOP, H4);

    // Both black rook and bishop should put white king in check
    assert!(is_check(&position, Color::White));
    assert!(!is_check(&position, Color::Black));
}

#[test]
fn test_king_check() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_KING, E1).put_piece(BLACK_KING, E2);

    // Kings adjacent to each other - both in check (illegal position but tests the logic)
    assert!(is_check(&position, Color::White));
    assert!(is_check(&position, Color::Black));
}
