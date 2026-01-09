use crate::engine::directions::squares::*;
use crate::engine::evaluation::pawn_structures::get_passed_pawns;
use crate::engine::piece::Piece::{self, *};
use crate::engine::position::*;
use crate::engine::{evaluation::*, position};

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
    let evaluation = Evaluation::new(&position);
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
        -8.0
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
        -8.0
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
        -6.0
    );
    assert_eq!(
        pawn_structures::get_doubled_pawns(&position, WhitePawn),
        -3.0
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
    assert_eq!(Evaluation::new(&position).score, -89.44995);
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
        -0.049999952
    );
    assert_eq!(
        squares::count_white(&position) - squares::count_black(&position),
        -0.5
    );
    assert_eq!(Evaluation::new(&position).score, 89.44995);
}

#[test]
fn test_get_passed_pawns() {
    let position1 = Position::default().put_piece(Piece::WhitePawn, A4);
    assert_eq!(get_passed_pawns(&position1, WhitePawn), 15.0);

    let position2 = Position::default()
        .put_piece(Piece::BlackPawn, A4)
        .put_piece(WhitePawn, A3);
    assert_eq!(get_passed_pawns(&position2, BlackPawn), 0.0);

    let position3 = Position::default()
        .put_piece(Piece::BlackPawn, A4)
        .put_piece(WhitePawn, B3);
    assert_eq!(get_passed_pawns(&position3, BlackPawn), 0.0);
}
