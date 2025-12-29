use crate::engine::directions::squares::*;
use crate::engine::heuristic::*;
use crate::engine::piece::Piece::{self, *};
use crate::engine::position::*;

#[test]
fn test_queen_loss() {
    let position = Position::new_starting_position()
        .remove_piece(E2)
        .put_piece(Piece::WhitePawn, E4)
        .remove_piece(C7)
        .put_piece(Piece::BlackPawn, C6)
        .remove_piece(D1)
        .put_piece(Piece::WhiteQueen, H5)
        .remove_piece(G7)
        .remove_piece(H5)
        .put_piece(Piece::BlackKnight, H5);
    let evaluation = heuristic(&position);
    println!("{:?}", evaluation);
}

#[test]
fn test_isolated_pawns() {
    let position = Position::default()
        .put_piece(WhitePawn, A1)
        .put_piece(WhitePawn, C1)
        .put_piece(WhitePawn, D1)
        .put_piece(WhitePawn, F1);
    assert_eq!(
        pawn_structures::get_isolated_pawns(&position, WhitePawn),
        -20.0
    );
}
#[test]
fn test_isolated_pawns_2() {
    let position = Position::default()
        .put_piece(BlackPawn, A1)
        .put_piece(BlackPawn, C3)
        .put_piece(BlackPawn, D4)
        .put_piece(BlackPawn, F8);
    assert_eq!(
        pawn_structures::get_isolated_pawns(&position, BlackPawn),
        -20.0
    );
}

#[test]
fn test_doubled_pawns() {
    let position = Position::default()
        .put_piece(BlackPawn, F2)
        .put_piece(BlackPawn, F3)
        .put_piece(BlackPawn, F4)
        .put_piece(WhitePawn, D2)
        .put_piece(WhitePawn, D4);
    assert_eq!(
        pawn_structures::get_doubled_pawns(&position, BlackPawn),
        -30.0
    );
    assert_eq!(
        pawn_structures::get_doubled_pawns(&position, WhitePawn),
        -15.0
    );

    let position_2 = Position::default().put_piece(BlackPawn, F2);
    assert_eq!(
        pawn_structures::get_doubled_pawns(&position_2, BlackPawn),
        0.0
    );
}
#[test]
fn test_equal_material() {
    let position = Position::new_starting_position();
    let score = material::count_white(&position) - material::count_black(&position);
    assert_eq!(score, 0.0);
}

#[test]
fn test_white_queen_missing() {
    let position = Position::new_starting_position().remove_piece(D1);
    assert_eq!(heuristic(&position).total, -88.0);
}

#[test]
fn test_black_queen_missing() {
    let position = Position::new_starting_position().remove_piece(D8);
    assert_eq!(
        material::count_white(&position) - material::count_black(&position),
        90.0
    );
    assert_eq!(
        mobility::count_white(&position) - mobility::count_black(&position),
        -1.5
    );
    assert_eq!(
        squares::count_white(&position) - squares::count_black(&position),
        -0.5
    );
    assert_eq!(heuristic(&position).total, 88.0);
}
