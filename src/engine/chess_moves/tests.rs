use crate::engine::{
    chess_moves::{
        castling::remove_castling_rights_if_necessary,
        common::{
            get_moves_for_bishop_at_square, get_moves_for_king_at_square,
            get_moves_for_knight_at_square, get_moves_for_queen_at_square,
            get_moves_for_rook_at_square, progess,
        },
        get_black_moves, get_white_moves,
        pawn::{en_passant, is_pawn_two_rows_forward, promote, set_en_passant_if_necessary},
    },
    directions::squares::*,
    piece::*,
    position::{CastlingType, Position},
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

// Tests for is_pawn_two_rows_forward function
#[test]
fn test_is_pawn_two_rows_forward_white_valid() {
    // White pawn moving from row 2 to row 4
    assert!(is_pawn_two_rows_forward(WHITE_PAWN, E2, E4));
    assert!(is_pawn_two_rows_forward(WHITE_PAWN, A2, A4));
    assert!(is_pawn_two_rows_forward(WHITE_PAWN, H2, H4));
}

#[test]
fn test_is_pawn_two_rows_forward_white_invalid() {
    // White pawn not moving two rows
    assert!(!is_pawn_two_rows_forward(WHITE_PAWN, E2, E3));
    assert!(!is_pawn_two_rows_forward(WHITE_PAWN, E3, E4));
    assert!(!is_pawn_two_rows_forward(WHITE_PAWN, E4, E5));
}

#[test]
fn test_is_pawn_two_rows_forward_black_valid() {
    // Black pawn moving from row 7 to row 5
    assert!(is_pawn_two_rows_forward(BLACK_PAWN, E7, E5));
    assert!(is_pawn_two_rows_forward(BLACK_PAWN, A7, A5));
    assert!(is_pawn_two_rows_forward(BLACK_PAWN, H7, H5));
}

#[test]
fn test_is_pawn_two_rows_forward_black_invalid() {
    // Black pawn not moving two rows
    assert!(!is_pawn_two_rows_forward(BLACK_PAWN, E7, E6));
    assert!(!is_pawn_two_rows_forward(BLACK_PAWN, E6, E5));
    assert!(!is_pawn_two_rows_forward(BLACK_PAWN, E5, E4));
}

// Tests for set_en_passant_if_necessary function
#[test]
fn test_set_en_passant_if_necessary_white_pawn_two_squares() {
    let position = Position::default();
    let new_position = set_en_passant_if_necessary(position, WHITE_PAWN, E2, E4);
    // En passant should be set to E4 (the destination square)
    assert_eq!(new_position.get_en_passant(), Some(E4));
}

#[test]
fn test_set_en_passant_if_necessary_black_pawn_two_squares() {
    let position = Position::default();
    let new_position = set_en_passant_if_necessary(position, BLACK_PAWN, E7, E5);
    // En passant should be set to E5 (the destination square)
    assert_eq!(new_position.get_en_passant(), Some(E5));
}

#[test]
fn test_set_en_passant_if_necessary_white_pawn_one_square() {
    let position = Position::default();
    let new_position = set_en_passant_if_necessary(position, WHITE_PAWN, E2, E3);
    // En passant should not be set for one square moves
    assert_eq!(new_position.get_en_passant(), None);
}

#[test]
fn test_set_en_passant_if_necessary_non_pawn() {
    let position = Position::default();
    let new_position = set_en_passant_if_necessary(position, WHITE_KING, E1, E2);
    // En passant should not be set for non-pawn moves
    assert_eq!(new_position.get_en_passant(), None);
}

// Tests for disallow_castle_if_necessary function
#[test]
fn test_disallow_castle_if_necessary_white_king_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, E1);
    assert!(!new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(!new_position.get_castling_right(CastlingType::WhiteQueenside));
    // Black castle rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
}

#[test]
fn test_disallow_castle_if_necessary_white_kingside_rook_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, H1);
    assert!(!new_position.get_castling_right(CastlingType::WhiteKingside));
    // White queenside should remain allowed
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
    // Black castle rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
}

#[test]
fn test_disallow_castle_if_necessary_white_queenside_rook_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, A1);
    assert!(!new_position.get_castling_right(CastlingType::WhiteQueenside));
    // White kingside should remain allowed
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    // Black castle rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
}

#[test]
fn test_disallow_castle_if_necessary_black_king_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, E8);
    assert!(!new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(!new_position.get_castling_right(CastlingType::BlackQueenside));
    // White castle rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_disallow_castle_if_necessary_black_kingside_rook_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, H8);
    assert!(!new_position.get_castling_right(CastlingType::BlackKingside));
    // Black queenside should remain allowed
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
    // White castle rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_disallow_castle_if_necessary_black_queenside_rook_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, A8);
    assert!(!new_position.get_castling_right(CastlingType::BlackQueenside));
    // Black kingside should remain allowed
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    // White castle rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_disallow_castle_if_necessary_other_piece_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, D4);
    // Moving a piece from D4 should not affect castle rights
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
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
    assert!(!new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(!new_position.get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_progess_disallows_castle_for_rook_move() {
    let position = Position::default();
    let new_position = progess(&position, WHITE_ROOK, H1, H2);
    assert!(!new_position.get_castling_right(CastlingType::WhiteKingside));
    // Queenside should still be allowed
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_all_moves_from_starting_position() {
    let position = Position::new_starting_position();
    assert_eq!(get_white_moves(&position).len(), 20);
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
    let positions = get_white_moves(&white);
    assert_eq!(positions.len(), 35);
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
        .put_piece(BLACK_KING, G8)
        .toggle_player();
    let positions = get_black_moves(&black);
    assert_eq!(positions.len(), 44);
    let mut all = Position::default()
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

    assert_eq!(get_white_moves(&all).len(), 29);
    all = all.toggle_player();
    assert_eq!(get_black_moves(&all).len(), 39);
}

#[test]
fn test_bishop_white_moves() {
    let position = Position::default().put_piece(WHITE_BISHOP, G4);
    let positions = get_moves_for_bishop_at_square(&position, WHITE_BISHOP, G4);
    assert_eq!(positions.len(), 9);
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
#[test]
fn test_king_black_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_moves_for_king_at_square(&position, BLACK_KING, D8).len() == 0);
    assert!(get_moves_for_king_at_square(&position, BLACK_KING, D3).len() == 8);
    assert!(get_moves_for_king_at_square(&position, BLACK_KING, F6).len() == 5);
    assert!(get_moves_for_king_at_square(&position, BLACK_KING, H6).len() == 3);
}

#[test]
fn test_king_white_moves() {
    let position: Position = Position::new_starting_position();

    assert!(get_moves_for_king_at_square(&position, WHITE_KING, D8).len() == 5);
    assert!(get_moves_for_king_at_square(&position, WHITE_KING, D3).len() == 5);
    assert!(get_moves_for_king_at_square(&position, WHITE_KING, F2).len() == 3);
    assert!(get_moves_for_king_at_square(&position, WHITE_KING, H6).len() == 5);
}

#[test]
fn test_knight_moves() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KNIGHT, E4)
        .put_piece(WHITE_PAWN, C5)
        .put_piece(BLACK_PAWN, G2);

    let positions = get_moves_for_knight_at_square(&position, WHITE_KNIGHT, E4);

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
#[test]
fn test_queen_white_moves() {
    let positions = get_moves_for_queen_at_square(&Position::default(), WHITE_QUEEN, G4);
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
#[test]
fn test_rook_white_moves() {
    let positions = get_moves_for_rook_at_square(&Position::default(), BLACK_ROOK, G4);

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
