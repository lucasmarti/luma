use std::assert_eq;

use crate::engine::chess_move::CastlingType::WhiteKingside;
use crate::engine::position::{self, CastlingRights};
use crate::engine::MoveType::{Castling, EnPassant, Promotion, PromotionCapture};
use crate::engine::Typ::Rook;
use crate::engine::{
    movegen::*,
    position::move_api::Undo,
    Color,
    MoveType::{self, Capture, DoublePawnPush, Quiet},
    Mve, Piece, Position,
    Typ::Pawn,
    WHITE_KNIGHT, WHITE_PAWN,
};
use crate::engine::{BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP, WHITE_KING, WHITE_ROOK};

#[test]
fn make_quiet_move() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mve = mv(WHITE_PAWN, E2, E3, Quiet);
    let (original, position, _) = make_move_from_fen(fen, mve);
    assert_eq!(position.get_player(), Color::Black);
    assert_eq!(position.castling_rights, original.castling_rights);
    assert_eq!(position.get_piece_at(E2), None);
    assert_eq!(position.get_piece_at(E3), Some(WHITE_PAWN));
    assert_eq!(position.get_en_passant(), None);
    assert_move_roundtrip(fen, mve);
}

#[test]
fn make_double_pawn_push() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mve = mv(WHITE_PAWN, E2, E4, DoublePawnPush(E3));
    let (original, position, _) = make_move_from_fen(fen, mve);
    assert_eq!(position.get_player(), Color::Black);
    assert_eq!(position.castling_rights, original.castling_rights);
    assert_eq!(position.get_piece_at(E2), None);
    assert_eq!(position.get_piece_at(E4), Some(WHITE_PAWN));
    assert_eq!(position.get_en_passant(), Some(E3));
    assert_move_roundtrip(fen, mve);
}

#[test]
fn make_capture() {
    let fen = "rnbqkbnr/p1p1pp1p/3p2p1/1p6/P7/6P1/1PPPPP1P/RNBQKBNR w KQkq - 0 4";
    let mve = mv(WHITE_PAWN, A4, B5, Capture);
    let (original, position, undo) = make_move_from_fen(fen, mve);
    assert_eq!(position.get_player(), Color::Black);
    assert_eq!(position.get_piece_at(A4), None);
    assert_eq!(position.get_piece_at(B5), Some(WHITE_PAWN));
    assert_eq!(position.get_squares(Color::Black, Pawn).contains(B5), false);
    assert_eq!(position.get_en_passant(), None);
    assert_eq!(position.castling_rights, original.castling_rights);
    assert_move_roundtrip(fen, mve);
}

#[test]
fn make_move_removes_castling_rights_when_rook_is_captured() {
    let fen = "rnbqkbnr/p1pppp2/1p4p1/7p/8/P5P1/1PPPPPBP/RNBQK1NR w KQkq - 0 4";
    let mve = mv(WHITE_BISHOP, G2, A8, Capture);
    let (_, position, _) = make_move_from_fen(fen, mve);
    let expected_castling_rights = CastlingRights::WHITE_KINGSIDE
        | CastlingRights::WHITE_QUEENSIDE
        | CastlingRights::BLACK_KINGSIDE;
    assert_eq!(position.castling_rights, expected_castling_rights);
}
#[test]
fn make_move_removes_castling_rights_when_rook_is_moved() {
    let fen = "rn1qkbnr/pb1ppp2/1p4P1/2p5/8/P6B/1PPPPP1P/RNBQK1NR b KQkq - 0 7";
    let mve = mv(BLACK_ROOK, H8, H6, Quiet);
    let (_, position, _) = make_move_from_fen(fen, mve);
    let expected_castling_rights = CastlingRights::WHITE_KINGSIDE
        | CastlingRights::WHITE_QUEENSIDE
        | CastlingRights::BLACK_QUEENSIDE;
    assert_eq!(position.castling_rights, expected_castling_rights);
}

