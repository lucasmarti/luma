use crate::engine::{
    chess_moves::{
        configurations::{MovesFn, BLACK_MOVE_CONFIG, WHITE_MOVE_CONFIG},
        get_black_moves, get_white_moves,
    },
    position::Position,
};

pub fn get_valid_drop_targets(position: &Position, from: u32) -> Vec<u32> {
    let mut targets: Vec<u32> = Vec::new();
    if let Some(piece) = position.get_piece_at(from) {
        let config = match piece.color {
            piece::Color::Black => BLACK_MOVE_CONFIG,
            piece::Color::White => WHITE_MOVE_CONFIG,
        };
        let moves_fn: MovesFn = match piece.typ {
            piece::Typ::King => config.king_fn,
            piece::Typ::Queen => config.queen_fn,
            piece::Typ::Rook => config.rook_fn,
            piece::Typ::Pawn => config.pawn_fn,
            piece::Typ::Knight => config.knight_fn,
            piece::Typ::Bishop => config.bishop_fn,
        };
        let positions = moves_fn(position, piece, from);
        for position in positions {
            if let Some(square) = position.get_target() {
                targets.push(square);
            }
        }
    }

    targets
}
pub fn get_next_positions(position: &Position) -> Vec<Position> {
    let positions: Vec<Position> = Vec::new();
    positions
}

pub fn new_game() -> Position {
    Position::new_starting_position()
}

mod check;
mod chess_moves;
mod directions;
mod evaluation;
mod minimax;
mod piece;
pub mod position;
#[cfg(test)]
mod tests;
