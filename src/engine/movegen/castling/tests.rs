// White Castling Tests

use crate::engine::{
    movegen::{
        castling::{get_black_castling_moves, get_castling_move, get_white_castling_moves},
        castling_config::{BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE},
        *,
    },
    piece::*,
    position::*,
};

#[test]
fn test_white_kingside_castling_allowed() {
    let position = Position::default()
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, H1);

    let moves = get_white_castling_moves(&position);

    // Should have normal king moves plus castling
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G1, WHITE_KING)
            && c.position.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    assert!(
        castling_move.is_some(),
        "White kingside castling should be possible"
    );
}

#[test]
fn test_white_queenside_castling_allowed() {
    let position = Position::default()
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, A1);

    let moves = get_white_castling_moves(&position);

    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(C1, WHITE_KING)
            && c.position.is_occupied_by_piece(D1, WHITE_ROOK)
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
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, H1)
        .with_piece(WHITE_QUEEN, F1); // Blocking piece

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G1, WHITE_KING)
            && c.position.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    assert!(
        castling_move.is_none(),
        "White kingside castling should be blocked"
    );

    // Queenside blocked
    let position = Position::default()
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, A1)
        .with_piece(WHITE_QUEEN, D1); // Blocking piece

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(C1, WHITE_KING)
            && c.position.is_occupied_by_piece(D1, WHITE_ROOK)
    });
    assert!(
        castling_move.is_none(),
        "White queenside castling should be blocked"
    );
}

#[test]
fn test_white_castling_not_allowed_when_in_check() {
    let position = Position::default()
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, H1)
        .with_piece(BLACK_ROOK, E8); // Attacking the king

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G1, WHITE_KING)
            && c.position.is_occupied_by_piece(F1, WHITE_ROOK)
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
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, H1)
        .with_piece(BLACK_ROOK, F8); // Attacking F1

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G1, WHITE_KING)
            && c.position.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    assert!(
        castling_move.is_none(),
        "castling should not be allowed through check"
    );

    // Queenside - D1 under attack
    let position = Position::default()
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, A1)
        .with_piece(BLACK_ROOK, D8); // Attacking D1

    let moves = get_white_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(C1, WHITE_KING)
            && c.position.is_occupied_by_piece(D1, WHITE_ROOK)
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
        .with_piece(BLACK_KING, E8)
        .with_piece(BLACK_ROOK, H8);

    let moves = get_black_castling_moves(&position);

    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G8, BLACK_KING)
            && c.position.is_occupied_by_piece(F8, BLACK_ROOK)
    });
    assert!(
        castling_move.is_some(),
        "Black kingside castling should be possible"
    );
}

#[test]
fn test_black_queenside_castling_allowed() {
    let position = Position::default()
        .with_piece(BLACK_KING, E8)
        .with_piece(BLACK_ROOK, A8);
    let moves = get_black_castling_moves(&position);

    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(C8, BLACK_KING)
            && c.position.is_occupied_by_piece(D8, BLACK_ROOK)
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
        .with_piece(BLACK_KING, E8)
        .with_piece(BLACK_ROOK, H8)
        .with_piece(BLACK_QUEEN, F8); // Blocking piece

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G8, BLACK_KING)
            && c.position.is_occupied_by_piece(F8, BLACK_ROOK)
    });
    assert!(
        castling_move.is_none(),
        "Black kingside castling should be blocked"
    );

    // Queenside blocked
    let position = Position::default()
        .with_piece(BLACK_KING, E8)
        .with_piece(BLACK_ROOK, A8)
        .with_piece(BLACK_QUEEN, B8); // Blocking piece

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(C8, BLACK_KING)
            && c.position.is_occupied_by_piece(D8, BLACK_ROOK)
    });
    assert!(
        castling_move.is_none(),
        "Black queenside castling should be blocked"
    );
}

