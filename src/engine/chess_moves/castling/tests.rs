// White Castling Tests

use crate::engine::{
    chess_moves::castling::{
        configurations::{BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE},
        get_black_castling_moves, get_castling_move, get_white_castling_moves,
    },
    directions::squares::*,
    piece::*,
    position::*,
};

#[test]
fn test_white_kingside_castling_allowed() {
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, H1);

    let moves = get_white_castling_moves(&position);

    // Should have normal king moves plus castling
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(F1, Piece::WhiteRook)
    });
    assert!(
        castling_move.is_some(),
        "White kingside castling should be possible"
    );
}

#[test]
fn test_white_queenside_castling_allowed() {
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, A1);

    let moves = get_white_castling_moves(&position);

    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(D1, Piece::WhiteRook)
    });
    assert!(
        castling_move.is_some(),
        "White queenside castling should be possible"
    );
}

#[test]
fn test_white_castling_blocked_by_pieces() {
    // Kingside blocked
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::WhiteQueen, F1); // Blocking piece

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(F1, Piece::WhiteRook)
    });
    assert!(
        castling_move.is_none(),
        "White kingside castling should be blocked"
    );

    // Queenside blocked
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, A1)
        .put_piece(Piece::WhiteQueen, D1); // Blocking piece

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(D1, Piece::WhiteRook)
    });
    assert!(
        castling_move.is_none(),
        "White queenside castling should be blocked"
    );
}

#[test]
fn test_white_castling_not_allowed_when_in_check() {
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::BlackRook, E8); // Attacking the king

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(F1, Piece::WhiteRook)
    });
    assert!(
        castling_move.is_none(),
        "castling should not be allowed when in check"
    );
}

#[test]
fn test_white_castling_not_allowed_through_check() {
    // Kingside - F1 under attack
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::BlackRook, F8); // Attacking F1

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(F1, Piece::WhiteRook)
    });
    assert!(
        castling_move.is_none(),
        "castling should not be allowed through check"
    );

    // Queenside - D1 under attack
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, A1)
        .put_piece(Piece::BlackRook, D8); // Attacking D1

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(D1, Piece::WhiteRook)
    });
    assert!(
        castling_move.is_none(),
        "castling should not be allowed through check"
    );
}

// Black Castling Tests

#[test]
fn test_black_kingside_castling_allowed() {
    let position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::BlackRook, H8);

    let moves = get_black_castling_moves(&position);

    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G8, Piece::BlackKing)
            && pos.is_occupied_by_piece(F8, Piece::BlackRook)
    });
    assert!(
        castling_move.is_some(),
        "Black kingside castling should be possible"
    );
}

#[test]
fn test_black_queenside_castling_allowed() {
    let position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::BlackRook, A8);
    let moves = get_black_castling_moves(&position);

    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C8, Piece::BlackKing)
            && pos.is_occupied_by_piece(D8, Piece::BlackRook)
    });
    assert!(
        castling_move.is_some(),
        "Black queenside castling should be possible"
    );
}

#[test]
fn test_black_castling_blocked_by_pieces() {
    // Kingside blocked
    let position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::BlackRook, H8)
        .put_piece(Piece::BlackQueen, F8); // Blocking piece

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G8, Piece::BlackKing)
            && pos.is_occupied_by_piece(F8, Piece::BlackRook)
    });
    assert!(
        castling_move.is_none(),
        "Black kingside castling should be blocked"
    );

    // Queenside blocked
    let position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::BlackRook, A8)
        .put_piece(Piece::BlackQueen, B8); // Blocking piece

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C8, Piece::BlackKing)
            && pos.is_occupied_by_piece(D8, Piece::BlackRook)
    });
    assert!(
        castling_move.is_none(),
        "Black queenside castling should be blocked"
    );
}

#[test]
fn test_black_castling_not_allowed_when_in_check() {
    let position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::BlackRook, H8)
        .put_piece(Piece::WhiteRook, E1); // Attacking the king

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G8, Piece::BlackKing)
            && pos.is_occupied_by_piece(F8, Piece::BlackRook)
    });
    assert!(
        castling_move.is_none(),
        "castling should not be allowed when in check"
    );
}

#[test]
fn test_black_castling_not_allowed_through_check() {
    // Kingside - F8 under attack
    let position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::BlackRook, H8)
        .put_piece(Piece::WhiteRook, F1); // Attacking F8

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G8, Piece::BlackKing)
            && pos.is_occupied_by_piece(F8, Piece::BlackRook)
    });
    assert!(
        castling_move.is_none(),
        "castling should not be allowed through check"
    );

    // Queenside - D8 under attack
    let position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::BlackRook, A8)
        .put_piece(Piece::WhiteRook, D1); // Attacking D8

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C8, Piece::BlackKing)
            && pos.is_occupied_by_piece(D8, Piece::BlackRook)
    });
    assert!(
        castling_move.is_none(),
        "castling should not be allowed through check"
    );
}

#[test]
fn test_castling_rights_false() {
    // White with pieces in position but no rights
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::WhiteRook, A1)
        .remove_castling_right(CastlingType::WhiteKingside)
        .remove_castling_right(CastlingType::WhiteQueenside);

    let moves = get_white_castling_moves(&position);
    let kingside_castling = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(F1, Piece::WhiteRook)
    });
    let queenside_castling = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(D1, Piece::WhiteRook)
    });

    assert!(
        kingside_castling.is_none(),
        "Kingside castling should not be allowed without rights"
    );
    assert!(
        queenside_castling.is_none(),
        "Queenside castling should not be allowed without rights"
    );
}

