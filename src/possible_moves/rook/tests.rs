use crate::{
    directions::*,
    piece::{BLACK_ROOK, WHITE_ROOK},
    position::Position,
    possible_moves::rook::get_possible_moves,
};

#[test]
fn test_get_possible_white_moves() {
    let positions = get_possible_moves(&Position::default(), G4, BLACK_ROOK);

    assert!(positions.len() == 14);
    let mut found_up = false;
    let mut found_down = false;
    let mut found_left = false;
    let mut found_right = false;
    let mut found_not = true;

    for position in positions {
        if position.is_occupied_by_piece(G3, BLACK_ROOK) {
            found_down = true;
        }
        if position.is_occupied_by_piece(G8, BLACK_ROOK) {
            found_up = true;
        }
        if position.is_occupied_by_piece(A4, BLACK_ROOK) {
            found_left = true;
        }
        if position.is_occupied_by_piece(H4, BLACK_ROOK) {
            found_right = true;
        }
        if position.is_occupied_by_piece(B2, BLACK_ROOK) {
            found_not = false;
        }
    }
    // down
    assert!(found_down);
    // up
    assert!(found_up);
    // left
    assert!(found_left);
    // right
    assert!(found_right);
    // not
    assert!(found_not);
}
