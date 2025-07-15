use crate::{
    chess_moves::ChessMove, directions::*, piece::WHITE_QUEEN, position::Position,
    possible_moves::queen::get_possible_moves,
};

#[test]
fn test_get_possible_white_moves() {
    let moves = get_possible_moves(&Position::default(), G4, WHITE_QUEEN);
    assert!(moves.len() == 23);
    // down
    assert!(contains(&moves, G3));
    // up
    assert!(contains(&moves, G8));
    // left
    assert!(contains(&moves, A4));
    // right
    assert!(contains(&moves, H4));
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
