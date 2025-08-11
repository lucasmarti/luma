use crate::engine::{
    chess_moves::get_white_moves,
    directions::*,
    get_valid_drop_targets,
    piece::{BLACK_PAWN, WHITE_KING, WHITE_PAWN, WHITE_ROOK},
    position::Position,
    tests,
};

#[test]
fn test_valid_drop_targets_pawn() {
    let position = Position::new_starting_position();
    let targets = get_valid_drop_targets(&position, D2);
    assert!(targets.contains(&D3));
    assert!(targets.contains(&D4));
}
#[test]
fn test_valid_drop_targets_knight() {
    let position = Position::new_starting_position();
    let targets = get_valid_drop_targets(&position, B8);
    assert!(targets.contains(&A6));
    assert!(targets.contains(&C6));
}

#[test]
fn test_valid_drop_targets_castle() {
    let position = Position::default()
        .put_piece(WHITE_ROOK, G1)
        .put_piece(WHITE_KING, E1);
    let targets = get_valid_drop_targets(&position, E1);
    assert!(targets.contains(&F1));
}

#[test]
fn test_valid_drop_targets_en_passant() {
    let position = Position::default()
        .put_piece(WHITE_PAWN, E4)
        .put_piece(BLACK_PAWN, D4)
        .set_en_passant(E4);
    let targets = get_valid_drop_targets(&position, D4);
    println!("{:?}", targets);
    assert!(targets.contains(&E3));
}
