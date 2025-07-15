use crate::{
    chess_moves::ChessMove,
    directions::*,
    piece::{BLACK_KING, WHITE_KING},
    position::Position,
    possible_moves::king::get_possible_moves,
};

#[test]
fn test_get_possible_black_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_possible_moves(&position, D8, BLACK_KING).len() == 0);
    assert!(get_possible_moves(&position, D3, BLACK_KING).len() == 8);
    assert!(get_possible_moves(&position, F2, BLACK_KING).len() == 8);
    assert!(get_possible_moves(&position, H6, BLACK_KING).len() == 3);
}

#[test]
fn test_get_possible_white_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_possible_moves(&position, D8, WHITE_KING).len() == 5);
    assert!(get_possible_moves(&position, D3, WHITE_KING).len() == 5);
    assert!(get_possible_moves(&position, F2, WHITE_KING).len() == 3);
    assert!(get_possible_moves(&position, H6, WHITE_KING).len() == 5);
}

fn contains_castle_move(moves: &Vec<ChessMove>, castle_move: ChessMove) -> bool {
    for m in moves {
        if *m == castle_move {
            return true;
        }
    }
    false
}
