use crate::bitboard::Bitboard;
use crate::evaluation::*;
use crate::position::*;

#[test]
fn test_equal_material() {
    let position: Position = Position {
        white_king: Bitboard::from(1),
        white_queen: Bitboard::from(1),
        white_rooks: Bitboard::from(1),
        white_bishops: Bitboard::from(1),
        white_knights: Bitboard::from(1),
        white_pawns: Bitboard::from(1),
        white_kingside_castle: false,
        white_queenside_castle: false,
        black_king: Bitboard::from(1),
        black_queen: Bitboard::from(1),
        black_rooks: Bitboard::from(1),
        black_bishops: Bitboard::from(1),
        black_knights: Bitboard::from(1),
        black_pawns: Bitboard::from(1),
        black_kingside_castle: false,
        black_queenside_castle: false,
        en_passant: None,
    };
    assert_eq!(evaluate(&position), 0);
}

#[test]
fn test_white_queen_missing() {
    let position: Position = Position {
        white_king: Bitboard::from(1),
        white_queen: Bitboard::new(),
        white_rooks: Bitboard::from(1),
        white_bishops: Bitboard::from(1),
        white_knights: Bitboard::from(1),
        white_pawns: Bitboard::from(1) | Bitboard::from(2),
        white_kingside_castle: false,
        white_queenside_castle: false,
        black_king: Bitboard::from(1),
        black_queen: Bitboard::from(1),
        black_rooks: Bitboard::from(1),
        black_bishops: Bitboard::from(1),
        black_knights: Bitboard::from(1),
        black_pawns: Bitboard::from(1) | Bitboard::from(2),
        black_kingside_castle: false,
        black_queenside_castle: false,
        en_passant: None,
    };
    assert_eq!(evaluate(&position), -9);
}

#[test]
fn test_black_queen_missing() {
    let position: Position = Position {
        white_king: Bitboard::from(1),
        white_queen: Bitboard::from(1),
        white_rooks: Bitboard::from(1),
        white_bishops: Bitboard::from(1),
        white_knights: Bitboard::from(1),
        white_pawns: Bitboard::from(1),
        white_kingside_castle: false,
        white_queenside_castle: false,
        black_king: Bitboard::from(1),
        black_queen: Bitboard::new(),
        black_rooks: Bitboard::from(1),
        black_bishops: Bitboard::from(1),
        black_knights: Bitboard::from(1),
        black_pawns: Bitboard::from(1),
        black_kingside_castle: false,
        black_queenside_castle: false,
        en_passant: None,
    };
    assert_eq!(evaluate(&position), 9);
}
