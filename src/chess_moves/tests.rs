use crate::{
    chess_moves::{
        self, black_kingside_castle, black_queenside_castle, en_passant, progess, promote,
        white_kingside_castle, white_queenside_castle,
    },
    directions::{self, *},
    piece::*,
    position::{self, Position},
    possible_moves::king::disallow_castle_if_necessary,
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
    assert!(new_position.is_some());
    assert!(new_position.unwrap().is_occupied_by_piece(G1, WHITE_KING));
    assert!(new_position.unwrap().is_occupied_by_piece(F1, WHITE_ROOK));
}

#[test]
fn test_white_queenside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1);
    let new_position = white_queenside_castle(&position);
    assert!(new_position.is_some());
    assert!(new_position.unwrap().is_occupied_by_piece(C1, WHITE_KING));
    assert!(new_position.unwrap().is_occupied_by_piece(D1, WHITE_ROOK));
}

#[test]
fn test_black_kingside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8);
    let new_position = black_kingside_castle(&position);
    assert!(new_position.is_some());
    assert!(new_position.unwrap().is_occupied_by_piece(G8, BLACK_KING));
    assert!(new_position.unwrap().is_occupied_by_piece(F8, BLACK_ROOK));
}

#[test]
fn test_black_queenside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8);
    let new_position = black_queenside_castle(&position);
    assert!(new_position.is_some());
    assert!(new_position.unwrap().is_occupied_by_piece(C8, BLACK_KING));
    assert!(new_position.unwrap().is_occupied_by_piece(D8, BLACK_ROOK));
}

// Tests for is_pawn_two_rows_forward function
#[test]
fn test_is_pawn_two_rows_forward_white_valid() {
    // White pawn moving from row 2 to row 4
    assert!(chess_moves::is_pawn_two_rows_forward(WHITE_PAWN, E2, E4));
    assert!(chess_moves::is_pawn_two_rows_forward(WHITE_PAWN, A2, A4));
    assert!(chess_moves::is_pawn_two_rows_forward(WHITE_PAWN, H2, H4));
}

#[test]
fn test_is_pawn_two_rows_forward_white_invalid() {
    // White pawn not moving two rows
    assert!(!chess_moves::is_pawn_two_rows_forward(WHITE_PAWN, E2, E3));
    assert!(!chess_moves::is_pawn_two_rows_forward(WHITE_PAWN, E3, E4));
    assert!(!chess_moves::is_pawn_two_rows_forward(WHITE_PAWN, E4, E5));
}

#[test]
fn test_is_pawn_two_rows_forward_black_valid() {
    // Black pawn moving from row 7 to row 5
    assert!(chess_moves::is_pawn_two_rows_forward(BLACK_PAWN, E7, E5));
    assert!(chess_moves::is_pawn_two_rows_forward(BLACK_PAWN, A7, A5));
    assert!(chess_moves::is_pawn_two_rows_forward(BLACK_PAWN, H7, H5));
}

#[test]
fn test_is_pawn_two_rows_forward_black_invalid() {
    // Black pawn not moving two rows
    assert!(!chess_moves::is_pawn_two_rows_forward(BLACK_PAWN, E7, E6));
    assert!(!chess_moves::is_pawn_two_rows_forward(BLACK_PAWN, E6, E5));
    assert!(!chess_moves::is_pawn_two_rows_forward(BLACK_PAWN, E5, E4));
}

// Tests for set_en_passant_if_necessary function
#[test]
fn test_set_en_passant_if_necessary_white_pawn_two_squares() {
    let position = Position::default();
    let new_position = chess_moves::set_en_passant_if_necessary(position, WHITE_PAWN, E2, E4);
    // En passant should be set to E4 (the destination square)
    assert_eq!(new_position.get_en_passant(), Some(E4));
}

#[test]
fn test_set_en_passant_if_necessary_black_pawn_two_squares() {
    let position = Position::default();
    let new_position = chess_moves::set_en_passant_if_necessary(position, BLACK_PAWN, E7, E5);
    // En passant should be set to E5 (the destination square)
    assert_eq!(new_position.get_en_passant(), Some(E5));
}

