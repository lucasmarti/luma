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
    let new_position = progess(&position, Piece::WhiteKing, E1, E2);
    assert!(!new_position
        .position
        .is_occupied_by_piece(E1, Piece::WhiteKing));
    assert!(new_position
        .position
        .is_occupied_by_piece(E2, Piece::WhiteKing));
}

#[test]
fn test_promotion() {
    let position = Position::default().put_piece(Piece::WhitePawn, A7);
    let new_position = promote(&position, A7, A8, Piece::WhiteQueen);
    assert!(new_position
        .position
        .is_occupied_by_piece(A8, Piece::WhiteQueen));
    assert!(!new_position
        .position
        .is_occupied_by_piece(A7, Piece::WhitePawn));
}
#[test]
fn test_en_passant() {
    let position = Position::default()
        .put_piece(Piece::WhitePawn, D4)
        .put_piece(Piece::BlackPawn, E4);

    let new_position = en_passant(&position, Piece::BlackPawn, E4, D3, D4);
    assert!(!new_position
        .position
        .is_occupied_by_piece(D4, Piece::WhitePawn));
    assert!(!new_position
        .position
        .is_occupied_by_piece(E4, Piece::BlackPawn));
    assert!(new_position
        .position
        .is_occupied_by_piece(D3, Piece::BlackPawn));
}

// Tests for is_pawn_two_rows_forward function
#[test]
fn test_is_pawn_two_rows_forward_white_valid() {
    // White pawn moving from row 2 to row 4
    assert!(is_pawn_two_rows_forward(Piece::WhitePawn, E2, E4));
    assert!(is_pawn_two_rows_forward(Piece::WhitePawn, A2, A4));
    assert!(is_pawn_two_rows_forward(Piece::WhitePawn, H2, H4));
}

#[test]
fn test_is_pawn_two_rows_forward_white_invalid() {
    // White pawn not moving two rows
    assert!(!is_pawn_two_rows_forward(Piece::WhitePawn, E2, E3));
    assert!(!is_pawn_two_rows_forward(Piece::WhitePawn, E3, E4));
    assert!(!is_pawn_two_rows_forward(Piece::WhitePawn, E4, E5));
}

#[test]
fn test_is_pawn_two_rows_forward_black_valid() {
    // Black pawn moving from row 7 to row 5
    assert!(is_pawn_two_rows_forward(Piece::BlackPawn, E7, E5));
    assert!(is_pawn_two_rows_forward(Piece::BlackPawn, A7, A5));
    assert!(is_pawn_two_rows_forward(Piece::BlackPawn, H7, H5));
}

#[test]
fn test_is_pawn_two_rows_forward_black_invalid() {
    // Black pawn not moving two rows
    assert!(!is_pawn_two_rows_forward(Piece::BlackPawn, E7, E6));
    assert!(!is_pawn_two_rows_forward(Piece::BlackPawn, E6, E5));
    assert!(!is_pawn_two_rows_forward(Piece::BlackPawn, E5, E4));
}

// Tests for set_en_passant_if_necessary function
#[test]
fn test_set_en_passant_if_necessary_white_pawn_two_squares() {
    let position = Position::default();
    let new_position = set_en_passant_if_necessary(position, Piece::WhitePawn, E2, E4);
    // En passant should be set to E4 (the destination square)
    assert_eq!(new_position.get_en_passant(), Some(E4));
}

#[test]
fn test_set_en_passant_if_necessary_black_pawn_two_squares() {
    let position = Position::default();
    let new_position = set_en_passant_if_necessary(position, Piece::BlackPawn, E7, E5);
    // En passant should be set to E5 (the destination square)
    assert_eq!(new_position.get_en_passant(), Some(E5));
}

#[test]
fn test_set_en_passant_if_necessary_white_pawn_one_square() {
    let position = Position::default();
    let new_position = set_en_passant_if_necessary(position, Piece::WhitePawn, E2, E3);
    // En passant should not be set for one square moves
    assert_eq!(new_position.get_en_passant(), None);
}

