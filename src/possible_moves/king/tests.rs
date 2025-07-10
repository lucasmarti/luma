use crate::{
    chess_moves::ChessMove,
    directions::*,
    position::Position,
    possible_moves::king::{get_possible_black_moves, get_possible_white_moves},
};

#[test]
fn test_get_possible_black_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_possible_black_moves(&position, D8).len() == 0);
    assert!(get_possible_black_moves(&position, D3).len() == 8);
    assert!(get_possible_black_moves(&position, F2).len() == 8);
    assert!(get_possible_black_moves(&position, H6).len() == 3);
}

#[test]
fn test_get_possible_white_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_possible_white_moves(&position, D8).len() == 5);
    assert!(get_possible_white_moves(&position, D3).len() == 5);
    assert!(get_possible_white_moves(&position, F2).len() == 3);
    assert!(get_possible_white_moves(&position, H6).len() == 5);
}

#[test]
fn test_castling_with_starting_position() {
    // Starting position should not allow castling because pieces are in the way
    let position = Position::new_starting_position();
    let white_moves = get_possible_white_moves(&position, E1);
    let black_moves = get_possible_black_moves(&position, E8);

    // Should have no moves from starting position (blocked by pieces)
    assert_eq!(white_moves.len(), 0);
    assert_eq!(black_moves.len(), 0);
}

#[test]
fn test_castling_with_clear_path() {
    // Create position with clear castling paths
    let mut position = Position::default();
    position.white_kingside_castle = true;
    position.white_queenside_castle = true;
    position.black_kingside_castle = true;
    position.black_queenside_castle = true;

    // Place only kings on starting squares
    position = position
        .put_piece(crate::piece::WHITE_KING, E1)
        .put_piece(crate::piece::BLACK_KING, E8);

    let white_moves = get_possible_white_moves(&position, E1);
    let black_moves = get_possible_black_moves(&position, E8);

    // Should include normal king moves + 2 castle moves each
    // King has 5 normal moves from E1/E8 (can't go down/up respectively)
    // Plus 2 castle moves = 7 total
    assert_eq!(white_moves.len(), 7);
    assert_eq!(black_moves.len(), 7);

    // Check that castle moves are included
    assert!(contains_castle_move(
        &white_moves,
        ChessMove::WhiteKingsideCastle
    ));
    assert!(contains_castle_move(
        &white_moves,
        ChessMove::WhiteQueensideCastle
    ));
    assert!(contains_castle_move(
        &black_moves,
        ChessMove::BlackKingsideCastle
    ));
    assert!(contains_castle_move(
        &black_moves,
        ChessMove::BlackQueensideCastle
    ));
}

#[test]
fn test_castling_disabled_by_flags() {
    // Create position with castling flags disabled
    let mut position = Position::default();
    position.white_kingside_castle = false;
    position.white_queenside_castle = false;
    position.black_kingside_castle = false;
    position.black_queenside_castle = false;

    position = position
        .put_piece(crate::piece::WHITE_KING, E1)
        .put_piece(crate::piece::BLACK_KING, E8);

    let white_moves = get_possible_white_moves(&position, E1);
    let black_moves = get_possible_black_moves(&position, E8);

    // Should only have normal king moves (5 each)
    assert_eq!(white_moves.len(), 5);
    assert_eq!(black_moves.len(), 5);

    // Check that no castle moves are included
    assert!(!contains_castle_move(
        &white_moves,
        ChessMove::WhiteKingsideCastle
    ));
    assert!(!contains_castle_move(
        &white_moves,
        ChessMove::WhiteQueensideCastle
    ));
    assert!(!contains_castle_move(
        &black_moves,
        ChessMove::BlackKingsideCastle
    ));
    assert!(!contains_castle_move(
        &black_moves,
        ChessMove::BlackQueensideCastle
    ));
}

fn contains_castle_move(moves: &Vec<ChessMove>, castle_move: ChessMove) -> bool {
    for m in moves {
        if *m == castle_move {
            return true;
        }
    }
    false
}
