use crate::engine::movegen::Square;

use crate::engine::chess_move::MoveType;
use crate::engine::position::Mve;
use crate::engine::{
    movegen::castling_config::{
        CastlingConfiguration, BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE,
    },
    movegen::check::{is_check, is_under_attack},
    piece::Color,
    position::Position,
};
pub fn generate_castling_moves(position: &Position, color: Color, moves: &mut Vec<Mve>) {
    let castling_configurations: &[CastlingConfiguration] = match color {
        Color::Black => &[BLACK_KINGSIDE, BLACK_QUEENSIDE],
        Color::White => &[WHITE_KINGSIDE, WHITE_QUEENSIDE],
    };
    for castling_config in castling_configurations {
        if let Some(chess_move) = generate_castling_move(position, castling_config) {
            moves.push(chess_move);
        }
    }
}

fn generate_castling_move(position: &Position, castling: &CastlingConfiguration) -> Option<Mve> {
    if !position.has_castling_rights(castling.castling_rights) {
        return None;
    }

    if !is_empty_path(position, castling.empty_path_squares) {
        return None;
    }
    if !(position.is_occupied_by_piece(castling.rook_from, castling.rook)
        && position.is_occupied_by_piece(castling.king_from, castling.king))
    {
        return None;
    }

    if !is_safe_king_path(position, castling.king_path_squares, castling.color) {
        return None;
    }

    if is_check(position, castling.color) {
        return None;
    }
    let mve = Mve {
        piece: castling.king,
        from: castling.king_from,
        to: castling.king_to,
        move_type: MoveType::Castling(castling.castling_type),
    };
    let mut pos = *position;
    pos.make_move(mve);
    if !is_check(&pos, castling.color) {
        return Some(mve);
    }
    None
}

fn is_safe_king_path(position: &Position, squares: &[Square], color: Color) -> bool {
    squares
        .iter()
        .all(|&square| !is_under_attack(position, square, color))
}

fn is_empty_path(position: &Position, sqares: &[Square]) -> bool {
    for square in sqares {
        if position.is_occupied(*square) {
            return false;
        }
    }
    true
}
