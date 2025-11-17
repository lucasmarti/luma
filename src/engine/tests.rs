use crate::engine::{
    directions::squares::*,
    get_valid_drop_positions,
    piece::Piece::{self, BlackPawn, WhiteKing, WhitePawn, WhiteRook},
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
fn test_valid_drop_targets_castling() {
    let position = Position::default()
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::WhiteKing, E1);
    let targets = get_valid_drop_positions(&position, E1);
    assert!(targets.iter().any(|p| p.is_occupied(G1)));
}

#[test]
fn test_valid_drop_targets_en_passant() {
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhitePawn, E4)
        .put_piece(Piece::BlackPawn, D4)
        .set_en_passant(E4);
    let targets = get_valid_drop_positions(&position, D4);
    println!("{:?}", targets);
    assert!(targets.iter().any(|p| p.is_occupied(E3)));
}
