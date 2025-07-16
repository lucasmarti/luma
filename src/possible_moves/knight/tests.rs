use crate::{
    directions::*,
    piece::{BLACK_PAWN, WHITE_KNIGHT, WHITE_PAWN},
    position::Position,
    possible_moves::knight::get_possible_moves,
};

#[test]
fn test_knight_moves() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KNIGHT, E4)
        .put_piece(WHITE_PAWN, C5)
        .put_piece(BLACK_PAWN, G2);

    let positions = get_possible_moves(&position, E4, WHITE_KNIGHT);

    let mut found_c3 = false;
    let mut found_d6 = false;
    let mut found_f6 = false;
    let mut found_d2 = false;
    let mut found_f2 = false;
    let mut found_g3 = false;
    let mut found_g5 = false;
    let mut found_not_c5 = true;

    println!("{:?}", positions.len());
    for position in positions {
        if position.is_occupied_by_piece(C3, WHITE_KNIGHT) {
            found_c3 = true;
        }
        if position.is_occupied_by_piece(D6, WHITE_KNIGHT) {
            found_d6 = true;
        }
        if position.is_occupied_by_piece(F6, WHITE_KNIGHT) {
            found_f6 = true;
        }
        if position.is_occupied_by_piece(D2, WHITE_KNIGHT) {
            found_d2 = true;
        }
        if position.is_occupied_by_piece(F2, WHITE_KNIGHT) {
            found_f2 = true;
        }
        if position.is_occupied_by_piece(G3, WHITE_KNIGHT) {
            found_g3 = true;
        }
        if position.is_occupied_by_piece(G5, WHITE_KNIGHT) {
            found_g5 = true;
        }
        if position.is_occupied_by_piece(C5, WHITE_KNIGHT) {
            found_not_c5 = false;
        }
    }

    assert!(found_c3);
    assert!(found_d6);
    assert!(found_f6);
    assert!(found_d2);
    assert!(found_f2);
    assert!(found_g3);
    assert!(found_g5);
    assert!(found_not_c5);
}
