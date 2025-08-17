use crate::engine::position::bitboard::Bitboard;
#[test]
fn test_count_ones() {
    let bits: u64 = 1 << 0 | 1 << 1 | 1 << 2;
    let bitboard = Bitboard(bits);
    assert_eq!(bitboard.count_ones(), 3);
}

#[test]
fn test_from() {
    let bitboard = Bitboard::from(2);
    assert_eq!(bitboard.0, 4);
}

#[test]
fn test_from_vec() {
    let bitboard = Bitboard::from_vec(vec![0, 1, 2]);
    assert_eq!(bitboard.0, 7)
}

#[test]
fn test_contains() {
    let bitboard = Bitboard::from(4);
    assert!(bitboard.contains(4));
}
#[test]
fn test_default() {
    let bitboard = Bitboard::default();
    assert_eq!(bitboard.count_ones(), 0);
}

#[test]
fn test_remove_bit() {
    let mut bitboard = Bitboard::from_vec(vec![0, 1, 2]);
    assert_eq!(bitboard.count_ones(), 3);
    bitboard.remove_bit(2);
    bitboard.remove_bit(1);
    assert_eq!(bitboard.count_ones(), 1);
}

#[test]
fn test_iterator() {
    let bitboard = Bitboard::from_vec(vec![2, 5, 23]);
    let mut iter = bitboard.iter();
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), Some(23));
    assert_eq!(iter.next(), None);
    let mut sum = 0;
    for value in bitboard.iter() {
        sum = sum + value;
    }
    assert_eq!(sum, 30);
}
