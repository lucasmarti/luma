// White Castling Tests

use crate::engine::{
    chess_moves::castle::{get_black_castle_moves, get_white_castle_moves},
    directions::*,
    piece::*,
    position::*,
};

#[test]
fn test_white_kingside_castle_allowed() {
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1);

    let moves = get_white_castle_moves(&position);

    // Should have normal king moves plus castling
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, WHITE_KING) && pos.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    assert!(
        castle_move.is_some(),
        "White kingside castle should be possible"
    );
}

#[test]
fn test_white_queenside_castle_allowed() {
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1);

    let moves = get_white_castle_moves(&position);

    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, WHITE_KING) && pos.is_occupied_by_piece(D1, WHITE_ROOK)
    });
    assert!(
        castle_move.is_some(),
        "White queenside castle should be possible"
    );
}

#[test]
fn test_white_castle_blocked_by_pieces() {
    // Kingside blocked
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(WHITE_QUEEN, F1); // Blocking piece

    let moves = get_white_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, WHITE_KING) && pos.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "White kingside castle should be blocked"
    );

    // Queenside blocked
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1)
        .put_piece(WHITE_QUEEN, D1); // Blocking piece

    let moves = get_white_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, WHITE_KING) && pos.is_occupied_by_piece(D1, WHITE_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "White queenside castle should be blocked"
    );
}

#[test]
fn test_white_castle_not_allowed_when_in_check() {
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(BLACK_ROOK, E8); // Attacking the king

    let moves = get_white_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, WHITE_KING) && pos.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "Castle should not be allowed when in check"
    );
}

#[test]
fn test_white_castle_not_allowed_through_check() {
    // Kingside - F1 under attack
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(BLACK_ROOK, F8); // Attacking F1

    let moves = get_white_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, WHITE_KING) && pos.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "Castle should not be allowed through check"
    );

    // Queenside - D1 under attack
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1)
        .put_piece(BLACK_ROOK, D8); // Attacking D1

    let moves = get_white_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, WHITE_KING) && pos.is_occupied_by_piece(D1, WHITE_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "Castle should not be allowed through check"
    );
}

// Black Castling Tests

#[test]
fn test_black_kingside_castle_allowed() {
    let mut position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8);

    let moves = get_black_castle_moves(&position);

    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G8, BLACK_KING) && pos.is_occupied_by_piece(F8, BLACK_ROOK)
    });
    assert!(
        castle_move.is_some(),
        "Black kingside castle should be possible"
    );
}

#[test]
fn test_black_queenside_castle_allowed() {
    let mut position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8);
    let moves = get_black_castle_moves(&position);

    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C8, BLACK_KING) && pos.is_occupied_by_piece(D8, BLACK_ROOK)
    });
    assert!(
        castle_move.is_some(),
        "Black queenside castle should be possible"
    );
}

#[test]
fn test_black_castle_blocked_by_pieces() {
    // Kingside blocked
    let mut position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8)
        .put_piece(BLACK_QUEEN, F8); // Blocking piece

    let moves = get_black_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G8, BLACK_KING) && pos.is_occupied_by_piece(F8, BLACK_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "Black kingside castle should be blocked"
    );

    // Queenside blocked
    let mut position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8)
        .put_piece(BLACK_QUEEN, B8); // Blocking piece

    let moves = get_black_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C8, BLACK_KING) && pos.is_occupied_by_piece(D8, BLACK_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "Black queenside castle should be blocked"
    );
}

#[test]
fn test_black_castle_not_allowed_when_in_check() {
    let mut position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8)
        .put_piece(WHITE_ROOK, E1); // Attacking the king

    let moves = get_black_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G8, BLACK_KING) && pos.is_occupied_by_piece(F8, BLACK_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "Castle should not be allowed when in check"
    );
}

#[test]
fn test_black_castle_not_allowed_through_check() {
    // Kingside - F8 under attack
    let mut position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8)
        .put_piece(WHITE_ROOK, F1); // Attacking F8

    let moves = get_black_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G8, BLACK_KING) && pos.is_occupied_by_piece(F8, BLACK_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "Castle should not be allowed through check"
    );

    // Queenside - D8 under attack
    let mut position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8)
        .put_piece(WHITE_ROOK, D1); // Attacking D8

    let moves = get_black_castle_moves(&position);
    let castle_move = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C8, BLACK_KING) && pos.is_occupied_by_piece(D8, BLACK_ROOK)
    });
    assert!(
        castle_move.is_none(),
        "Castle should not be allowed through check"
    );
}

#[test]
fn test_castle_rights_false() {
    // White with pieces in position but no rights
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(WHITE_ROOK, A1)
        .disallow_white_kingside_castle()
        .disallow_white_queenside_castle();

    let moves = get_white_castle_moves(&position);
    let kingside_castle = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, WHITE_KING) && pos.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    let queenside_castle = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, WHITE_KING) && pos.is_occupied_by_piece(D1, WHITE_ROOK)
    });

    assert!(
        kingside_castle.is_none(),
        "Kingside castle should not be allowed without rights"
    );
    assert!(
        queenside_castle.is_none(),
        "Queenside castle should not be allowed without rights"
    );
}

#[test]
fn test_both_castles_available() {
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(WHITE_ROOK, A1);

    let moves = get_white_castle_moves(&position);

    let kingside_castle = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(G1, WHITE_KING) && pos.is_occupied_by_piece(F1, WHITE_ROOK)
    });
    let queenside_castle = moves.iter().find(|pos| {
        pos.is_occupied_by_piece(C1, WHITE_KING) && pos.is_occupied_by_piece(D1, WHITE_ROOK)
    });

    assert!(
        kingside_castle.is_some(),
        "Kingside castle should be available"
    );
    assert!(
        queenside_castle.is_some(),
        "Queenside castle should be available"
    );
}
