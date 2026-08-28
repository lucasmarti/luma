mod castling;
mod castling_config;
pub(super) use castling_config::*;
mod check;
mod common;
mod config;
pub(super) mod directions;
mod pawn;
mod squares;
pub(super) use check::filter_checks;
pub use check::is_check;
pub(super) use config::CastlingMovesFn;
pub(super) use config::MovesFn;
pub(super) use config::BLACK_MOVE_CONFIG;
pub(super) use config::WHITE_MOVE_CONFIG;
pub use squares::*;

use crate::engine::movegen::config::Config;
use crate::engine::ChessMove;
use crate::engine::Color;
use crate::engine::Mve;
use crate::engine::Piece;
use crate::engine::Position;

pub fn get_check_square(position: &Position) -> Option<Square> {
    is_check(position, position.get_player())
        .then(|| position.get_king_square(position.get_player()))
}

pub fn get_current_player_moves(position: &Position) -> Vec<ChessMove> {
    get_moves_by_color(position, position.get_player(), MoveMode::Legal)
}

pub fn get_moves_by_color(
    position: &Position,
    color: Color,
    move_mode: MoveMode,
) -> Vec<ChessMove> {
    match color {
        Color::Black => get_moves(position, BLACK_MOVE_CONFIG, move_mode),
        Color::White => get_moves(position, WHITE_MOVE_CONFIG, move_mode),
    }
}

fn get_moves(position: &Position, config: Config, move_mode: MoveMode) -> Vec<ChessMove> {
    let mut chess_moves: Vec<ChessMove> = Vec::new();

    chess_moves.extend(get_new_positions(position, config.bishop, config.bishop_fn));
    chess_moves.extend(get_new_positions(position, config.king, config.king_fn));
    chess_moves.extend(get_new_positions(position, config.queen, config.queen_fn));
    chess_moves.extend(get_new_positions(position, config.rook, config.rook_fn));
    chess_moves.extend(get_new_positions(position, config.knight, config.knight_fn));
    chess_moves.extend(get_new_positions(position, config.pawn, config.pawn_fn));
    chess_moves.extend((config.castling_move_fn)(position));

    match move_mode {
        MoveMode::Legal => filter_checks(chess_moves, position.get_player()),
        MoveMode::PseudoLegal => chess_moves,
    }
}
fn get_new_positions(position: &Position, piece: Piece, get_moves_fn: MovesFn) -> Vec<ChessMove> {
    let mut chess_moves: Vec<ChessMove> = Vec::new();
    for square in position.get_squares(piece.color, piece.typ).iter() {
        chess_moves.extend(get_moves_fn(position, piece, square));
    }
    chess_moves
}
pub enum MoveMode {
    Legal,
    PseudoLegal,
}
#[cfg(test)]
mod tests;
