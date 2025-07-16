use crate::{
    directions::*, piece::WHITE_QUEEN, position::Position,
    possible_moves::queen::get_possible_moves,
};

#[test]
fn test_get_possible_white_moves() {
    let positions = get_possible_moves(&Position::default(), G4, WHITE_QUEEN);
    assert!(positions.len() == 23);

    let mut found_up = false;
    let mut found_down = false;
    let mut found_left = false;
    let mut found_right = false;
    let mut found_not = true;
    let mut found_left_up = false;
    let mut found_left_down = false;
    let mut found_right_up = false;
    let mut found_right_down = false;

    for position in positions {
        if position.is_occupied_by_piece(G3, WHITE_QUEEN) {
            found_down = true;
        }
        if position.is_occupied_by_piece(G8, WHITE_QUEEN) {
            found_up = true;
        }
        if position.is_occupied_by_piece(A4, WHITE_QUEEN) {
            found_left = true;
        }
        if position.is_occupied_by_piece(H4, WHITE_QUEEN) {
            found_right = true;
        }
        if position.is_occupied_by_piece(B2, WHITE_QUEEN) {
            found_not = false;
        }
        if position.is_occupied_by_piece(F5, WHITE_QUEEN) {
            found_left_up = true;
        }
        if position.is_occupied_by_piece(F3, WHITE_QUEEN) {
            found_left_down = true;
        }
        if position.is_occupied_by_piece(H5, WHITE_QUEEN) {
            found_right_up = true;
        }
        if position.is_occupied_by_piece(H3, WHITE_QUEEN) {
            found_right_down = true;
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
    // left up
    assert!(found_left_up);
    // left down
    assert!(found_left_down);
    // right up
    assert!(found_right_up);
    // right down
    assert!(found_right_down);
}
