use rand::seq::IndexedRandom;

use crate::engine::{
    check::is_check,
    chess_moves::{
        configurations::{CastlingMovesFn, MovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG},
        get_current_player_moves,
    },
    directions::squares::{Square, BLACK_KING_STARTING_POSITION, WHITE_KING_STARTING_POSITION},
    minimax::chess_impl::get_best_move,
    piece::{
        Color,
        Piece::{self},
    },
    position::Position,
};

pub fn get_next_move(position: &Position) -> MoveOrEnd {
    match get_best_move(*position) {
        Some(position) => return MoveOrEnd::Move(position),
        None => {
            if is_check(position, position.get_player()) {
                return MoveOrEnd::GameEnd(GameEnd::Victory(
                    position.get_player().get_opponent_color(),
                ));
            } else {
                return MoveOrEnd::GameEnd(GameEnd::Draw);
            }
        }
    }
}

pub fn get_possible_moves(position: &Position) -> Result<Vec<Position>, GameEnd> {
    let positions: Vec<Position> = get_current_player_moves(position);

    if positions.is_empty() {
        if is_check(position, position.get_player()) {
            Err(GameEnd::Victory(position.get_player().get_opponent_color()))
        } else {
            Err(GameEnd::Draw)
        }
    } else {
        Ok(positions)
    }
}

pub fn get_check_square(position: &Position) -> Option<Square> {
    if is_check(position, position.get_player()) {
        let king = match position.get_player() {
            piece::Color::Black => Piece::BlackKing,
            piece::Color::White => Piece::WhiteKing,
        };
        if let Some(square) = position.get_squares(king).iter().next() {
            return Some(square);
        }
    }
    None
}
pub enum MoveOrEnd {
    Move(Position),
    GameEnd(GameEnd),
}
pub enum GameEnd {
    Draw,
    Victory(Color),
}

fn get_castling_moves_fn(square: Square, piece: Piece) -> Option<CastlingMovesFn> {
    if square == WHITE_KING_STARTING_POSITION && piece == Piece::WhiteKing {
        Some(WHITE_MOVE_CONFIG.castling_move_fn)
    } else if square == BLACK_KING_STARTING_POSITION && piece == Piece::BlackKing {
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

/*
pub fn get_valid_drop_positions(position: &Position, from: Square) -> Vec<Position> {
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

    pub fn execute_move(position: &Position, chess_move: ChessMove) -> Option<Position> {
    get_valid_drop_positions(position, chess_move.from)
        .iter()
        .find(|pos| pos.get_last_move() == Some(chess_move))
        .cloned()
}

#[allow(dead_code)]
fn get_random_move(position: &Position) -> Option<Position> {
    let positions = get_current_player_moves(position);
    positions.choose(&mut rand::rng()).copied()
}

pub fn is_valid_drag_square(position: &Position, square: Square) -> bool {
    match position.get_piece_at(square) {
        Some(piece) => piece.get_color() == position.get_player(),
        None => false,
    }
}
*/

mod check;
pub mod chess_moves;
pub mod directions;
mod heuristic;
mod minimax;
pub mod piece;
pub mod position;
#[cfg(test)]
mod tests;
