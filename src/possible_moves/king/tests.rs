use crate::{
    directions::*,
    piece::{BLACK_KING, BLACK_QUEEN, BLACK_ROOK, WHITE_KING, WHITE_QUEEN, WHITE_ROOK},
    position::Position,
    possible_moves::king::get_possible_moves,
};

#[test]
fn test_get_possible_black_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_possible_moves(&position, D8, BLACK_KING).len() == 0);
    assert!(get_possible_moves(&position, D3, BLACK_KING).len() == 8);
    assert!(get_possible_moves(&position, F6, BLACK_KING).len() == 5);
    assert!(get_possible_moves(&position, H6, BLACK_KING).len() == 3);
}

#[test]
fn test_get_possible_white_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_possible_moves(&position, D8, WHITE_KING).len() == 5);
    assert!(get_possible_moves(&position, D3, WHITE_KING).len() == 5);
    assert!(get_possible_moves(&position, F2, WHITE_KING).len() == 3);
    assert!(get_possible_moves(&position, H6, WHITE_KING).len() == 5);
}

// White Castling Tests

#[test]
fn test_white_kingside_castle_allowed() {
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1);
    position.white_kingside_castle_allowed = true;

    let moves = get_possible_moves(&position, E1, WHITE_KING);

    // Should have normal king moves plus castling
    let castle_move = moves
        .iter()
        .find(|pos| pos.white_king.contains(G1) && pos.white_rooks.contains(F1));
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
    position.white_queenside_castle_allowed = true;

    let moves = get_possible_moves(&position, E1, WHITE_KING);

    let castle_move = moves
        .iter()
        .find(|pos| pos.white_king.contains(C1) && pos.white_rooks.contains(D1));
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
    position.white_kingside_castle_allowed = true;

    let moves = get_possible_moves(&position, E1, WHITE_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.white_king.contains(G1) && pos.white_rooks.contains(F1));
    assert!(
        castle_move.is_none(),
        "White kingside castle should be blocked"
    );

    // Queenside blocked
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1)
        .put_piece(WHITE_QUEEN, D1); // Blocking piece
    position.white_queenside_castle_allowed = true;

    let moves = get_possible_moves(&position, E1, WHITE_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.white_king.contains(C1) && pos.white_rooks.contains(D1));
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
    position.white_kingside_castle_allowed = true;

    let moves = get_possible_moves(&position, E1, WHITE_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.white_king.contains(G1) && pos.white_rooks.contains(F1));
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
    position.white_kingside_castle_allowed = true;

    let moves = get_possible_moves(&position, E1, WHITE_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.white_king.contains(G1) && pos.white_rooks.contains(F1));
    assert!(
        castle_move.is_none(),
        "Castle should not be allowed through check"
    );

    // Queenside - D1 under attack
    let mut position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1)
        .put_piece(BLACK_ROOK, D8); // Attacking D1
    position.white_queenside_castle_allowed = true;

    let moves = get_possible_moves(&position, E1, WHITE_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.white_king.contains(C1) && pos.white_rooks.contains(D1));
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
    position.black_kingside_castle_allowed = true;

    let moves = get_possible_moves(&position, E8, BLACK_KING);

    let castle_move = moves
        .iter()
        .find(|pos| pos.black_king.contains(G8) && pos.black_rooks.contains(F8));
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
    position.black_queenside_castle_allowed = true;

    let moves = get_possible_moves(&position, E8, BLACK_KING);

    let castle_move = moves
        .iter()
        .find(|pos| pos.black_king.contains(C8) && pos.black_rooks.contains(D8));
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
    position.black_kingside_castle_allowed = true;

    let moves = get_possible_moves(&position, E8, BLACK_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.black_king.contains(G8) && pos.black_rooks.contains(F8));
    assert!(
        castle_move.is_none(),
        "Black kingside castle should be blocked"
    );

    // Queenside blocked
    let mut position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8)
        .put_piece(BLACK_QUEEN, B8); // Blocking piece
    position.black_queenside_castle_allowed = true;

    let moves = get_possible_moves(&position, E8, BLACK_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.black_king.contains(C8) && pos.black_rooks.contains(D8));
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
    position.black_kingside_castle_allowed = true;

    let moves = get_possible_moves(&position, E8, BLACK_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.black_king.contains(G8) && pos.black_rooks.contains(F8));
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
    position.black_kingside_castle_allowed = true;

    let moves = get_possible_moves(&position, E8, BLACK_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.black_king.contains(G8) && pos.black_rooks.contains(F8));
    assert!(
        castle_move.is_none(),
        "Castle should not be allowed through check"
    );

    // Queenside - D8 under attack
    let mut position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8)
        .put_piece(WHITE_ROOK, D1); // Attacking D8
    position.black_queenside_castle_allowed = true;

    let moves = get_possible_moves(&position, E8, BLACK_KING);
    let castle_move = moves
        .iter()
        .find(|pos| pos.black_king.contains(C8) && pos.black_rooks.contains(D8));
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
        .put_piece(WHITE_ROOK, A1);
    // Castle rights are false by default

    let moves = get_possible_moves(&position, E1, WHITE_KING);
    let kingside_castle = moves
        .iter()
        .find(|pos| pos.white_king.contains(G1) && pos.white_rooks.contains(F1));
    let queenside_castle = moves
        .iter()
        .find(|pos| pos.white_king.contains(C1) && pos.white_rooks.contains(D1));

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
    position.white_kingside_castle_allowed = true;
    position.white_queenside_castle_allowed = true;

    let moves = get_possible_moves(&position, E1, WHITE_KING);

    let kingside_castle = moves
        .iter()
        .find(|pos| pos.white_king.contains(G1) && pos.white_rooks.contains(F1));
    let queenside_castle = moves
        .iter()
        .find(|pos| pos.white_king.contains(C1) && pos.white_rooks.contains(D1));

    assert!(
        kingside_castle.is_some(),
        "Kingside castle should be available"
    );
    assert!(
        queenside_castle.is_some(),
        "Queenside castle should be available"
    );

    // Should have 5 normal king moves + 2 castle moves = 7 total
    assert_eq!(
        moves.len(),
        7,
        "Should have 7 total moves including castles"
    );
}
