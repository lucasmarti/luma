use crate::directions::*;

#[test]
fn test_up() {
    assert_eq!(up(1).unwrap(), 9);
    assert_eq!(up(56), None);
}
#[test]
fn test_down() {
    assert_eq!(down(7), None);
    assert_eq!(down(22).unwrap(), 14);
}

#[test]
fn test_left() {
    assert_eq!(left(2).unwrap(), 1);
    assert_eq!(left(8), None);
}
#[test]
fn test_right() {
    assert_eq!(right(15), None);
    assert_eq!(right(33).unwrap(), 34);
}
#[test]
fn test_up_left() {
    assert_eq!(up_left(42).unwrap(), 49);
    assert_eq!(up_left(58), None);
    assert_eq!(up_left(40), None);
}
#[test]
fn test_up_right() {
    assert_eq!(up_right(22).unwrap(), 31);
    assert_eq!(up_right(23), None);
    assert_eq!(up_right(61), None);
}
#[test]
fn test_down_left() {
    assert_eq!(down_left(20).unwrap(), 11);
    assert_eq!(down_left(3), None);
    assert_eq!(down_left(8), None);
}
#[test]
fn test_down_right() {
    assert_eq!(down_right(19).unwrap(), 12);
    assert_eq!(down_right(3), None);
    assert_eq!(down_right(15), None);
}
#[test]
fn test_left_left_down() {
    assert_eq!(left_left_down(25), None);
    assert_eq!(left_left_down(27).unwrap(), 17);
    assert_eq!(left_left_down(5), None);
}
#[test]
fn test_left_left_up() {
    assert_eq!(left_left_up(25), None);
    assert_eq!(left_left_up(63), None);
    assert_eq!(left_left_up(28).unwrap(), 34);
}

#[test]
fn test_right_right_down() {
    assert_eq!(right_right_down(30), None);
    assert_eq!(right_right_down(27).unwrap(), 21);
    assert_eq!(right_right_down(3), None);
}

#[test]
fn test_right_right_up() {
    assert_eq!(right_right_up(30), None);
    assert_eq!(right_right_up(58), None);
    assert_eq!(right_right_up(10).unwrap(), 20);
}
#[test]
fn test_up_up_left() {
    assert_eq!(up_up_left(8), None);
    assert_eq!(up_up_left(49), None);
    assert_eq!(up_up_left(19).unwrap(), 34);
}
#[test]
fn test_up_up_right() {
    assert_eq!(up_up_right(51), None);
    assert_eq!(up_up_right(7), None);
    assert_eq!(up_up_right(18).unwrap(), 35);
}
#[test]
fn test_down_down_left() {
    assert_eq!(down_down_left(12), None);
    assert_eq!(down_down_left(16), None);
    assert_eq!(down_down_left(28).unwrap(), 11);
}
#[test]
fn test_down_down_right() {
    assert_eq!(down_down_right(12), None);
    assert_eq!(down_down_right(39), None);
    assert_eq!(down_down_right(43).unwrap(), 28);
}
