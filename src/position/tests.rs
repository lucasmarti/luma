use crate::bitboard::Bitboard;

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


