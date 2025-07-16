use crate::{
    chess_moves::{
        self, black_kingside_castle, black_queenside_castle, en_passant, progess, promote,
        white_kingside_castle, white_queenside_castle,
    },
    directions::{self, *},
    piece::*,
    position::{self, Position},
};
#[test]
fn test_progress_white_king() {
    let position = Position::default();
    let new_position = progess(&position, WHITE_KING, E1, E2);
    assert!(!new_position.is_occupied_by_piece(E1, WHITE_KING));
    assert!(new_position.is_occupied_by_piece(E2, WHITE_KING));
}

#[test]
fn test_promotion() {
    let position = Position::default().put_piece(WHITE_PAWN, A7);
    let new_position = promote(&position, A7, A8, WHITE_QUEEN);
    assert!(new_position.is_occupied_by_piece(A8, WHITE_QUEEN));
    assert!(!new_position.is_occupied_by_piece(A7, WHITE_PAWN));
}
#[test]
fn test_en_passant() {
    let position = Position::default()
        .put_piece(WHITE_PAWN, D4)
        .put_piece(BLACK_PAWN, E4);

    let new_position = en_passant(&position, BLACK_PAWN, E4, D3, D4);
    assert!(!new_position.is_occupied_by_piece(D4, WHITE_PAWN));
    assert!(!new_position.is_occupied_by_piece(E4, BLACK_PAWN));
    assert!(new_position.is_occupied_by_piece(D3, BLACK_PAWN));
}

#[test]
fn test_white_kingside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1);
    let new_position = white_kingside_castle(&position);
    assert!(new_position.is_occupied_by_piece(G1, WHITE_KING));
    assert!(new_position.is_occupied_by_piece(F1, WHITE_ROOK));
}

#[test]
fn test_white_queenside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1);
    let new_position = white_queenside_castle(&position);
    assert!(new_position.is_occupied_by_piece(C1, WHITE_KING));
    assert!(new_position.is_occupied_by_piece(D1, WHITE_ROOK));
}

#[test]
fn test_black_kingside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8);
    let new_position = black_kingside_castle(&position);
    assert!(new_position.is_occupied_by_piece(G8, BLACK_KING));
    assert!(new_position.is_occupied_by_piece(F8, BLACK_ROOK));
}

#[test]
fn test_black_queenside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8);
    let new_position = black_queenside_castle(&position);
    assert!(new_position.is_occupied_by_piece(C8, BLACK_KING));
    assert!(new_position.is_occupied_by_piece(D8, BLACK_ROOK));
}
