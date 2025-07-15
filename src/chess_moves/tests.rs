use crate::{
    chess_moves::{
        self, move_piece, BlackKingsideCastle, BlackQueensideCastle, ChessMove, EnPassant,
        Progress, Promotion, WhiteKingsideCastle, WhiteQueensideCastle,
    },
    directions::{self, *},
    piece::*,
    position::{self, Position},
};
#[test]
fn test_progress_white_king() {
    let position = Position::new_starting_position();
    let progress = Progress {
        from: E1,
        to: E2,
        piece: WHITE_KING,
    };
    let new_position = move_piece(&position, chess_moves::ChessMove::Progress(progress));
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
    let new_position = move_piece(&&position, ChessMove::Promotion(promotion));
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
    let new_position = move_piece(&position, ChessMove::EnPassant(enPassant));
    assert!(!new_position.is_occupied_by_piece(D4, WHITE_PAWN));
    assert!(!new_position.is_occupied_by_piece(E4, BLACK_PAWN));
    assert!(new_position.is_occupied_by_piece(D3, BLACK_PAWN));
}

#[test]
fn test_white_kingside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, H1);
    let new_position = move_piece(
        &position,
        ChessMove::WhiteKingsideCastle(WhiteKingsideCastle {}),
    );
    assert!(new_position.is_occupied_by_piece(G1, WHITE_KING));
    assert!(new_position.is_occupied_by_piece(F1, WHITE_ROOK));
}

#[test]
fn test_white_queenside_castle() {
    let position = Position::default()
        .put_piece(WHITE_KING, E1)
        .put_piece(WHITE_ROOK, A1);
    let new_position = move_piece(
        &position,
        ChessMove::WhiteQueensideCastle(WhiteQueensideCastle {}),
    );
    assert!(new_position.is_occupied_by_piece(C1, WHITE_KING));
    assert!(new_position.is_occupied_by_piece(D1, WHITE_ROOK));
}

#[test]
fn test_black_kingside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, H8);
    let new_position = move_piece(
        &position,
        ChessMove::BlackKingsideCastle(BlackKingsideCastle {}),
    );
    assert!(new_position.is_occupied_by_piece(G8, BLACK_KING));
    assert!(new_position.is_occupied_by_piece(F8, BLACK_ROOK));
}

#[test]
fn test_black_queenside_castle() {
    let position = Position::default()
        .put_piece(BLACK_KING, E8)
        .put_piece(BLACK_ROOK, A8);
    let new_position = move_piece(
        &position,
        ChessMove::BlackQueensideCastle(BlackQueensideCastle {}),
    );
    assert!(new_position.is_occupied_by_piece(C8, BLACK_KING));
    assert!(new_position.is_occupied_by_piece(D8, BLACK_ROOK));
}