#[test]
fn test_black_castling_not_allowed_when_in_check() {
    let position = Position::default()
        .with_piece(BLACK_KING, E8)
        .with_piece(BLACK_ROOK, H8)
        .with_piece(WHITE_ROOK, E1); // Attacking the king

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G8, BLACK_KING)
            && c.position.is_occupied_by_piece(F8, BLACK_ROOK)
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
        .with_piece(BLACK_KING, E8)
        .with_piece(BLACK_ROOK, H8)
        .with_piece(WHITE_ROOK, F1); // Attacking F8

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G8, BLACK_KING)
            && c.position.is_occupied_by_piece(F8, BLACK_ROOK)
    });
    assert!(
        castling_move.is_none(),
        "castling should not be allowed through check"
    );

    // Queenside - D8 under attack
    let position = Position::default()
        .with_piece(BLACK_KING, E8)
        .with_piece(BLACK_ROOK, A8)
        .with_piece(WHITE_ROOK, D1); // Attacking D8

    let moves = get_black_castling_moves(&position);
    let castling_move = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(C8, BLACK_KING)
            && c.position.is_occupied_by_piece(D8, BLACK_ROOK)
    });
    assert!(
        castling_move.is_none(),
        "castling should not be allowed through check"
    );
}

#[test]
fn test_castling_rights_false() {
    // White with pieces in position but no rights
    let mut position = Position::default()
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, H1)
        .with_piece(WHITE_ROOK, A1);
    position.remove_castling_right(CastlingRights::WHITE_KINGSIDE);
    position.remove_castling_right(CastlingRights::WHITE_QUEENSIDE);

    let moves = get_white_castling_moves(&position);
    let kingside_castling = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G1, WHITE_KING)
            && c.position.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    let queenside_castling = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(C1, WHITE_KING)
            && c.position.is_occupied_by_piece(D1, WHITE_ROOK)
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
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, H1)
        .with_piece(WHITE_ROOK, A1);

    let moves = get_white_castling_moves(&position);

    let kingside_castling = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(G1, WHITE_KING)
            && c.position.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    let queenside_castling = moves.iter().find(|c| {
        c.position.is_occupied_by_piece(C1, WHITE_KING)
            && c.position.is_occupied_by_piece(D1, WHITE_ROOK)
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
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, H1);
    let c = get_castling_move(&position, WHITE_KINGSIDE);
    assert!(c.is_some());
    assert!(c.unwrap().position.is_occupied_by_piece(G1, WHITE_KING));
    assert!(c.unwrap().position.is_occupied_by_piece(F1, WHITE_ROOK));
}

#[test]
fn test_white_queenside_castling() {
    let position = Position::default()
        .with_piece(WHITE_KING, E1)
        .with_piece(WHITE_ROOK, A1);
    let c = get_castling_move(&position, WHITE_QUEENSIDE);
    assert!(c.is_some());
    assert!(c.unwrap().position.is_occupied_by_piece(C1, WHITE_KING));
    assert!(c.unwrap().position.is_occupied_by_piece(D1, WHITE_ROOK));
}

#[test]
fn test_black_kingside_castling() {
    let position = Position::default()
        .with_piece(BLACK_KING, E8)
        .with_piece(BLACK_ROOK, H8);
    let c = get_castling_move(&position, BLACK_KINGSIDE);
    assert!(c.is_some());
    assert!(c.unwrap().position.is_occupied_by_piece(G8, BLACK_KING));
    assert!(c.unwrap().position.is_occupied_by_piece(F8, BLACK_ROOK));
}

#[test]
fn test_black_queenside_castling() {
    let position = Position::default()
        .with_piece(BLACK_KING, E8)
        .with_piece(BLACK_ROOK, A8);
    let c = get_castling_move(&position, BLACK_QUEENSIDE);
    assert!(c.is_some());
    assert!(c.unwrap().position.is_occupied_by_piece(C8, BLACK_KING));
    assert!(c.unwrap().position.is_occupied_by_piece(D8, BLACK_ROOK));
}

