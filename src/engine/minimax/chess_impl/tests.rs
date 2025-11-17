use crate::engine::{
    directions::squares::*,
    minimax::chess_impl::get_best_move,
    piece::*,
    position::{print::Print, Position},
};

#[test]
fn test_get_best_move() {
    let position = Position::default()
        .put_piece(Piece::BlackKing, D7)
        .put_piece(Piece::WhiteKing, D2)
        .put_piece(Piece::WhitePawn, B6)
        .put_piece(Piece::BlackKnight, A7);
    position.print_board();
    if let Some(best_position) = get_best_move(position) {
        best_position.print_board();
        assert!(best_position.is_occupied_by_piece(A7, Piece::WhitePawn));
    }
}

#[test]
fn test_get_best_move2() {
    let position = Position::default()
        .put_piece(Piece::BlackKing, D7)
        .put_piece(Piece::WhiteKing, D2)
        .put_piece(Piece::BlackPawn, B7)
        .put_piece(Piece::WhiteKnight, A6)
        .toggle_player();
    position.print_board();
    if let Some(best_position) = get_best_move(position) {
        assert!(best_position.is_occupied_by_piece(A6, Piece::BlackPawn));
    }
}
