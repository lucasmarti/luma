use crate::engine::movegen::*;
use crate::engine::{
    piece::*,
    position::{print::Print, Position},
    search_algorithms::get_best_move,
};

#[test]
fn test_get_best_move() {
    let position = Position::default()
        .put_piece(BLACK_KING, D7)
        .put_piece(WHITE_KING, D2)
        .put_piece(WHITE_PAWN, B6)
        .put_piece(BLACK_KNIGHT, A7);
    position.print_board();
    if let Some(best_move) = get_best_move(position) {
        best_move.position.print_board();
        assert!(best_move.position.is_occupied_by_piece(A7, WHITE_PAWN));
    }
}

#[test]
fn test_get_best_move2() {
    let position = Position::default()
        .put_piece(BLACK_KING, D7)
        .put_piece(WHITE_KING, D2)
        .put_piece(BLACK_PAWN, B7)
        .put_piece(WHITE_KNIGHT, A6)
        .toggle_player();
    position.print_board();
    if let Some(best_move) = get_best_move(position) {
        assert!(best_move.position.is_occupied_by_piece(A6, BLACK_PAWN));
    }
}
