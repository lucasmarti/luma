use std::u32;

use crate::engine::{
    chess_moves::{
        common::{
            generate_path_with_limit, get_moves_for_king_at_square, get_moves_for_rook_at_square,
            slide,
        },
        ChessMove,
    },
    directions::{self, squares::*},
    piece::*,
    position::Position,
};

#[test]
fn test_slide1() {
    let mut position: Position = Position::default();
    position = position
        .put_piece(Piece::WhiteQueen, G4)
        .put_piece(Piece::WhitePawn, G7);

    let path = generate_path_with_limit(G4, directions::up, u32::MAX);
    let positions = slide(&position, G4, path, Piece::WhiteQueen);
    assert!(positions
        .iter()
        .any(|p| p.position.is_occupied_by_piece(G5, Piece::WhiteQueen)
            && p.position.is_occupied_by_piece(G7, Piece::WhitePawn)));
    assert!(positions
        .iter()
        .any(|p| p.position.is_occupied_by_piece(G6, Piece::WhiteQueen)
            && p.position.is_occupied_by_piece(G7, Piece::WhitePawn)));
}

#[test]
fn test_get_piece_moves_king() {
    let mut position: Position = Position::default();
    position = position.put_piece(Piece::WhiteKing, D4);
    let chess_move = get_moves_for_king_at_square(&position, Piece::WhiteKing, D4);

    assert_eq!(chess_move.len(), 8);

    // Check specific moves using helper function
    assert!(contains_move(&chess_move, Piece::WhiteKing, D5)); // up
    assert!(contains_move(&chess_move, Piece::WhiteKing, D3)); // down
    assert!(contains_move(&chess_move, Piece::WhiteKing, C4)); // left
    assert!(contains_move(&chess_move, Piece::WhiteKing, E4)); // right
}

#[test]
fn test_get_piece_moves_king_blocked_by_own_piece() {
    let mut position: Position = Position::default();
    position = position
        .put_piece(Piece::WhiteKing, D4)
        .put_piece(Piece::WhitePawn, D5); // Block upward movement

    let positions = get_moves_for_king_at_square(&position, Piece::WhiteKing, D4);
    assert_eq!(positions.len(), 7); // One less because D5 is blocked

    assert!(!contains_move(&positions, Piece::WhiteKing, D5)); // Should be blocked
    assert!(contains_move(&positions, Piece::WhiteKing, D3)); // Should still be available
}

#[test]
fn test_get_piece_moves_rook() {
    let position: Position = Position::default().put_piece(Piece::WhiteRook, D4);

    // Test rook directions (4 directions)
    let moves = get_moves_for_rook_at_square(&position, Piece::WhiteRook, D4);
    // Should be able to move in all 4 directions until board edge
    // Up: D5, D6, D7, D8 (4 moves)
    // Down: D3, D2, D1 (3 moves)
    // Left: C4, B4, A4 (3 moves)
    // Right: E4, F4, G4, H4 (4 moves)
    assert_eq!(moves.len(), 14);
}

#[test]
fn test_get_piece_moves_rook_with_obstacles() {
    let mut position: Position = Position::default().put_piece(Piece::WhiteKing, A1);
    position = position
        .put_piece(Piece::WhiteRook, D4)
        .put_piece(Piece::WhitePawn, D6) // Block upward at D6
        .put_piece(Piece::BlackPawn, F4); // Enemy piece at F4 (can capture)

    let positions = get_moves_for_rook_at_square(&position, Piece::WhiteRook, D4);
    // Up: only D5 (blocked by own pawn at D6)
    assert!(contains_move(&positions, Piece::WhiteRook, D5));
    assert!(!contains_move(&positions, Piece::WhiteRook, D6)); // Blocked by own piece

    // Right: E4, F4 (can capture enemy at F4, but can't go beyond)
    assert!(contains_move(&positions, Piece::WhiteRook, E4));
    assert!(contains_move(&positions, Piece::WhiteRook, F4)); // Can capture
    assert!(!contains_move(&positions, Piece::WhiteRook, G4)); // Can't go beyond captured piece
}

fn contains_move(chess_moves: &Vec<ChessMove>, piece: Piece, field: Square) -> bool {
    for chess_move in chess_moves {
        if chess_move.position.is_occupied_by_piece(field, piece) {
            return true;
        }
    }
    false
}

#[test]
fn test_slide2() {
    let mut position: Position = Position::default();
    position = position
        .put_piece(Piece::WhiteQueen, G4)
        .put_piece(Piece::BlackPawn, G7);
    let path = generate_path_with_limit(G4, directions::up, u32::MAX);

    let positions = slide(&position, G4, path, Piece::WhiteQueen);

    assert!(positions
        .iter()
        .any(|p| p.position.is_occupied_by_piece(G5, Piece::WhiteQueen)
            && p.position.is_occupied_by_piece(G7, Piece::BlackPawn)));
    assert!(positions
        .iter()
        .any(|p| p.position.is_occupied_by_piece(G6, Piece::WhiteQueen)
            && p.position.is_occupied_by_piece(G7, Piece::BlackPawn)));
    assert!(positions
        .iter()
        .any(|p| p.position.is_occupied_by_piece(G7, Piece::WhiteQueen)
            && !p.position.is_occupied_by_piece(G7, Piece::BlackPawn)));
}

#[test]
fn test_slide3() {
    let mut position: Position = Position::default();
    position = position.put_piece(Piece::WhiteQueen, G4);
    let path = generate_path_with_limit(G4, directions::up, u32::MAX);

    let positions = slide(&position, G4, path, Piece::WhiteQueen);
    assert!(positions
        .iter()
        .any(|p| p.position.is_occupied_by_piece(G5, Piece::WhiteQueen)));
    assert!(positions
        .iter()
        .any(|p| p.position.is_occupied_by_piece(G6, Piece::WhiteQueen)));
    assert!(positions
        .iter()
        .any(|p| p.position.is_occupied_by_piece(G7, Piece::WhiteQueen)));
    assert!(positions
        .iter()
        .any(|p| p.position.is_occupied_by_piece(G8, Piece::WhiteQueen)));
}
