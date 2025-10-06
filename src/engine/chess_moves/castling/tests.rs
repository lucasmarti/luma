// White Castling Tests

use crate::engine::{
    chess_moves::castling::{
        configurations::{BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE},
        get_black_castle_moves, get_castling_move, get_white_castle_moves,
    },
    directions::squares::*,
    piece::*,
    position::*,
};

#[test]
fn test_white_kingside_castle_allowed() {
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
    let position = Position::default()
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
        .remove_castling_right(CastlingType::WhiteKingside)
        .remove_castling_right(CastlingType::WhiteQueenside);

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
    let position = Position::default()
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

#[test]
fn test_white_kingside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1);
    let new_position = get_castling_move(&position, WHITE_KINGSIDE);
    assert!(new_position.is_some());
    assert!(new_position.unwrap().is_occupied_by_piece(G1, WHITE_KING));
    assert!(new_position.unwrap().is_occupied_by_piece(F1, WHITE_ROOK));
}

#[test]
fn test_white_queenside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1);
    let new_position = get_castling_move(&position, WHITE_QUEENSIDE);
    assert!(new_position.is_some());
    assert!(new_position.unwrap().is_occupied_by_piece(C1, WHITE_KING));
    assert!(new_position.unwrap().is_occupied_by_piece(D1, WHITE_ROOK));
}

#[test]
fn test_black_kingside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8);
    let new_position = get_castling_move(&position, BLACK_KINGSIDE);
    assert!(new_position.is_some());
    assert!(new_position.unwrap().is_occupied_by_piece(G8, BLACK_KING));
    assert!(new_position.unwrap().is_occupied_by_piece(F8, BLACK_ROOK));
}

#[test]
fn test_black_queenside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8);
    let new_position = get_castling_move(&position, BLACK_QUEENSIDE);
    assert!(new_position.is_some());
    assert!(new_position.unwrap().is_occupied_by_piece(C8, BLACK_KING));
    assert!(new_position.unwrap().is_occupied_by_piece(D8, BLACK_ROOK));
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
fn test_castle_methods_reset_en_passant() {
    let position = Position::default()
        .put_piece(WHITE_ROOK, A1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_ROOK, A8)
        .put_piece(BLACK_ROOK, H8)
        .put_piece(BLACK_KING, E8)
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
fn test_castle_methods_disable_castle_rights() {
    let position = Position::default()
        .put_piece(WHITE_ROOK, A1)
        .put_piece(WHITE_ROOK, H1)
        .put_piece(WHITE_KING, E1)
        .put_piece(BLACK_ROOK, A8)
        .put_piece(BLACK_ROOK, H8)
        .put_piece(BLACK_KING, E8);

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
