use crate::{
    directions::*,
    piece::*,
    position::{self, print::Print, Position},
    possible_moves::{
        common::{
            get_moves_for_king_at_square, get_moves_for_knight_at_square,
            get_moves_for_queen_at_square, get_moves_for_rook_at_square,
        },
        get_black_moves, get_white_moves,
    },
};

#[test]
fn test_all_moves_from_starting_position() {
    let position = Position::new_starting_position();
    assert_eq!(get_white_moves(&position).len(), 20);
}
#[test]
fn test_position1() {
    let white = Position::default()
        .put_piece(WHITE_PAWN, A2)
        .put_piece(WHITE_PAWN, C3)
        .put_piece(WHITE_PAWN, C4)
        .put_piece(WHITE_QUEEN, D3)
        .put_piece(WHITE_ROOK, F1)
        .put_piece(WHITE_PAWN, F2)
        .put_piece(WHITE_KNIGHT, F3)
        .put_piece(WHITE_KING, G1)
        .put_piece(WHITE_BISHOP, G2)
        .put_piece(WHITE_PAWN, G3)
        .put_piece(WHITE_PAWN, H2);
    let positions = get_white_moves(&white);
    assert!(positions.len() == 35);
    let black = Position::default()
        .put_piece(BLACK_PAWN, A7)
        .put_piece(BLACK_PAWN, B7)
        .put_piece(BLACK_PAWN, D6)
        .put_piece(BLACK_PAWN, F7)
        .put_piece(BLACK_PAWN, G7)
        .put_piece(BLACK_PAWN, H6)
        .put_piece(BLACK_BISHOP, B6)
        .put_piece(BLACK_ROOK, E4)
        .put_piece(BLACK_BISHOP, G4)
        .put_piece(BLACK_QUEEN, G6)
        .put_piece(BLACK_KING, G8);
    let positions = get_black_moves(&black);
    assert_eq!(positions.len(), 44);

    let all = Position::default()
        .put_piece(WHITE_PAWN, A2)
        .put_piece(WHITE_PAWN, C3)
        .put_piece(WHITE_PAWN, C4)
        .put_piece(WHITE_QUEEN, D3)
        .put_piece(WHITE_ROOK, F1)
        .put_piece(WHITE_PAWN, F2)
        .put_piece(WHITE_KNIGHT, F3)
        .put_piece(WHITE_KING, G1)
        .put_piece(WHITE_BISHOP, G2)
        .put_piece(WHITE_PAWN, G3)
        .put_piece(WHITE_PAWN, H2)
        .put_piece(BLACK_PAWN, A7)
        .put_piece(BLACK_PAWN, B7)
        .put_piece(BLACK_PAWN, D6)
        .put_piece(BLACK_PAWN, F7)
        .put_piece(BLACK_PAWN, G7)
        .put_piece(BLACK_PAWN, H6)
        .put_piece(BLACK_BISHOP, B6)
        .put_piece(BLACK_ROOK, E4)
        .put_piece(BLACK_BISHOP, G4)
        .put_piece(BLACK_QUEEN, G6)
        .put_piece(BLACK_KING, G8);

    assert!(39 == get_black_moves(&all).len());
    assert!(29 == get_white_moves(&all).len());
}

#[test]
fn test_bishop_white_moves() {
    let position = Position::default().put_piece(WHITE_BISHOP, G4);
    let positions = get_white_moves(&position);
    assert_eq!(positions.len(), 9);
    let mut left_up = false;
    let mut left_down = false;
    let mut right_up = false;
    let mut right_down = false;
    let mut not_valid = false;

    for position in positions {
        if position.is_occupied_by_piece(F5, WHITE_BISHOP) {
            left_up = true;
        }
        if position.is_occupied_by_piece(F3, WHITE_BISHOP) {
            left_down = true;
        }
        if position.is_occupied_by_piece(H5, WHITE_BISHOP) {
            right_up = true;
        }
        if position.is_occupied_by_piece(H3, WHITE_BISHOP) {
            right_down = true;
        }
        if position.is_occupied_by_piece(B2, WHITE_BISHOP) {
            not_valid = true;
        }
    }

    assert!(left_up);
    assert!(left_down);
    assert!(right_up);
    assert!(right_down);
    assert!(!not_valid);
}
#[test]
fn test_king_black_moves() {
    let position: Position = Position::new_starting_position();
    assert!(get_moves_for_king_at_square(&position, BLACK_KING, D8).len() == 0);
    assert!(get_moves_for_king_at_square(&position, BLACK_KING, D3).len() == 8);
    assert!(get_moves_for_king_at_square(&position, BLACK_KING, F6).len() == 5);
    assert!(get_moves_for_king_at_square(&position, BLACK_KING, H6).len() == 3);
}