#[test]
fn test_set_en_passant_if_necessary_white_pawn_one_square() {
    let position = Position::default();
    let new_position = chess_moves::set_en_passant_if_necessary(position, WHITE_PAWN, E2, E3);
    // En passant should not be set for one square moves
    assert_eq!(new_position.get_en_passant(), None);
}

#[test]
fn test_set_en_passant_if_necessary_non_pawn() {
    let position = Position::default();
    let new_position = chess_moves::set_en_passant_if_necessary(position, WHITE_KING, E1, E2);
    // En passant should not be set for non-pawn moves
    assert_eq!(new_position.get_en_passant(), None);
}

// Tests for disallow_castle_if_necessary function
#[test]
fn test_disallow_castle_if_necessary_white_king_move() {
    let position = Position::default();
    let new_position = disallow_castle_if_necessary(position, E1);
    assert!(!new_position.is_white_kingside_castle_allowed());
    assert!(!new_position.is_white_queenside_castle_allowed());
    // Black castle rights should remain unchanged
    assert!(new_position.is_black_kingside_castle_allowed());
    assert!(new_position.is_black_queenside_castle_allowed());
}

#[test]
fn test_disallow_castle_if_necessary_white_kingside_rook_move() {
    let position = Position::default();
    let new_position = disallow_castle_if_necessary(position, H1);
    assert!(!new_position.is_white_kingside_castle_allowed());
    // White queenside should remain allowed
    assert!(new_position.is_white_queenside_castle_allowed());
    // Black castle rights should remain unchanged
    assert!(new_position.is_black_kingside_castle_allowed());
    assert!(new_position.is_black_queenside_castle_allowed());
}

#[test]
fn test_disallow_castle_if_necessary_white_queenside_rook_move() {
    let position = Position::default();
    let new_position = disallow_castle_if_necessary(position, A1);
    assert!(!new_position.is_white_queenside_castle_allowed());
    // White kingside should remain allowed
    assert!(new_position.is_white_kingside_castle_allowed());
    // Black castle rights should remain unchanged
    assert!(new_position.is_black_kingside_castle_allowed());
    assert!(new_position.is_black_queenside_castle_allowed());
}

#[test]
fn test_disallow_castle_if_necessary_black_king_move() {
    let position = Position::default();
    let new_position = disallow_castle_if_necessary(position, E8);
    assert!(!new_position.is_black_kingside_castle_allowed());
    assert!(!new_position.is_black_queenside_castle_allowed());
    // White castle rights should remain unchanged
    assert!(new_position.is_white_kingside_castle_allowed());
    assert!(new_position.is_white_queenside_castle_allowed());
}

#[test]
fn test_disallow_castle_if_necessary_black_kingside_rook_move() {
    let position = Position::default();
    let new_position = disallow_castle_if_necessary(position, H8);
    assert!(!new_position.is_black_kingside_castle_allowed());
    // Black queenside should remain allowed
    assert!(new_position.is_black_queenside_castle_allowed());
    // White castle rights should remain unchanged
    assert!(new_position.is_white_kingside_castle_allowed());
    assert!(new_position.is_white_queenside_castle_allowed());
}

#[test]
fn test_disallow_castle_if_necessary_black_queenside_rook_move() {
    let position = Position::default();
    let new_position = disallow_castle_if_necessary(position, A8);
    assert!(!new_position.is_black_queenside_castle_allowed());
    // Black kingside should remain allowed
    assert!(new_position.is_black_kingside_castle_allowed());
    // White castle rights should remain unchanged
    assert!(new_position.is_white_kingside_castle_allowed());
    assert!(new_position.is_white_queenside_castle_allowed());
}

#[test]
fn test_disallow_castle_if_necessary_other_piece_move() {
    let position = Position::default();
    let new_position = disallow_castle_if_necessary(position, D4);
    // Moving a piece from D4 should not affect castle rights
    assert!(new_position.is_white_kingside_castle_allowed());
    assert!(new_position.is_white_queenside_castle_allowed());
    assert!(new_position.is_black_kingside_castle_allowed());
    assert!(new_position.is_black_queenside_castle_allowed());
}

