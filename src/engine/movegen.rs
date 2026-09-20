mod castling;
mod castling_config;
pub(super) use castling_config::*;
mod check;
mod common;
mod config;
pub(super) mod directions;
mod pawn;
mod squares;
use crate::engine::movegen::castling::generate_castling_moves;
use crate::engine::movegen::common::generate_moves_for_typ;
use crate::engine::movegen::config::BISHOP_CONFIG;
use crate::engine::movegen::config::KING_CONFIG;
use crate::engine::movegen::config::KNIGHT_CONFIG;
use crate::engine::movegen::config::QUEEN_CONFIG;
use crate::engine::movegen::config::ROOK_CONFIG;
use crate::engine::movegen::pawn::generate_pawn_moves;
use crate::engine::Color;
use crate::engine::Mve;
use crate::engine::Piece;
use crate::engine::Position;
use crate::engine::Typ;
pub(super) use check::filter_checks;
pub use check::is_check;
pub use squares::*;

pub fn get_check_square(position: &Position) -> Option<Square> {
    is_check(position, position.get_player())
        .then(|| position.get_king_square(position.get_player()))
}

pub fn get_current_player_moves(position: &Position) -> Vec<Mve> {
    generate_moves(position, position.get_player())
}
pub fn generate_moves(position: &Position, color: Color) -> Vec<Mve> {
    let mut moves = generate_pseude_legal_moves(position, color);
    filter_checks(position, &mut moves);
    moves
}

pub fn generate_pseude_legal_moves(position: &Position, color: Color) -> Vec<Mve> {
    let mut moves = Vec::new();

    for (piece, square) in position.iter_color(color) {
        generate_moves_for_piece_at_square(position, piece, square, &mut moves);
    }
    generate_castling_moves(position, color, &mut moves);
    moves
}

fn generate_moves_for_piece_at_square(
    position: &Position,
    piece: Piece,
    square: Square,
    moves: &mut Vec<Mve>,
) {
    match piece.typ {
        Typ::King => generate_moves_for_typ(position, piece, square, &KING_CONFIG, moves),
        Typ::Queen => generate_moves_for_typ(position, piece, square, &QUEEN_CONFIG, moves),
        Typ::Rook => generate_moves_for_typ(position, piece, square, &ROOK_CONFIG, moves),
        Typ::Knight => generate_moves_for_typ(position, piece, square, &KNIGHT_CONFIG, moves),
        Typ::Bishop => generate_moves_for_typ(position, piece, square, &BISHOP_CONFIG, moves),
        Typ::Pawn => generate_pawn_moves(position, piece, square, moves),
    }
}
mod tests;
