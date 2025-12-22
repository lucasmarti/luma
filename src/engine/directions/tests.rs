use crate::engine::directions::*;
#[test]
fn test_get_column() {
    assert_eq!(get_column(A1), 1);
    assert_eq!(get_column(F1), 6);
    assert_eq!(get_column(H7), 8);
}

#[test]
fn test_up() {
    assert_eq!(up(A1).unwrap(), A2);
    assert_eq!(up(A8), None);
}
#[test]
fn test_down() {
    assert_eq!(down(H1), None);
    assert_eq!(down(G3).unwrap(), G2);
}

#[test]
fn test_left() {
    assert_eq!(left(C1).unwrap(), B1);
    assert_eq!(left(A2), None);
}
#[test]
fn test_right() {
    assert_eq!(right(H2), None);
    assert_eq!(right(B5).unwrap(), C5);
}
#[test]
fn test_up_left() {
    assert_eq!(up_left(C1).unwrap(), B2);
    assert_eq!(up_left(D8), None);
    assert_eq!(up_left(A6), None);
}
#[test]
fn test_up_right() {
    assert_eq!(up_right(G3).unwrap(), H4);
    assert_eq!(up_right(H3), None);
    assert_eq!(up_right(F8), None);
}
#[test]
fn test_down_left() {
    assert_eq!(down_left(E3).unwrap(), D2);
    assert_eq!(down_left(D1), None);
    assert_eq!(down_left(A2), None);
}
#[test]
fn test_down_right() {
    assert_eq!(down_right(D3).unwrap(), E2);
    assert_eq!(down_right(D1), None);
    assert_eq!(down_right(H2), None);
}
#[test]
fn test_left_left_down() {
    assert_eq!(left_left_down(B4), None);
    assert_eq!(left_left_down(D4).unwrap(), B3);
    assert_eq!(left_left_down(F1), None);
}
#[test]
fn test_left_left_up() {
    assert_eq!(left_left_up(B4), None);
    assert_eq!(left_left_up(H8), None);
    assert_eq!(left_left_up(E4).unwrap(), C5);
}

#[test]
fn test_right_right_down() {
    assert_eq!(right_right_down(G4), None);
    assert_eq!(right_right_down(D4).unwrap(), F3);
    assert_eq!(right_right_down(D1), None);
}

#[test]
fn test_right_right_up() {
    assert_eq!(right_right_up(G4), None);
    assert_eq!(right_right_up(D8), None);
    assert_eq!(right_right_up(C2).unwrap(), E3);
}
#[test]
fn test_up_up_left() {
    assert_eq!(up_up_left(A2), None);
    assert_eq!(up_up_left(B7), None);
    assert_eq!(up_up_left(D3).unwrap(), C5);
}
#[test]
fn test_up_up_right() {
    assert_eq!(up_up_right(D7), None);
    assert_eq!(up_up_right(H1), None);
    assert_eq!(up_up_right(C3).unwrap(), D5);
}
#[test]
fn test_down_down_left() {
    assert_eq!(down_down_left(E2), None);
    assert_eq!(down_down_left(A3), None);
    assert_eq!(down_down_left(E4).unwrap(), D2);
}
#[test]
fn test_down_down_right() {
    assert_eq!(down_down_right(E2), None);
    assert_eq!(down_down_right(H5), None);
    assert_eq!(down_down_right(D6).unwrap(), E4);
}

#[test]
fn test_row_2() {
    assert!(is_in_row_2(F2));
}
#[test]
fn test_row_7() {
    assert!(is_in_row_7(E7));
}