#[test]
fn test_king_white_moves() {
    let position: Position = Position::new_starting_position();

    assert!(get_moves_for_king_at_square(&position, WHITE_KING, D8).len() == 5);
    assert!(get_moves_for_king_at_square(&position, WHITE_KING, D3).len() == 5);
    assert!(get_moves_for_king_at_square(&position, WHITE_KING, F2).len() == 3);
    assert!(get_moves_for_king_at_square(&position, WHITE_KING, H6).len() == 5);
}

#[test]
fn test_knight_moves() {
    let mut position = Position::default();
    position = position
        .put_piece(WHITE_KNIGHT, E4)
        .put_piece(WHITE_PAWN, C5)
        .put_piece(BLACK_PAWN, G2);

    let positions = get_moves_for_knight_at_square(&position, WHITE_KNIGHT, E4);

    let mut found_c3 = false;
    let mut found_d6 = false;
    let mut found_f6 = false;
    let mut found_d2 = false;
    let mut found_f2 = false;
    let mut found_g3 = false;
    let mut found_g5 = false;
    let mut found_not_c5 = true;

    println!("{:?}", positions.len());
    for position in positions {
        if position.is_occupied_by_piece(C3, WHITE_KNIGHT) {
            found_c3 = true;
        }
        if position.is_occupied_by_piece(D6, WHITE_KNIGHT) {
            found_d6 = true;
        }
        if position.is_occupied_by_piece(F6, WHITE_KNIGHT) {
            found_f6 = true;
        }
        if position.is_occupied_by_piece(D2, WHITE_KNIGHT) {
            found_d2 = true;
        }
        if position.is_occupied_by_piece(F2, WHITE_KNIGHT) {
            found_f2 = true;
        }
        if position.is_occupied_by_piece(G3, WHITE_KNIGHT) {
            found_g3 = true;
        }
        if position.is_occupied_by_piece(G5, WHITE_KNIGHT) {
            found_g5 = true;
        }
        if position.is_occupied_by_piece(C5, WHITE_KNIGHT) {
            found_not_c5 = false;
        }
    }

    assert!(found_c3);
    assert!(found_d6);
    assert!(found_f6);
    assert!(found_d2);
    assert!(found_f2);
    assert!(found_g3);
    assert!(found_g5);
    assert!(found_not_c5);
}
#[test]
fn test_queen_white_moves() {
    let positions = get_moves_for_queen_at_square(&Position::default(), WHITE_QUEEN, G4);
    assert!(positions.len() == 23);

    let mut found_up = false;
    let mut found_down = false;
    let mut found_left = false;
    let mut found_right = false;
    let mut found_not = true;
    let mut found_left_up = false;
    let mut found_left_down = false;
    let mut found_right_up = false;
    let mut found_right_down = false;

    for position in positions {
        if position.is_occupied_by_piece(G3, WHITE_QUEEN) {
            found_down = true;
        }
        if position.is_occupied_by_piece(G8, WHITE_QUEEN) {
            found_up = true;
        }
        if position.is_occupied_by_piece(A4, WHITE_QUEEN) {
            found_left = true;
        }
        if position.is_occupied_by_piece(H4, WHITE_QUEEN) {
            found_right = true;
        }
        if position.is_occupied_by_piece(B2, WHITE_QUEEN) {
            found_not = false;
        }
        if position.is_occupied_by_piece(F5, WHITE_QUEEN) {
            found_left_up = true;
        }
        if position.is_occupied_by_piece(F3, WHITE_QUEEN) {
            found_left_down = true;
        }
        if position.is_occupied_by_piece(H5, WHITE_QUEEN) {
            found_right_up = true;
        }
        if position.is_occupied_by_piece(H3, WHITE_QUEEN) {
            found_right_down = true;
        }
    }
    // down
    assert!(found_down);
    // up
    assert!(found_up);
    // left
    assert!(found_left);
    // right
    assert!(found_right);
    // not
    assert!(found_not);
    // left up
    assert!(found_left_up);
    // left down
    assert!(found_left_down);
    // right up
    assert!(found_right_up);
    // right down
    assert!(found_right_down);
}
#[test]
fn test_rook_white_moves() {
    let positions = get_moves_for_rook_at_square(&Position::default(), BLACK_ROOK, G4);

    assert!(positions.len() == 14);
    let mut found_up = false;
    let mut found_down = false;
    let mut found_left = false;
    let mut found_right = false;
    let mut found_not = true;

    for position in positions {
        if position.is_occupied_by_piece(G3, BLACK_ROOK) {
            found_down = true;
        }
        if position.is_occupied_by_piece(G8, BLACK_ROOK) {
            found_up = true;
        }
        if position.is_occupied_by_piece(A4, BLACK_ROOK) {
            found_left = true;
        }
        if position.is_occupied_by_piece(H4, BLACK_ROOK) {
            found_right = true;
        }
        if position.is_occupied_by_piece(B2, BLACK_ROOK) {
            found_not = false;
        }
    }
    // down
    assert!(found_down);
    // up
    assert!(found_up);
    // left
    assert!(found_left);
    // right
    assert!(found_right);
    // not
    assert!(found_not);
}
