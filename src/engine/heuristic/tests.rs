use crate::engine::directions::squares::{D1, D8};
use crate::engine::heuristic::*;
use crate::engine::position::*;

#[test]
fn test_equal_material() {
    let position = Position::new_starting_position();
    assert_eq!(heuristic(&position), 0.0);
}

#[test]
fn test_white_queen_missing() {
    let position = Position::new_starting_position().remove_piece(D1);
    assert_eq!(heuristic(&position), -88.0);
}

#[test]
fn test_black_queen_missing() {
    let position = Position::new_starting_position().remove_piece(D8);
    assert_eq!(
        material::count_white(&position) - material::count_black(&position),
        90.0
    );
    assert_eq!(
        mobility::count_white(&position) - mobility::count_black(&position),
        -1.5
    );
    assert_eq!(
        squares::count_white(&position) - squares::count_black(&position),
        -0.5
    );
    assert_eq!(heuristic(&position), 88.0);
}
