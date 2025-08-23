use std::u32;

use crate::engine::{
    chess_moves::common::{
        generate_path_with_limit, get_moves_for_king_at_square, get_moves_for_rook_at_square, slide,
    },
    directions::{self, squares::*},
    piece::*,
    position::Position,
};

#[test]
fn test_slide1() {
    let mut position: Position = Position::default();
    position = position
        .put_piece(WHITE_QUEEN, G4)
        .put_piece(WHITE_PAWN, G7);

    let path = generate_path_with_limit(G4, directions::up, u32::MAX);
    let positions = slide(&position, G4, path, WHITE_QUEEN);

    let result: Vec<Position> = vec![
        Position::default()
            .put_piece(WHITE_QUEEN, G5)
            .put_piece(WHITE_PAWN, G7)
            .toggle_player()
            .set_to_square(G5),
        Position::default()
            .put_piece(WHITE_QUEEN, G6)
            .put_piece(WHITE_PAWN, G7)
            .toggle_player()
            .set_to_square(G6),
    ];
    assert_eq!(positions, result);
}

#[test]
fn test_get_piece_moves_king() {
    let mut position: Position = Position::default();
    position = position.put_piece(WHITE_KING, D4);
    let positions = get_moves_for_king_at_square(&position, WHITE_KING, D4);

    assert_eq!(positions.len(), 8);

    // Check specific moves using helper function
    assert!(contains_move(&positions, WHITE_KING, D5)); // up
    assert!(contains_move(&positions, WHITE_KING, D3)); // down
    assert!(contains_move(&positions, WHITE_KING, C4)); // left
    assert!(contains_move(&positions, WHITE_KING, E4)); // right
}

#[test]
fn test_get_piece_moves_king_blocked_by_own_piece() {
    let mut position: Position = Position::default();
    position = position.put_piece(WHITE_KING, D4).put_piece(WHITE_PAWN, D5); // Block upward movement

    let positions = get_moves_for_king_at_square(&position, WHITE_KING, D4);
    assert_eq!(positions.len(), 7); // One less because D5 is blocked

    assert!(!contains_move(&positions, WHITE_KING, D5)); // Should be blocked
    assert!(contains_move(&positions, WHITE_KING, D3)); // Should still be available
}

#[test]
fn test_get_piece_moves_rook() {
    let position: Position = Position::default().put_piece(WHITE_ROOK, D4);

    // Test rook directions (4 directions)
    let moves = get_moves_for_rook_at_square(&position, WHITE_ROOK, D4);
    // Should be able to move in all 4 directions until board edge
    // Up: D5, D6, D7, D8 (4 moves)
    // Down: D3, D2, D1 (3 moves)
    // Left: C4, B4, A4 (3 moves)
    // Right: E4, F4, G4, H4 (4 moves)
    assert_eq!(moves.len(), 14);
}

#[test]
fn test_get_piece_moves_rook_with_obstacles() {
    let mut position: Position = Position::default().put_piece(WHITE_KING, A1);
    position = position
        .put_piece(WHITE_ROOK, D4)
        .put_piece(WHITE_PAWN, D6) // Block upward at D6
        .put_piece(BLACK_PAWN, F4); // Enemy piece at F4 (can capture)

    let positions = get_moves_for_rook_at_square(&position, WHITE_ROOK, D4);
    // Up: only D5 (blocked by own pawn at D6)
    assert!(contains_move(&positions, WHITE_ROOK, D5));
    assert!(!contains_move(&positions, WHITE_ROOK, D6)); // Blocked by own piece

    // Right: E4, F4 (can capture enemy at F4, but can't go beyond)
    assert!(contains_move(&positions, WHITE_ROOK, E4));
    assert!(contains_move(&positions, WHITE_ROOK, F4)); // Can capture
    assert!(!contains_move(&positions, WHITE_ROOK, G4)); // Can't go beyond captured piece
}

fn contains_move(positions: &Vec<Position>, piece: Piece, field: u32) -> bool {
    for position in positions {
        if position.is_occupied_by_piece(field, piece) {
            return true;
        }
    }
    false
}

#[test]
fn test_slide2() {
    let mut position: Position = Position::default();
    position = position
        .put_piece(WHITE_QUEEN, G4)
        .put_piece(BLACK_PAWN, G7);
    let path = generate_path_with_limit(G4, directions::up, u32::MAX);

    let positions = slide(&position, G4, path, WHITE_QUEEN);
    let result: Vec<Position> = vec![
        Position::default()
            .put_piece(WHITE_QUEEN, G5)
            .put_piece(BLACK_PAWN, G7)
            .toggle_player()
            .set_to_square(G5),
        Position::default()
            .put_piece(WHITE_QUEEN, G6)
            .put_piece(BLACK_PAWN, G7)
            .toggle_player()
            .set_to_square(G6),
        Position::default()
            .put_piece(WHITE_QUEEN, G7)
            .toggle_player()
            .set_to_square(G7),
    ];
    assert!(positions == result);
}

#[test]
fn test_slide3() {
    let mut position: Position = Position::default();
    position = position.put_piece(WHITE_QUEEN, G4);
    let path = generate_path_with_limit(G4, directions::up, u32::MAX);

    let positions = slide(&position, G4, path, WHITE_QUEEN);
    let result: Vec<Position> = vec![
        Position::default()
            .put_piece(WHITE_QUEEN, G5)
            .toggle_player()
            .set_to_square(G5),
        Position::default()
            .put_piece(WHITE_QUEEN, G6)
            .toggle_player()
            .set_to_square(G6),
        Position::default()
            .put_piece(WHITE_QUEEN, G7)
            .toggle_player()
            .set_to_square(G7),
        Position::default()
            .put_piece(WHITE_QUEEN, G8)
            .toggle_player()
            .set_to_square(G8),
    ];
    assert!(positions == result);
}
