use crate::{
    chess_moves::ChessMove, directions::*, position::Position,
    possible_moves::rook::get_possible_black_moves,
};

#[test]
fn test_get_possible_white_moves() {
    let moves = get_possible_black_moves(&Position::default(), G4);
    assert!(moves.len() == 14);
    // down
    assert!(contains(&moves, G3));
    // up
    assert!(contains(&moves, G8));
    // left
    assert!(contains(&moves, A4));
    // right
    assert!(contains(&moves, H4));
    // not
    assert!(!contains(&moves, B2));
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
