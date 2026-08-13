use std::assert_eq;

use crate::engine::{movegen::*, piece::*};

use super::{print::Print, Position};

#[test]
fn test_starting() {
    let position: Position = Position::starting();
    position.print_board();
}
#[test]
fn test_get_black() {
    let position = Position::starting();
    assert_eq!(position.occupied[Color::Black].count_ones(), 16);
}
#[test]
fn test_get_white() {
    let position = Position::starting();
    assert_eq!(position.occupied[Color::White].count_ones(), 16);
}

#[test]
fn test_get_all() {
    let position = Position::starting();
    assert_eq!(position.all.count_ones(), 32);
}

#[test]
fn test_remove_white_king() {
    let mut position = Position::starting();
    assert_eq!(position.boards[(Color::White, Typ::King)].count_ones(), 1);
    position.remove_piece(E1);
    assert_eq!(position.boards[(Color::White, Typ::King)].count_ones(), 0);
}

#[test]
fn test_put_white_king() {
    let position = Position::starting();
    let new_position = position.with_piece(WHITE_KING, E2);
    assert_ne!(position.is_occupied_by_piece(E2, WHITE_KING), true);
    assert_eq!(new_position.is_occupied_by_piece(E2, WHITE_KING), true);
}

#[test]
fn test_en_passant() {
    let mut position = Position::starting();
    assert_eq!(position.get_en_passant(), None);
    position = position.with_en_passant(Some(E4));
    assert_eq!(position.en_passant, Some(E4));
    position = position.with_en_passant(None);
    assert_eq!(position.get_en_passant(), None);
}
