use std::default;

use crate::{
    chess_moves::ChessMove,
    directions::*,
    piece::{BLACK_PAWN, WHITE_KNIGHT, WHITE_PAWN},
    position::Position,
    possible_moves::knight::get_possible_white_moves,
};

#[test]
fn test_knight_moves() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KNIGHT, E4)
        .put_piece(WHITE_PAWN, C5)
        .put_piece(BLACK_PAWN, D2);

    let moves = get_possible_white_moves(&position, E4);

    assert!(!contains(&moves, C5));
    assert!(contains(&moves, C3));
    assert!(contains(&moves, D6));
    assert!(contains(&moves, F6));
    assert!(contains(&moves, D2));
    assert!(contains(&moves, F2));
    assert!(contains(&moves, G3));
    assert!(contains(&moves, G5));
    assert!(contains(&moves, D2));
}

fn contains(moves: &Vec<ChessMove>, field: u32) -> bool {
    for m in moves {
        match m {
            ChessMove::Progress(progress) => {
                if progress.to == field {
                    return true;
                }
            }
            _ => {}
        }
    }
    return false;
}