#[test]
fn test_both_castlings_available() {
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::WhiteRook, A1);

    let moves = get_white_castling_moves(&position);

    let kingside_castling = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(F1, Piece::WhiteRook)
    });
    let queenside_castling = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, Piece::WhiteKing)
            && pos.is_occupied_by_piece(D1, Piece::WhiteRook)
    });

    assert!(
        kingside_castling.is_some(),
        "Kingside castling should be available"
    );
    assert!(
        queenside_castling.is_some(),
        "Queenside castling should be available"
    );
}

#[test]
fn test_white_kingside_castling() {
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, H1);
    let new_position = get_castling_move(&position, WHITE_KINGSIDE);
    assert!(new_position.is_some());
    assert!(new_position
        .unwrap()
        .is_occupied_by_piece(G1, Piece::WhiteKing));
    assert!(new_position
        .unwrap()
        .is_occupied_by_piece(F1, Piece::WhiteRook));
}

#[test]
fn test_white_queenside_castling() {
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhiteRook, A1);
    let new_position = get_castling_move(&position, WHITE_QUEENSIDE);
    assert!(new_position.is_some());
    assert!(new_position
        .unwrap()
        .is_occupied_by_piece(C1, Piece::WhiteKing));
    assert!(new_position
        .unwrap()
        .is_occupied_by_piece(D1, Piece::WhiteRook));
}

#[test]
fn test_black_kingside_castling() {
    let position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::BlackRook, H8);
    let new_position = get_castling_move(&position, BLACK_KINGSIDE);
    assert!(new_position.is_some());
    assert!(new_position
        .unwrap()
        .is_occupied_by_piece(G8, Piece::BlackKing));
    assert!(new_position
        .unwrap()
        .is_occupied_by_piece(F8, Piece::BlackRook));
}

#[test]
fn test_black_queenside_castling() {
    let position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::BlackRook, A8);
    let new_position = get_castling_move(&position, BLACK_QUEENSIDE);
    assert!(new_position.is_some());
    assert!(new_position
        .unwrap()
        .is_occupied_by_piece(C8, Piece::BlackKing));
    assert!(new_position
        .unwrap()
        .is_occupied_by_piece(D8, Piece::BlackRook));
}

// Integration tests for castling methods
#[test]
fn test_castling_methods_toggle_player() {
    let position = Position::default()
        .put_piece(Piece::WhiteRook, A1)
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::BlackRook, A8)
        .put_piece(Piece::BlackRook, H8)
        .put_piece(Piece::BlackKing, E8);
    // White to move

    let white_kingside = get_castling_move(&position, WHITE_KINGSIDE);
    assert_eq!(white_kingside.unwrap().get_player(), Color::Black);

    let white_queenside = get_castling_move(&position, WHITE_QUEENSIDE);
    assert_eq!(white_queenside.unwrap().get_player(), Color::Black);

    let position_black = position.toggle_player(); // Black to move
    let black_kingside = get_castling_move(&position_black, BLACK_KINGSIDE);
    assert_eq!(black_kingside.unwrap().get_player(), Color::White);

    let black_queenside = get_castling_move(&position_black, BLACK_QUEENSIDE);
    assert_eq!(black_queenside.unwrap().get_player(), Color::White);
}

#[test]
fn test_castling_methods_reset_en_passant() {
    let position = Position::default()
        .put_piece(Piece::WhiteRook, A1)
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::BlackRook, A8)
        .put_piece(Piece::BlackRook, H8)
        .put_piece(Piece::BlackKing, E8)
        .set_en_passant(E4);

    let white_kingside = get_castling_move(&position, WHITE_KINGSIDE);
    assert_eq!(white_kingside.unwrap().get_en_passant(), None);

    let white_queenside = get_castling_move(&position, WHITE_QUEENSIDE);
    assert_eq!(white_queenside.unwrap().get_en_passant(), None);

    let black_kingside = get_castling_move(&position, BLACK_KINGSIDE);
    assert_eq!(black_kingside.unwrap().get_en_passant(), None);

    let black_queenside = get_castling_move(&position, BLACK_QUEENSIDE);
    assert_eq!(black_queenside.unwrap().get_en_passant(), None);
}

#[test]
fn test_castling_methods_disable_castling_rights() {
    let position = Position::default()
        .put_piece(Piece::WhiteRook, A1)
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::BlackRook, A8)
        .put_piece(Piece::BlackRook, H8)
        .put_piece(Piece::BlackKing, E8);

    let white_kingside = get_castling_move(&position, WHITE_KINGSIDE);
    assert!(!white_kingside
        .unwrap()
        .get_castling_right(CastlingType::WhiteKingside));
    assert!(!white_kingside
        .unwrap()
        .get_castling_right(CastlingType::WhiteQueenside));

    let white_queenside = get_castling_move(&position, WHITE_QUEENSIDE);
    assert!(!white_queenside
        .unwrap()
        .get_castling_right(CastlingType::WhiteKingside));
    assert!(!white_queenside
        .unwrap()
        .get_castling_right(CastlingType::WhiteQueenside));

    let black_kingside = get_castling_move(&position, BLACK_KINGSIDE);
    assert!(!black_kingside
        .unwrap()
        .get_castling_right(CastlingType::BlackKingside));
    assert!(!black_kingside
        .unwrap()
        .get_castling_right(CastlingType::BlackQueenside));

    let black_queenside = get_castling_move(&position, BLACK_QUEENSIDE);
    assert!(!black_queenside
        .unwrap()
        .get_castling_right(CastlingType::BlackKingside));
    assert!(!black_queenside
        .unwrap()
        .get_castling_right(CastlingType::BlackQueenside));
}
