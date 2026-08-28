use crate::engine::movegen::*;
use crate::engine::{
    piece::*,
    position::{print::Print, Position},
    search_algorithms::search_best_move,
};

#[test]
fn test_get_best_move() {
    let position = Position::default()
        .with_piece(BLACK_KING, D7)
        .with_piece(WHITE_KING, D2)
        .with_piece(WHITE_PAWN, B6)
        .with_piece(BLACK_KNIGHT, A7);
    position.print_board();
    if let Some(best_move) = search_best_move(position) {
        best_move.position.print_board();
        assert!(best_move.position.is_occupied_by_piece(A7, WHITE_PAWN));
    }
}

#[test]
fn test_get_best_move2() {
    let mut position = Position::default()
        .with_piece(BLACK_KING, D7)
        .with_piece(WHITE_KING, D2)
        .with_piece(BLACK_PAWN, B7)
        .with_piece(WHITE_KNIGHT, A6);
    position.toggle_player();
    position.print_board();
    if let Some(best_move) = search_best_move(position) {
        assert!(best_move.position.is_occupied_by_piece(A6, BLACK_PAWN));
    }
}
