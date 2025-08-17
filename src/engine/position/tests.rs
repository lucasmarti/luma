use crate::engine::{directions::squares::*, piece::*};

use super::{print::Print, Position};

#[test]
fn test_new_starting_position() {
    let position: Position = Position::new_starting_position();
    position.print_board();
}
#[test]
fn test_get_black() {
    let position = Position::new_starting_position();
    assert_eq!(position.get_black().count_ones(), 16);
}
#[test]
fn test_get_white() {
    let position = Position::new_starting_position();
    assert_eq!(position.get_white().count_ones(), 16);
}

#[test]
fn test_get_all() {
    let position = Position::new_starting_position();
    assert_eq!(position.get_all().count_ones(), 32);
}

#[test]
fn test_remove_white_king() {
    let position = Position::new_starting_position();
    assert_eq!(position.white_king.count_ones(), 1);
    let new_position = position.remove_piece(E1);
    assert_eq!(new_position.white_king.count_ones(), 0);
}

#[test]
fn test_put_white_king() {
    let position = Position::new_starting_position();
    let new_position = position.put_piece(WHITE_KING, E2);
    assert_ne!(position.is_occupied_by_piece(E2, WHITE_KING), true);
    assert_eq!(new_position.is_occupied_by_piece(E2, WHITE_KING), true);
}