// Integration tests for castling methods
#[test]
fn test_castling_methods_toggle_player() {
    let position = Position::default()
        .with_piece(WHITE_ROOK, A1)
        .with_piece(WHITE_ROOK, H1)
        .with_piece(WHITE_KING, E1)
        .with_piece(BLACK_ROOK, A8)
        .with_piece(BLACK_ROOK, H8)
        .with_piece(BLACK_KING, E8);
    // White to move

    let white_kingside = get_castling_move(&position, WHITE_KINGSIDE);
    assert_eq!(white_kingside.unwrap().position.get_player(), Color::Black);

    let white_queenside = get_castling_move(&position, WHITE_QUEENSIDE);
    assert_eq!(white_queenside.unwrap().position.get_player(), Color::Black);

    let mut position_black = position;
    position_black.toggle_player(); // Black to move
    let black_kingside = get_castling_move(&position_black, BLACK_KINGSIDE);
    assert_eq!(black_kingside.unwrap().position.get_player(), Color::White);

    let black_queenside = get_castling_move(&position_black, BLACK_QUEENSIDE);
    assert_eq!(black_queenside.unwrap().position.get_player(), Color::White);
}

#[test]
fn test_castling_methods_reset_en_passant() {
    let mut position = Position::default()
        .with_piece(WHITE_ROOK, A1)
        .with_piece(WHITE_ROOK, H1)
        .with_piece(WHITE_KING, E1)
        .with_piece(BLACK_ROOK, A8)
        .with_piece(BLACK_ROOK, H8)
        .with_piece(BLACK_KING, E8);

    position.set_en_passant(Some(E4));

    let white_kingside = get_castling_move(&position, WHITE_KINGSIDE);
    assert_eq!(white_kingside.unwrap().position.get_en_passant(), None);

    let white_queenside = get_castling_move(&position, WHITE_QUEENSIDE);
    assert_eq!(white_queenside.unwrap().position.get_en_passant(), None);

    let black_kingside = get_castling_move(&position, BLACK_KINGSIDE);
    assert_eq!(black_kingside.unwrap().position.get_en_passant(), None);

    let black_queenside = get_castling_move(&position, BLACK_QUEENSIDE);
    assert_eq!(black_queenside.unwrap().position.get_en_passant(), None);
}

#[test]
fn test_castling_methods_disable_castling_rights() {
    let position = Position::default()
        .with_piece(WHITE_ROOK, A1)
        .with_piece(WHITE_ROOK, H1)
        .with_piece(WHITE_KING, E1)
        .with_piece(BLACK_ROOK, A8)
        .with_piece(BLACK_ROOK, H8)
        .with_piece(BLACK_KING, E8);

    let white_kingside = get_castling_move(&position, WHITE_KINGSIDE);
    assert!(!white_kingside
        .unwrap()
        .position
        .has_castling_rights(CastlingRights::WHITE_KINGSIDE));
    assert!(!white_kingside
        .unwrap()
        .position
        .has_castling_rights(CastlingRights::WHITE_QUEENSIDE));

    let white_queenside = get_castling_move(&position, WHITE_QUEENSIDE);
    assert!(!white_queenside
        .unwrap()
        .position
        .has_castling_rights(CastlingRights::WHITE_KINGSIDE));
    assert!(!white_queenside
        .unwrap()
        .position
        .has_castling_rights(CastlingRights::WHITE_QUEENSIDE));

    let black_kingside = get_castling_move(&position, BLACK_KINGSIDE);
    assert!(!black_kingside
        .unwrap()
        .position
        .has_castling_rights(CastlingRights::BLACK_KINGSIDE));
    assert!(!black_kingside
        .unwrap()
        .position
        .has_castling_rights(CastlingRights::BLACK_QUEENSIDE));

    let black_queenside = get_castling_move(&position, BLACK_QUEENSIDE);
    assert!(!black_queenside
        .unwrap()
        .position
        .has_castling_rights(CastlingRights::BLACK_KINGSIDE));
    assert!(!black_queenside
        .unwrap()
        .position
        .has_castling_rights(CastlingRights::BLACK_QUEENSIDE));
}
