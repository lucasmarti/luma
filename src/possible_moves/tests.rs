use crate::{
    directions::*,
    piece::{
        BLACK_BISHOP, BLACK_KING, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP, WHITE_KING,
        WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    position::{self, Position},
    possible_moves::get_all_moves,
};

#[test]
fn test_all_moves_from_starting_position() {
    let position = Position::new_starting_position();
    assert!(get_all_moves(&position, crate::piece::Color::White).len() == 20);
}
#[test]
fn test_position1() {
    let white = Position::default()
        .put_piece(WHITE_PAWN, A2)
        .put_piece(WHITE_PAWN, C3)
        .put_piece(WHITE_PAWN, C4)
        .put_piece(WHITE_QUEEN, D3)
        .put_piece(WHITE_ROOK, F1)
        .put_piece(WHITE_PAWN, F2)
        .put_piece(WHITE_KNIGHT, F3)
        .put_piece(WHITE_KING, G1)
        .put_piece(WHITE_BISHOP, G2)
        .put_piece(WHITE_PAWN, G3)
        .put_piece(WHITE_PAWN, H2);
    let positions = get_all_moves(&white, crate::piece::Color::White);
    assert!(positions.len() == 35);
    let black = Position::default()
        .put_piece(BLACK_PAWN, A7)
        .put_piece(BLACK_PAWN, B7)
        .put_piece(BLACK_PAWN, D6)
        .put_piece(BLACK_PAWN, F7)
        .put_piece(BLACK_PAWN, G7)
        .put_piece(BLACK_PAWN, H6)
        .put_piece(BLACK_BISHOP, B6)
        .put_piece(BLACK_ROOK, E4)
        .put_piece(BLACK_BISHOP, G4)
        .put_piece(BLACK_QUEEN, G6)
        .put_piece(BLACK_KING, G8);
    let positions = get_all_moves(&black, crate::piece::Color::Black);
    assert!(positions.len() == 44);

    let all = Position::default()
        .put_piece(WHITE_PAWN, A2)
        .put_piece(WHITE_PAWN, C3)
        .put_piece(WHITE_PAWN, C4)
        .put_piece(WHITE_QUEEN, D3)
        .put_piece(WHITE_ROOK, F1)
        .put_piece(WHITE_PAWN, F2)
        .put_piece(WHITE_KNIGHT, F3)
        .put_piece(WHITE_KING, G1)
        .put_piece(WHITE_BISHOP, G2)
        .put_piece(WHITE_PAWN, G3)
        .put_piece(WHITE_PAWN, H2)
        .put_piece(BLACK_PAWN, A7)
        .put_piece(BLACK_PAWN, B7)
        .put_piece(BLACK_PAWN, D6)
        .put_piece(BLACK_PAWN, F7)
        .put_piece(BLACK_PAWN, G7)
        .put_piece(BLACK_PAWN, H6)
        .put_piece(BLACK_BISHOP, B6)
        .put_piece(BLACK_ROOK, E4)
        .put_piece(BLACK_BISHOP, G4)
        .put_piece(BLACK_QUEEN, G6)
        .put_piece(BLACK_KING, G8);

    assert!(39 == get_all_moves(&all, crate::piece::Color::Black).len());
    assert!(29 == get_all_moves(&all, crate::piece::Color::White).len());
}
