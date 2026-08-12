use super::*;
use crate::engine::movegen::*;
#[test]
fn from_maps_castling_squares() {
    assert_eq!(CastlingRights::from(A1), CastlingRights::WHITE_QUEENSIDE);
    assert_eq!(
        CastlingRights::from(E1),
        CastlingRights::WHITE_QUEENSIDE | CastlingRights::WHITE_KINGSIDE
    );
    assert_eq!(CastlingRights::from(H1), CastlingRights::WHITE_KINGSIDE);

    assert_eq!(CastlingRights::from(A8), CastlingRights::BLACK_QUEENSIDE);
    assert_eq!(
        CastlingRights::from(E8),
        CastlingRights::BLACK_QUEENSIDE | CastlingRights::BLACK_KINGSIDE
    );
    assert_eq!(CastlingRights::from(H8), CastlingRights::BLACK_KINGSIDE);
}

#[test]
fn from_returns_none_for_other_squares() {
    assert_eq!(CastlingRights::from(B1), CastlingRights::NONE);
    assert_eq!(CastlingRights::from(D1), CastlingRights::NONE);
    assert_eq!(CastlingRights::from(F1), CastlingRights::NONE);

    assert_eq!(CastlingRights::from(B8), CastlingRights::NONE);
    assert_eq!(CastlingRights::from(D8), CastlingRights::NONE);
    assert_eq!(CastlingRights::from(F8), CastlingRights::NONE);
}

#[test]
fn all_contains_every_right() {
    let all = CastlingRights::all();

    assert!(all.has(CastlingRights::BLACK_KINGSIDE));
    assert!(all.has(CastlingRights::BLACK_QUEENSIDE));
    assert!(all.has(CastlingRights::WHITE_KINGSIDE));
    assert!(all.has(CastlingRights::WHITE_QUEENSIDE));
}

#[test]
fn bitor_combines_rights() {
    let rights = CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE;

    assert_eq!(rights, CastlingRights::from(E1));
}

#[test]
fn add_adds_rights() {
    let mut rights = CastlingRights::NONE;

    rights.add(CastlingRights::WHITE_KINGSIDE);
    assert_eq!(rights, CastlingRights::WHITE_KINGSIDE);

    rights.add(CastlingRights::WHITE_QUEENSIDE);
    assert_eq!(
        rights,
        CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE
    );
}

#[test]
fn add_preserves_existing_rights() {
    let mut rights = CastlingRights::WHITE_KINGSIDE;

    rights.add(CastlingRights::BLACK_KINGSIDE);

    assert!(rights.has(CastlingRights::WHITE_KINGSIDE));
    assert!(rights.has(CastlingRights::BLACK_KINGSIDE));
}

#[test]
fn remove_removes_rights() {
    let mut rights = CastlingRights::all();

    rights.remove(CastlingRights::WHITE_KINGSIDE);

    assert!(!rights.has(CastlingRights::WHITE_KINGSIDE));
    assert!(rights.has(CastlingRights::WHITE_QUEENSIDE));
    assert!(rights.has(CastlingRights::BLACK_KINGSIDE));
    assert!(rights.has(CastlingRights::BLACK_QUEENSIDE));
}

#[test]
fn remove_can_remove_multiple_rights() {
    let mut rights = CastlingRights::all();

    rights.remove(CastlingRights::WHITE_KINGSIDE | CastlingRights::BLACK_KINGSIDE);

    assert_eq!(
        rights,
        CastlingRights::WHITE_QUEENSIDE | CastlingRights::BLACK_QUEENSIDE
    );
}

#[test]
fn has_returns_true_for_present_right() {
    let rights = CastlingRights::WHITE_KINGSIDE;

    assert!(rights.has(CastlingRights::WHITE_KINGSIDE));
}

#[test]
fn has_returns_false_for_absent_right() {
    let rights = CastlingRights::WHITE_KINGSIDE;

    assert!(!rights.has(CastlingRights::WHITE_QUEENSIDE));
    assert!(!rights.has(CastlingRights::BLACK_KINGSIDE));
    assert!(!rights.has(CastlingRights::BLACK_QUEENSIDE));
}

#[test]
fn has_requires_all_requested_rights() {
    let rights = CastlingRights::WHITE_KINGSIDE;

    assert!(!rights.has(CastlingRights::WHITE_KINGSIDE | CastlingRights::WHITE_QUEENSIDE));
}

#[test]
fn has_none_is_true() {
    assert!(CastlingRights::NONE.has(CastlingRights::NONE));
    assert!(CastlingRights::all().has(CastlingRights::NONE));
}

#[test]
fn add_none_does_nothing() {
    let mut rights = CastlingRights::WHITE_KINGSIDE;

    rights.add(CastlingRights::NONE);

    assert_eq!(rights, CastlingRights::WHITE_KINGSIDE);
}

#[test]
fn remove_none_does_nothing() {
    let mut rights = CastlingRights::all();

    rights.remove(CastlingRights::NONE);

    assert_eq!(rights, CastlingRights::all());
}
