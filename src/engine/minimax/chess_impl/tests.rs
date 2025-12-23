use crate::engine::{
    directions::squares::*,
    minimax::{
        chess_impl::{get_best_move, Node},
        Minimax, MIN_VALUE,
    },
    piece::*,
    position::{print::Print, Position},
};

#[test]
fn test_evaluate() {
    let checkmate_white_position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::WhiteKing, A1)
        .put_piece(Piece::BlackQueen, A8)
        .put_piece(Piece::BlackRook, B8);
    let mut node = Node {
        position: checkmate_white_position,
        children: Vec::new(),
    };

    assert_eq!(node.evaluate(), MIN_VALUE);

    let draw_position = Position::default()
        .put_piece(Piece::BlackKing, E8)
        .put_piece(Piece::WhiteKing, A1)
        .put_piece(Piece::BlackQueen, B8)
        .put_piece(Piece::BlackRook, H2);
    let node2 = Node {
        position: draw_position,
        children: Vec::new(),
    };
    assert_eq!(node2.evaluate(), 0.0);
}

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
