use crate::{
    bitboard::Bitboard,
    piece::{Color, Piece, Type},
    position,
};

use super::{print::Print, Position};

#[test]
fn test_new_starting_position() {
    let position: Position = Position::new_starting_position();
    position.print_board();
}
#[test]
fn test_get_black() {
    let position = Position::new_starting_position();
    assert_eq!(position.get_black().count_ones(), 16);
}
#[test]
fn test_get_white() {
    let position = Position::new_starting_position();
    assert_eq!(position.get_white().count_ones(), 16);
}

#[test]
fn test_get_all() {
    let position = Position::new_starting_position();
    assert_eq!(position.get_all().count_ones(), 32);
}

#[test]
fn test_move_white_king() {
    let position = Position::new_starting_position();
    let white_king = Piece {
        color: Color::WHITE,
        type_: Type::KING,
    };
    let new_position = position.move_piece(white_king, 4, 12);
    assert_eq!(position.white_king.contains(4), true);
    assert_eq!(new_position.white_king.contains(4), false);
    assert_eq!(new_position.white_king.contains(12), true);
}

#[test]
fn test_remove_white_king() {
    let position = Position::new_starting_position();
    let white_king = Piece {
        color: Color::WHITE,
        type_: Type::KING,
    };
    let index = 4;
    let new_position = position.remove_piece(index);
    assert_eq!(position.white_king.count_ones(), 1);
    assert_eq!(new_position.white_king.count_ones(), 0);
}

#[test]
fn test_put_white_king() {
    let position = Position::new_starting_position();
    let white_king = Piece {
        color: Color::WHITE,
        type_: Type::KING,
    };
    let index = 12;
    let new_position = position.put_piece(white_king, index);
    assert_eq!(position.white_king.contains(index), false);
    assert_eq!(new_position.white_king.contains(index), true);
}