#[test]
fn make_promotion() {
    let fen = "rn1qkbnr/pb2pp2/1p1p2P1/8/P7/N1P4B/1p1PPP1P/R1BQK1NR b KQkq - 1 11";
    let mve = mv(BLACK_PAWN, B2, B1, Promotion(BLACK_QUEEN));
    let (original, position, _) = make_move_from_fen(fen, mve);
    assert_eq!(position.get_player(), Color::White);
    assert_eq!(position.get_piece_at(B1), Some(BLACK_QUEEN));
    assert_eq!(position.get_piece_at(B2), None);
    assert_eq!(position.get_en_passant(), None);
    assert_eq!(position.castling_rights, original.castling_rights);
    assert_move_roundtrip(fen, mve);
}

#[test]
fn make_promotion_capture() {
    let fen = "rn1qkbnr/pb2pp2/1p1p2P1/8/P7/N1P4B/1p1PPP1P/R1BQK1NR b KQkq - 1 11";
    let mve = mv(BLACK_PAWN, B2, A1, PromotionCapture(BLACK_QUEEN));
    let (_, position, _) = make_move_from_fen(fen, mve);
    assert_eq!(position.get_player(), Color::White);
    assert_eq!(position.get_piece_at(A1), Some(BLACK_QUEEN));
    assert_eq!(position.get_piece_at(B2), None);
    assert_eq!(position.get_squares(Color::Black, Rook).contains(A1), false);
    assert_eq!(position.get_en_passant(), None);
    assert_move_roundtrip(fen, mve);
}

#[test]
fn make_en_passant() {
    let fen = "rn1qkbnr/pb3p2/3p2P1/1p2pP2/P7/N1P4B/3PP2P/R1nQK1NR w KQkq e6 0 14";
    let mve = mv(WHITE_PAWN, F5, E6, EnPassant(E5));
    let (original, position, _) = make_move_from_fen(fen, mve);
    assert_eq!(position.en_passant, None);
    assert_eq!(position.get_piece_at(F5), None);
    assert_eq!(position.get_piece_at(E5), None);
    assert_eq!(position.get_piece_at(E6), Some(WHITE_PAWN));
    assert_eq!(position.castling_rights, original.castling_rights);
    assert_move_roundtrip(fen, mve);
}

#[test]
fn make_castling() {
    let fen = "rn1qk1nr/pb2b3/3pp1P1/1p6/P7/N1P2N1B/3PP2P/R1nQK2R w KQkq - 2 16";
    let mve = mv(WHITE_KING, E1, G1, Castling(WhiteKingside));
    let (original, position, _) = make_move_from_fen(fen, mve);
    let mut expected_castling_rights = original.castling_rights;
    expected_castling_rights
        .remove(CastlingRights::WHITE_QUEENSIDE | CastlingRights::WHITE_KINGSIDE);
    assert_eq!(position.castling_rights, expected_castling_rights);
    assert_eq!(position.get_piece_at(E1), None);
    assert_eq!(position.get_piece_at(G1), Some(WHITE_KING));
    assert_eq!(position.get_piece_at(H1), None);
    assert_eq!(position.get_piece_at(F1), Some(WHITE_ROOK));
    assert_eq!(position.en_passant, None);
    assert_move_roundtrip(fen, mve);
}

const fn mv(piece: Piece, from: Square, to: Square, move_type: MoveType) -> Mve {
    Mve {
        piece,
        from,
        to,
        move_type,
    }
}
fn make_move_from_fen(fen: &str, mve: Mve) -> (Position, Position, Undo) {
    let original = Position::from_fen(fen).unwrap();
    let mut position = original.clone();

    let undo = position.make_move(mve);

    (original, position, undo)
}

fn assert_move_roundtrip(fen: &str, mve: Mve) {
    let (original, mut position, undo) = make_move_from_fen(fen, mve);

    position.unmake(mve, undo);

    assert_eq!(position, original);
}
