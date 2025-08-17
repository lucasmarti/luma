use crate::engine::{
    directions::squares::*,
    minimax::chess_impl::get_best_move,
    piece::{BLACK_KING, BLACK_KNIGHT, WHITE_KING, WHITE_PAWN},
    position::{print::Print, Position},
};

#[test]
fn test_get_best_move() {
    let position = Position::default()
        .put_piece(BLACK_KING, D7)
        .put_piece(WHITE_KING, D2)
        .put_piece(WHITE_PAWN, B6)
        .put_piece(BLACK_KNIGHT, A7);
    position.print_board();
    if let Some(best_position) = get_best_move(position) {
        assert!(best_position.is_occupied_by_piece(A7, WHITE_PAWN));
    }
}
