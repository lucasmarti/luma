use crate::{
    bitboard::Bitboard,
    chess_moves::{self, ChessMove, Progress, Promotion},
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
    assert_eq!(position.white_king.contains(E1), true);
    assert_eq!(new_position.white_king.contains(E1), false);
    assert_eq!(new_position.white_king.contains(E2), true);
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
    assert_eq!(new_position.white_queen.contains(A8), true);
    assert_eq!(new_position.white_pawns.contains(A7), false);
}
#[test]
fn test_white_kingside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1);
    let new_position = position.move_piece(ChessMove::WhiteKingsideCastle);
    assert_eq!(new_position.white_king.contains(G1), true);
    assert_eq!(new_position.white_rooks.contains(F1), true);
}

#[test]
fn test_white_queenside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1);
    let new_position = position.move_piece(ChessMove::WhiteQueensideCastle);
    assert_eq!(new_position.white_king.contains(C1), true);
    assert_eq!(new_position.white_rooks.contains(D1), true);
}

#[test]
fn test_black_kingside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8);
    let new_position = position.move_piece(ChessMove::BlackKingsideCastle);
    assert_eq!(new_position.black_king.contains(G8), true);
    assert_eq!(new_position.black_rooks.contains(F8), true);
}

#[test]
fn test_black_queenside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8);
    let new_position = position.move_piece(ChessMove::BlackQueensideCastle);
    assert_eq!(new_position.black_king.contains(C8), true);
    assert_eq!(new_position.black_rooks.contains(D8), true);
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
    assert_eq!(position.white_king.contains(E2), false);
    assert_eq!(new_position.white_king.contains(E2), true);
}
