use crate::bitboard::*;
#[test]
fn test_count_ones() {
    let bits: u64 = 1 << 0 | 1 << 1 | 1 << 2;
    let bitboard = Bitboard(bits);
    assert_eq!(bitboard.count_ones(), 3);
}
#[test]
fn test_new_empty() {
    let bitboard = Bitboard::new();
    assert_eq!(bitboard.count_ones(), 0);
}

#[test]
fn test_from() {
    let bitboard = Bitboard::from(2);
    assert_eq!(bitboard.0, 4);
}

#[test]
fn test_set_bit() {
    let mut bitboard = Bitboard::from(2);
    bitboard = bitboard.set_bit(1);
    assert_eq!(bitboard.0, 6);
}

#[test]
fn test_from_vec() {
    let bitboard = Bitboard::from_vec(vec![0, 1, 2]);
    assert_eq!(bitboard.0, 7)
}