#[test]
fn test_set_en_passant_if_necessary_non_pawn() {
    let position = Position::default();
    let new_position = set_en_passant_if_necessary(position, Piece::WhiteKing, E1, E2);
    // En passant should not be set for non-pawn moves
    assert_eq!(new_position.get_en_passant(), None);
}

// Tests for disallow_castling_if_necessary function
#[test]
fn test_disallow_castling_if_necessary_white_king_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, E1);
    assert!(!new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(!new_position.get_castling_right(CastlingType::WhiteQueenside));
    // Black castling rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
}

#[test]
fn test_disallow_castling_if_necessary_white_kingside_rook_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, H1);
    assert!(!new_position.get_castling_right(CastlingType::WhiteKingside));
    // White queenside should remain allowed
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
    // Black castling rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
}

#[test]
fn test_disallow_castling_if_necessary_white_queenside_rook_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, A1);
    assert!(!new_position.get_castling_right(CastlingType::WhiteQueenside));
    // White kingside should remain allowed
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    // Black castling rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
}

#[test]
fn test_disallow_castling_if_necessary_black_king_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, E8);
    assert!(!new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(!new_position.get_castling_right(CastlingType::BlackQueenside));
    // White castling rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_disallow_castling_if_necessary_black_kingside_rook_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, H8);
    assert!(!new_position.get_castling_right(CastlingType::BlackKingside));
    // Black queenside should remain allowed
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
    // White castling rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_disallow_castling_if_necessary_black_queenside_rook_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, A8);
    assert!(!new_position.get_castling_right(CastlingType::BlackQueenside));
    // Black kingside should remain allowed
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    // White castling rights should remain unchanged
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_disallow_castling_if_necessary_other_piece_move() {
    let position = Position::default();
    let new_position = remove_castling_rights_if_necessary(position, D4);
    // Moving a piece from D4 should not affect castling rights
    assert!(new_position.get_castling_right(CastlingType::WhiteKingside));
    assert!(new_position.get_castling_right(CastlingType::WhiteQueenside));
    assert!(new_position.get_castling_right(CastlingType::BlackKingside));
    assert!(new_position.get_castling_right(CastlingType::BlackQueenside));
}

// Integration tests for progess function
#[test]
fn test_progess_toggles_player() {
    let position = Position::default(); // White to move
    let new_position = progess(&position, Piece::WhitePawn, E2, E3);
    assert_eq!(new_position.position.get_player(), Color::Black);
}

#[test]
fn test_progess_sets_en_passant_for_pawn_two_squares() {
    let position = Position::default();
    let new_position = progess(&position, Piece::WhitePawn, E2, E4);
    assert_eq!(new_position.position.get_en_passant(), Some(E4));
}

#[test]
fn test_progess_disallows_castling_for_king_move() {
    let position = Position::default();
    let new_position = progess(&position, Piece::WhiteKing, E1, E2);
    assert!(!new_position
        .position
        .get_castling_right(CastlingType::WhiteKingside));
    assert!(!new_position
        .position
        .get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_progess_disallows_castling_for_rook_move() {
    let position = Position::default();
    let new_position = progess(&position, Piece::WhiteRook, H1, H2);
    assert!(!new_position
        .position
        .get_castling_right(CastlingType::WhiteKingside));
    // Queenside should still be allowed
    assert!(new_position
        .position
        .get_castling_right(CastlingType::WhiteQueenside));
}

#[test]
fn test_all_moves_from_starting_position() {
    let position = Position::new_starting_position();
    assert_eq!(get_white_moves(&position).len(), 20);
}
#[test]
fn test_position1() {
    let white = Position::default()
        .put_piece(Piece::WhitePawn, A2)
        .put_piece(Piece::WhitePawn, C3)
        .put_piece(Piece::WhitePawn, C4)
        .put_piece(Piece::WhiteQueen, D3)
        .put_piece(Piece::WhiteRook, F1)
        .put_piece(Piece::WhitePawn, F2)
        .put_piece(Piece::WhiteKnight, F3)
        .put_piece(Piece::WhiteKing, G1)
        .put_piece(Piece::WhiteBishop, G2)
        .put_piece(Piece::WhitePawn, G3)
        .put_piece(Piece::WhitePawn, H2);
    let positions = get_white_moves(&white);
    assert_eq!(positions.len(), 35);
    let black = Position::default()
        .put_piece(Piece::BlackPawn, A7)
        .put_piece(Piece::BlackPawn, B7)
        .put_piece(Piece::BlackPawn, D6)
        .put_piece(Piece::BlackPawn, F7)
        .put_piece(Piece::BlackPawn, G7)
        .put_piece(Piece::BlackPawn, H6)
        .put_piece(Piece::BlackBishop, B6)
        .put_piece(Piece::BlackRook, E4)
        .put_piece(Piece::BlackBishop, G4)
        .put_piece(Piece::BlackQueen, G6)
        .put_piece(Piece::BlackKing, G8)
        .toggle_player();
    let positions = get_black_moves(&black);
    assert_eq!(positions.len(), 44);
    let mut all = Position::default()
        .put_piece(Piece::WhitePawn, A2)
        .put_piece(Piece::WhitePawn, C3)
        .put_piece(Piece::WhitePawn, C4)
        .put_piece(Piece::WhiteQueen, D3)
        .put_piece(Piece::WhiteRook, F1)
        .put_piece(Piece::WhitePawn, F2)
        .put_piece(Piece::WhiteKnight, F3)
        .put_piece(Piece::WhiteKing, G1)
        .put_piece(Piece::WhiteBishop, G2)
        .put_piece(Piece::WhitePawn, G3)
        .put_piece(Piece::WhitePawn, H2)
        .put_piece(Piece::BlackPawn, A7)
        .put_piece(Piece::BlackPawn, B7)
        .put_piece(Piece::BlackPawn, D6)
        .put_piece(Piece::BlackPawn, F7)
        .put_piece(Piece::BlackPawn, G7)
        .put_piece(Piece::BlackPawn, H6)
        .put_piece(Piece::BlackBishop, B6)
        .put_piece(Piece::BlackRook, E4)
        .put_piece(Piece::BlackBishop, G4)
        .put_piece(Piece::BlackQueen, G6)
        .put_piece(Piece::BlackKing, G8);

    assert_eq!(get_white_moves(&all).len(), 29);
    all = all.toggle_player();
    assert_eq!(get_black_moves(&all).len(), 39);
}

#[test]
fn test_bishop_white_moves() {
    let position = Position::default().put_piece(Piece::WhiteBishop, G4);
    let positions = get_moves_for_bishop_at_square(&position, Piece::WhiteBishop, G4);
    assert_eq!(positions.len(), 9);
    let mut left_up = false;
    let mut left_down = false;
    let mut right_up = false;
    let mut right_down = false;
    let mut not_valid = false;

    for position in positions {
        if position
            .position
            .is_occupied_by_piece(F5, Piece::WhiteBishop)
        {
            left_up = true;
        }
        if position
            .position
            .is_occupied_by_piece(F3, Piece::WhiteBishop)
        {
            left_down = true;
        }
        if position
            .position
            .is_occupied_by_piece(H5, Piece::WhiteBishop)
        {
            right_up = true;
        }
        if position
            .position
            .is_occupied_by_piece(H3, Piece::WhiteBishop)
        {
            right_down = true;
        }
        if position
            .position
            .is_occupied_by_piece(B2, Piece::WhiteBishop)
        {
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
    assert!(get_moves_for_king_at_square(&position, Piece::BlackKing, D8).len() == 0);
    assert!(get_moves_for_king_at_square(&position, Piece::BlackKing, D3).len() == 8);
    assert!(get_moves_for_king_at_square(&position, Piece::BlackKing, F6).len() == 5);
    assert!(get_moves_for_king_at_square(&position, Piece::BlackKing, H6).len() == 3);
}

#[test]
fn test_king_white_moves() {
    let position: Position = Position::new_starting_position();

    assert!(get_moves_for_king_at_square(&position, Piece::WhiteKing, D8).len() == 5);
    assert!(get_moves_for_king_at_square(&position, Piece::WhiteKing, D3).len() == 5);
    assert!(get_moves_for_king_at_square(&position, Piece::WhiteKing, F2).len() == 3);
    assert!(get_moves_for_king_at_square(&position, Piece::WhiteKing, H6).len() == 5);
}

#[test]
fn test_knight_moves() {
    let mut position = Position::default();
    position = position
        .put_piece(Piece::WhiteKnight, E4)
        .put_piece(Piece::WhitePawn, C5)
        .put_piece(Piece::BlackPawn, G2);

    let positions = get_moves_for_knight_at_square(&position, Piece::WhiteKnight, E4);

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
        if position
            .position
            .is_occupied_by_piece(C3, Piece::WhiteKnight)
        {
            found_c3 = true;
        }
        if position
            .position
            .is_occupied_by_piece(D6, Piece::WhiteKnight)
        {
            found_d6 = true;
        }
        if position
            .position
            .is_occupied_by_piece(F6, Piece::WhiteKnight)
        {
            found_f6 = true;
        }
        if position
            .position
            .is_occupied_by_piece(D2, Piece::WhiteKnight)
        {
            found_d2 = true;
        }
        if position
            .position
            .is_occupied_by_piece(F2, Piece::WhiteKnight)
        {
            found_f2 = true;
        }
        if position
            .position
            .is_occupied_by_piece(G3, Piece::WhiteKnight)
        {
            found_g3 = true;
        }
        if position
            .position
            .is_occupied_by_piece(G5, Piece::WhiteKnight)
        {
            found_g5 = true;
        }
        if position
            .position
            .is_occupied_by_piece(C5, Piece::WhiteKnight)
        {
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
    let positions = get_moves_for_queen_at_square(&Position::default(), Piece::WhiteQueen, G4);
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
        if position
            .position
            .is_occupied_by_piece(G3, Piece::WhiteQueen)
        {
            found_down = true;
        }
        if position
            .position
            .is_occupied_by_piece(G8, Piece::WhiteQueen)
        {
            found_up = true;
        }
        if position
            .position
            .is_occupied_by_piece(A4, Piece::WhiteQueen)
        {
            found_left = true;
        }
        if position
            .position
            .is_occupied_by_piece(H4, Piece::WhiteQueen)
        {
            found_right = true;
        }
        if position
            .position
            .is_occupied_by_piece(B2, Piece::WhiteQueen)
        {
            found_not = false;
        }
        if position
            .position
            .is_occupied_by_piece(F5, Piece::WhiteQueen)
        {
            found_left_up = true;
        }
        if position
            .position
            .is_occupied_by_piece(F3, Piece::WhiteQueen)
        {
            found_left_down = true;
        }
        if position
            .position
            .is_occupied_by_piece(H5, Piece::WhiteQueen)
        {
            found_right_up = true;
        }
        if position
            .position
            .is_occupied_by_piece(H3, Piece::WhiteQueen)
        {
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
    let positions = get_moves_for_rook_at_square(&Position::default(), Piece::BlackRook, G4);

    assert!(positions.len() == 14);
    let mut found_up = false;
    let mut found_down = false;
    let mut found_left = false;
    let mut found_right = false;
    let mut found_not = true;

    for position in positions {
        if position.position.is_occupied_by_piece(G3, Piece::BlackRook) {
            found_down = true;
        }
        if position.position.is_occupied_by_piece(G8, Piece::BlackRook) {
            found_up = true;
        }
        if position.position.is_occupied_by_piece(A4, Piece::BlackRook) {
            found_left = true;
        }
        if position.position.is_occupied_by_piece(H4, Piece::BlackRook) {
            found_right = true;
        }
        if position.position.is_occupied_by_piece(B2, Piece::BlackRook) {
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
