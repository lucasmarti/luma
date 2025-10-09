use crate::engine::{
    directions::squares::{A1, B1, C1, E1, F1, H3},
    position::bitboard::Bitboard,
};
#[test]
fn test_count_ones() {
    let bits: u64 = 1 << 0 | 1 << 1 | 1 << 2;
    let bitboard = Bitboard(bits);
    assert_eq!(bitboard.count_ones(), 3);
}

#[test]
fn test_from() {
    let bitboard = Bitboard::from(C1);
    assert_eq!(bitboard.0, 4);
}

#[test]
fn test_from_vec() {
    let bitboard = Bitboard::from_vec(vec![A1, B1, C1]);
    assert_eq!(bitboard.0, 7)
}

#[test]
fn test_contains() {
    let bitboard = Bitboard::from(E1);
    assert!(bitboard.contains(E1));
}
#[test]
fn test_default() {
    let bitboard = Bitboard::default();
    assert_eq!(bitboard.count_ones(), 0);
}

#[test]
fn test_remove_bit() {
    let mut bitboard = Bitboard::from_vec(vec![A1, B1, C1]);
    assert_eq!(bitboard.count_ones(), 3);
    bitboard.remove_bit(C1);
    bitboard.remove_bit(B1);
    assert_eq!(bitboard.count_ones(), 1);
}

#[test]
fn test_iterator() {
    let bitboard = Bitboard::from_vec(vec![C1, F1, H3]);
    let mut iter = bitboard.iter();
    assert_eq!(iter.next(), Some(C1));
    assert_eq!(iter.next(), Some(F1));
    assert_eq!(iter.next(), Some(H3));
    assert_eq!(iter.next(), None);
    let mut sum = 0;
    for square in bitboard.iter() {
        sum = sum + square.as_index();
    }
    assert_eq!(sum, 30);
}
