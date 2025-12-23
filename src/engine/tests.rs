#[cfg(test)]
use crate::engine::chess_moves::configurations::MovesFn;
use crate::engine::{
    check::filter_checks,
    chess_moves::configurations::{CastlingMovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG},
    piece,
};
use crate::engine::{
    directions::squares::*,
    piece::Piece::{self},
    position::Position,
};

fn get_valid_drop_positions(position: &Position, from: Square) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    if let Some(piece) = position.get_piece_at(from) {
        let moves_fn = get_moves_fn(piece);
        positions.extend(moves_fn(position, piece, from));
        if let Some(castling_moves_fn) = get_castling_moves_fn(from, piece) {
            positions.extend(castling_moves_fn(position));
        }
        positions = filter_checks(positions, position.get_player());
    }
    positions
}
pub const WHITE_KING_STARTING_POSITION: Square = E1;
pub const BLACK_KING_STARTING_POSITION: Square = E8;

fn get_castling_moves_fn(square: Square, piece: Piece) -> Option<CastlingMovesFn> {
    if square == WHITE_KING_STARTING_POSITION && piece == Piece::WhiteKing {
        Some(WHITE_MOVE_CONFIG.castling_move_fn)
    } else if square == BLACK_KING_STARTING_POSITION && piece == Piece::BlackKing {
        use crate::engine::chess_moves::configurations::BLACK_MOVE_CONFIG;

        Some(BLACK_MOVE_CONFIG.castling_move_fn)
    } else {
        None
    }
}
fn get_moves_fn(piece: Piece) -> MovesFn {
    let config = match piece.get_color() {
        piece::Color::Black => BLACK_MOVE_CONFIG,
        piece::Color::White => WHITE_MOVE_CONFIG,
    };
    let moves_fn: MovesFn = match piece.get_type() {
        piece::Typ::King => config.king_fn,
        piece::Typ::Queen => config.queen_fn,
        piece::Typ::Rook => config.rook_fn,
        piece::Typ::Pawn => config.pawn_fn,
        piece::Typ::Knight => config.knight_fn,
        piece::Typ::Bishop => config.bishop_fn,
    };
    moves_fn
}
#[test]
fn test_valid_drop_targets_pawn() {
    let position = Position::new_starting_position();
    let targets = get_valid_drop_positions(&position, D2);
    assert!(targets.iter().any(|p| p.is_occupied(D3)));
    assert!(targets.iter().any(|p| p.is_occupied(D4)));
}
#[test]
fn test_valid_drop_targets_knight() {
    let position = Position::new_starting_position();
    let targets = get_valid_drop_positions(&position, B8);
    assert!(targets.iter().any(|p| p.is_occupied(A6)));
    assert!(targets.iter().any(|p| p.is_occupied(C6)));
}

#[test]
fn test_valid_drop_targets_castling() {
    let position = Position::default()
        .put_piece(Piece::WhiteRook, H1)
        .put_piece(Piece::WhiteKing, E1);
    let targets = get_valid_drop_positions(&position, E1);
    assert!(targets.iter().any(|p| p.is_occupied(G1)));
}

#[test]
fn test_valid_drop_targets_en_passant() {
    let position = Position::default()
        .put_piece(Piece::WhiteKing, E1)
        .put_piece(Piece::WhitePawn, E4)
        .put_piece(Piece::BlackPawn, D4)
        .set_en_passant(E4);
    let targets = get_valid_drop_positions(&position, D4);
    println!("{:?}", targets);
    assert!(targets.iter().any(|p| p.is_occupied(E3)));
}