// Integration tests for progess function
#[test]
fn test_progess_toggles_player() {
    let position = Position::default(); // White to move
    let new_position = progess(&position, WHITE_PAWN, E2, E3);
    assert_eq!(new_position.get_player(), Color::Black);
}

#[test]
fn test_progess_sets_en_passant_for_pawn_two_squares() {
    let position = Position::default();
    let new_position = progess(&position, WHITE_PAWN, E2, E4);
    assert_eq!(new_position.get_en_passant(), Some(E4));
}

#[test]
fn test_progess_disallows_castle_for_king_move() {
    let position = Position::default();
    let new_position = progess(&position, WHITE_KING, E1, E2);
    assert!(!new_position.is_white_kingside_castle_allowed());
    assert!(!new_position.is_white_queenside_castle_allowed());
}

#[test]
fn test_progess_disallows_castle_for_rook_move() {
    let position = Position::default();
    let new_position = progess(&position, WHITE_ROOK, H1, H2);
    assert!(!new_position.is_white_kingside_castle_allowed());
    // Queenside should still be allowed
    assert!(new_position.is_white_queenside_castle_allowed());
}

// Integration tests for castle methods
#[test]
fn test_castle_methods_toggle_player() {
    let position = Position::default()
        .put_piece(WHITE_ROOK, A1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_ROOK, A8)
        .put_piece(BLACK_ROOK, H8)
        .put_piece(BLACK_KING, E8);
    // White to move

    let white_kingside = white_kingside_castle(&position);
    assert_eq!(white_kingside.unwrap().get_player(), Color::Black);

    let white_queenside = white_queenside_castle(&position);
    assert_eq!(white_queenside.unwrap().get_player(), Color::Black);

    let position_black = position.toggle_player(); // Black to move
    let black_kingside = black_kingside_castle(&position_black);
    assert_eq!(black_kingside.unwrap().get_player(), Color::White);

    let black_queenside = black_queenside_castle(&position_black);
    assert_eq!(black_queenside.unwrap().get_player(), Color::White);
}

#[test]
fn test_castle_methods_reset_en_passant() {
    let position = Position::default()
        .put_piece(WHITE_ROOK, A1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_ROOK, A8)
        .put_piece(BLACK_ROOK, H8)
        .put_piece(BLACK_KING, E8)
        .set_en_passant(E4);

    let white_kingside = white_kingside_castle(&position);
    assert_eq!(white_kingside.unwrap().get_en_passant(), None);

    let white_queenside = white_queenside_castle(&position);
    assert_eq!(white_queenside.unwrap().get_en_passant(), None);

    let black_kingside = black_kingside_castle(&position);
    assert_eq!(black_kingside.unwrap().get_en_passant(), None);

    let black_queenside = black_queenside_castle(&position);
    assert_eq!(black_queenside.unwrap().get_en_passant(), None);
}

#[test]
fn test_castle_methods_disable_castle_rights() {
    let position = Position::default()
        .put_piece(WHITE_ROOK, A1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_ROOK, A8)
        .put_piece(BLACK_ROOK, H8)
        .put_piece(BLACK_KING, E8);

    let white_kingside = white_kingside_castle(&position);
    assert!(!white_kingside.unwrap().is_white_kingside_castle_allowed());
    assert!(!white_kingside.unwrap().is_white_queenside_castle_allowed());

    let white_queenside = white_queenside_castle(&position);
    assert!(!white_queenside.unwrap().is_white_kingside_castle_allowed());
    assert!(!white_queenside.unwrap().is_white_queenside_castle_allowed());

    let black_kingside = black_kingside_castle(&position);
    assert!(!black_kingside.unwrap().is_black_kingside_castle_allowed());
    assert!(!black_kingside.unwrap().is_black_queenside_castle_allowed());

    let black_queenside = black_queenside_castle(&position);
    assert!(!black_queenside.unwrap().is_black_kingside_castle_allowed());
    assert!(!black_queenside.unwrap().is_black_queenside_castle_allowed());
}
