use crate::{
    chess_moves::ChessMove, directions::*, position::Position,
    possible_moves::bishop::get_possible_white_moves,
};

#[test]
fn test_get_possible_white_moves() {
    let moves = get_possible_white_moves(&Position::default(), G4);
    assert!(moves.len() == 9);
    // left up
    assert!(contains(&moves, F5));
    // left down
    assert!(contains(&moves, F3));
    // right up
    assert!(contains(&moves, H5));
    // right down
    assert!(contains(&moves, H3));

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
