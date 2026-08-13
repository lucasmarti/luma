use crate::engine::evaluation::pawn_structures::get_passed_pawns;
use crate::engine::evaluation::*;
use crate::engine::movegen::*;
use crate::engine::piece::*;
use crate::engine::position::*;

#[test]
fn test_queen_loss() {
    let mut position = Position::starting()
        .with_piece(WHITE_PAWN, E4)
        .with_piece(BLACK_PAWN, C6)
        .with_piece(WHITE_QUEEN, H5)
        .with_piece(BLACK_KNIGHT, H5);
    position.remove_piece(E2);
    position.remove_piece(C7);
    position.remove_piece(D1);
    position.remove_piece(G7);
    position.remove_piece(H5);
    let evaluation = Evaluation::new(&position);
    println!("{:?}", evaluation);
}

#[test]
fn test_isolated_pawns() {
    let position = Position::default()
        .with_piece(WHITE_PAWN, A1)
        .with_piece(WHITE_PAWN, C1)
        .with_piece(WHITE_PAWN, D1)
        .with_piece(WHITE_PAWN, F1);
    assert_eq!(
        pawn_structures::get_isolated_pawns(&position, Color::White),
        -8.0
    );
}
#[test]
fn test_isolated_pawns_2() {
    let position = Position::default()
        .with_piece(BLACK_PAWN, A1)
        .with_piece(BLACK_PAWN, C3)
        .with_piece(BLACK_PAWN, D4)
        .with_piece(BLACK_PAWN, F8);
    assert_eq!(
        pawn_structures::get_isolated_pawns(&position, Color::Black),
        -8.0
    );
}

#[test]
fn test_doubled_pawns() {
    let position = Position::default()
        .with_piece(BLACK_PAWN, F2)
        .with_piece(BLACK_PAWN, F3)
        .with_piece(BLACK_PAWN, F4)
        .with_piece(WHITE_PAWN, D2)
        .with_piece(WHITE_PAWN, D4);
    assert_eq!(
        pawn_structures::get_doubled_pawns(&position, Color::Black),
        -6.0
    );
    assert_eq!(
        pawn_structures::get_doubled_pawns(&position, Color::White),
        -3.0
    );

    let position_2 = Position::default().with_piece(BLACK_PAWN, F2);
    assert_eq!(
        pawn_structures::get_doubled_pawns(&position_2, Color::Black),
        0.0
    );
}
#[test]
fn test_equal_material() {
    let position = Position::starting();
    let score = material::count_white(&position) - material::count_black(&position);
    assert_eq!(score, 0.0);
}

#[test]
fn test_white_queen_missing() {
    let mut position = Position::starting();
    position.remove_piece(D1);
    assert_eq!(Evaluation::new(&position).score, -89.44995);
}

#[test]
fn test_black_queen_missing() {
    let mut position = Position::starting();
    position.remove_piece(D8);
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
    let position1 = Position::default().with_piece(WHITE_PAWN, A4);
    assert_eq!(get_passed_pawns(&position1, Color::White), 15.0);

    let position2 = Position::default()
        .with_piece(BLACK_PAWN, A4)
        .with_piece(WHITE_PAWN, A3);
    assert_eq!(get_passed_pawns(&position2, Color::Black), 0.0);

    let position3 = Position::default()
        .with_piece(BLACK_PAWN, A4)
        .with_piece(WHITE_PAWN, B3);
    assert_eq!(get_passed_pawns(&position3, Color::Black), 0.0);
}
