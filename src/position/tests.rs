use crate::{
    chess_moves::{self, ChessMove, EnPassant, Progress, Promotion},
    directions::*,
    piece::*,
    position,
};

use super::{print::Print, Position};

#[test]
fn test_new_starting_position() {
    let position: Position = Position::new_starting_position();
    position.print_board();
}
#[test]
fn test_get_black() {
    let position = Position::new_starting_position();
    assert_eq!(position.get_black().count_ones(), 16);
}
#[test]
fn test_get_white() {
    let position = Position::new_starting_position();
    assert_eq!(position.get_white().count_ones(), 16);
}

#[test]
fn test_get_all() {
    let position = Position::new_starting_position();
    assert_eq!(position.get_all().count_ones(), 32);
}

#[test]
fn test_progress_white_king() {
    let position = Position::new_starting_position();
    let progress = Progress {
        from: E1,
        to: E2,
        piece: WHITE_KING,
    };
    let new_position = position.move_piece(chess_moves::ChessMove::Progress(progress));
    assert!(position.is_occupied_by_piece(E1, WHITE_KING));
    assert!(!new_position.is_occupied_by_piece(E1, WHITE_KING));
    assert!(new_position.is_occupied_by_piece(E2, WHITE_KING));
}

#[test]
fn test_promotion() {
    let position = Position::default().put_piece(WHITE_PAWN, A7);
    let promotion: Promotion = Promotion {
        piece: WHITE_PAWN,
        from: A7,
        to: A8,
        new_piece: WHITE_QUEEN,
    };
    let new_position = position.move_piece(ChessMove::Promotion(promotion));
    assert!(new_position.is_occupied_by_piece(A8, WHITE_QUEEN));
    assert!(!new_position.is_occupied_by_piece(A7, WHITE_PAWN));
}
#[test]
fn test_en_passant() {
    let position = Position::default()
        .put_piece(WHITE_PAWN, D4)
        .put_piece(BLACK_PAWN, E4);
    let enPassant = EnPassant {
        piece: BLACK_PAWN,
        from: E4,
        to: D3,
        capture: D4,
    };
    let new_position = position.move_piece(ChessMove::EnPassant(enPassant));
    assert!(!new_position.is_occupied_by_piece(D4, WHITE_PAWN));
    assert!(!new_position.is_occupied_by_piece(E4, BLACK_PAWN));
    assert!(new_position.is_occupied_by_piece(D3, BLACK_PAWN));
}

#[test]
fn test_white_kingside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1);
    let new_position = position.move_piece(ChessMove::WhiteKingsideCastle);
    assert!(new_position.is_occupied_by_piece(G1, WHITE_KING));
    assert!(new_position.is_occupied_by_piece(F1, WHITE_ROOK));
}

#[test]
fn test_white_queenside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1);
    let new_position = position.move_piece(ChessMove::WhiteQueensideCastle);
    assert!(new_position.is_occupied_by_piece(C1, WHITE_KING));
    assert!(new_position.is_occupied_by_piece(D1, WHITE_ROOK));
}

#[test]
fn test_black_kingside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8);
    let new_position = position.move_piece(ChessMove::BlackKingsideCastle);
    assert!(new_position.is_occupied_by_piece(G8, BLACK_KING));
    assert!(new_position.is_occupied_by_piece(F8, BLACK_ROOK));
}

#[test]
fn test_black_queenside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8);
    let new_position = position.move_piece(ChessMove::BlackQueensideCastle);
    assert!(new_position.is_occupied_by_piece(C8, BLACK_KING));
    assert!(new_position.is_occupied_by_piece(D8, BLACK_ROOK));
}

#[test]
fn test_remove_white_king() {
    let position = Position::new_starting_position();
    assert_eq!(position.white_king.count_ones(), 1);
    let new_position = position.remove_piece(E1);
    assert_eq!(new_position.white_king.count_ones(), 0);
}

#[test]
fn test_put_white_king() {
    let position = Position::new_starting_position();
    let new_position = position.put_piece(WHITE_KING, E2);
    assert_ne!(position.is_occupied_by_piece(E2, WHITE_KING), true);
    assert_eq!(new_position.is_occupied_by_piece(E2, WHITE_KING), true);
}
