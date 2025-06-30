use crate::{
    directions::*,
    position::Position,
    possible_moves::king::{get_possible_black_moves, get_possible_white_moves},
};

#[test]
fn test_get_possible_black_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_possible_black_moves(&position, D8).len() == 0);
    assert!(get_possible_black_moves(&position, D3).len() == 8);
    assert!(get_possible_black_moves(&position, F2).len() == 8);
    assert!(get_possible_black_moves(&position, H6).len() == 3);
}

#[test]
fn test_get_possible_white_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_possible_white_moves(&position, D8).len() == 5);
    assert!(get_possible_white_moves(&position, D3).len() == 5);
    assert!(get_possible_white_moves(&position, F2).len() == 3);
    assert!(get_possible_white_moves(&position, H6).len() == 5);
}
