use crate::{
    chess_moves::{ChessMove, Progress},
    directions::{self, *},
    piece::*,
    position::{self, Position},
    possible_moves::common::{explore, get_opponent_pieces, get_own_pieces},
};

#[test]
fn test_get_own_pieces() {
    let position: Position = Position::new_starting_position();
    let own_pieces = get_own_pieces(&position, Color::White);
    assert!(own_pieces == position.get_white());
}

#[test]
fn test_get_opponent_pieces() {
    let position: Position = Position::new_starting_position();
    let opponent_pieces = get_opponent_pieces(&position, Color::White);
    assert!(opponent_pieces == position.get_black());
}

#[test]
fn test_explore1() {
    let mut position: Position = Position::default();
    position = position
        .put_piece(WHITE_QUEEN, G4)
        .put_piece(WHITE_KING, G7);
    let path = directions::all_up(G4);
    let moves = explore(&position, G4, path, WHITE_QUEEN);

    let result: Vec<ChessMove> = vec![
        ChessMove::Progress(Progress {
            from: G4,
            to: G5,
            piece: WHITE_QUEEN,
        }),
        ChessMove::Progress(Progress {
            from: G4,
            to: G6,
            piece: WHITE_QUEEN,
        }),
    ];
    assert!(moves == result);
}

#[test]
fn test_explore2() {
    let mut position: Position = Position::default();
    position = position
        .put_piece(WHITE_QUEEN, G4)
        .put_piece(BLACK_KING, G7);
    let path = directions::all_up(G4);
    let moves = explore(&position, G4, path, WHITE_QUEEN);

    let result: Vec<ChessMove> = vec![
        ChessMove::Progress(Progress {
            from: G4,
            to: G5,
            piece: WHITE_QUEEN,
        }),
        ChessMove::Progress(Progress {
            from: G4,
            to: G6,
            piece: WHITE_QUEEN,
        }),
        ChessMove::Progress(Progress {
            from: G4,
            to: G7,
            piece: WHITE_QUEEN,
        }),
    ];
    assert!(moves == result);
}

#[test]
fn test_explore3() {
    let mut position: Position = Position::default();
    position = position.put_piece(WHITE_QUEEN, G4);
    let path = directions::all_up(G4);
    let moves = explore(&position, G4, path, WHITE_QUEEN);

    let result: Vec<ChessMove> = vec![
        ChessMove::Progress(Progress {
            from: G4,
            to: G5,
            piece: WHITE_QUEEN,
        }),
        ChessMove::Progress(Progress {
            from: G4,
            to: G6,
            piece: WHITE_QUEEN,
        }),
        ChessMove::Progress(Progress {
            from: G4,
            to: G7,
            piece: WHITE_QUEEN,
        }),
        ChessMove::Progress(Progress {
            from: G4,
            to: G8,
            piece: WHITE_QUEEN,
        }),
    ];
    assert!(moves == result);
}
