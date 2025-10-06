use crate::engine::{
    directions::squares::*,
    get_valid_drop_positions,
    piece::{BLACK_PAWN, WHITE_KING, WHITE_PAWN, WHITE_ROOK},
    position::Position,
};

#[test]
fn test_valid_drop_targets_pawn() {
    let position = Position::new_starting_position();
    let targets = get_valid_drop_positions(&position, D2);
    assert!(targets.iter().any(|p| p.is_occupied(D3)));
    assert!(targets.iter().any(|p| p.is_occupied(D4)));
}
#[test]
fn test_valid_drop_targets_knight() {
    let position = Position::new_starting_position();
    let targets = get_valid_drop_positions(&position, B8);
    assert!(targets.iter().any(|p| p.is_occupied(A6)));
    assert!(targets.iter().any(|p| p.is_occupied(C6)));
}

#[test]
fn test_valid_drop_targets_castle() {
    let position = Position::default()
        .put_piece(WHITE_ROOK, H1)
        .put_piece(WHITE_KING, E1);
    let targets = get_valid_drop_positions(&position, E1);
    assert!(targets.iter().any(|p| p.is_occupied(G1)));
}

#[test]
fn test_valid_drop_targets_en_passant() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_PAWN, E4)
        .put_piece(BLACK_PAWN, D4)
        .set_en_passant(E4);
    let targets = get_valid_drop_positions(&position, D4);
    println!("{:?}", targets);
    assert!(targets.iter().any(|p| p.is_occupied(E3)));
}
