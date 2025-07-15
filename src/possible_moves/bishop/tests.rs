use crate::{
    chess_moves::ChessMove,
    directions::*,
    piece::{WHITE_BISHOP, WHITE_KING},
    position::Position,
    possible_moves::bishop::get_possible_moves,
};

#[test]
fn test_get_possible_white_moves() {
    let position = Position::default().put_piece(WHITE_KING, A1);
    let positions = get_possible_moves(&position, G4, WHITE_BISHOP);
    assert!(positions.len() == 9);
    let mut left_up = false;
    let mut left_down = false;
    let mut right_up = false;
    let mut right_down = false;
    let mut not_valid = false;

    for position in positions {
        if position.is_occupied_by_piece(F5, WHITE_BISHOP) {
            left_up = true;
        }
        if position.is_occupied_by_piece(F3, WHITE_BISHOP) {
            left_down = true;
        }
        if position.is_occupied_by_piece(H5, WHITE_BISHOP) {
            right_up = true;
        }
        if position.is_occupied_by_piece(H3, WHITE_BISHOP) {
            right_down = true;
        }
        if position.is_occupied_by_piece(B2, WHITE_BISHOP) {
            not_valid = true;
        }
    }

    assert!(left_up);
    assert!(left_down);
    assert!(right_up);
    assert!(right_down);
    assert!(!not_valid);
}
