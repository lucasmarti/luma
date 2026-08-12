use crate::engine::movegen::Square;

use crate::engine::chess_move::{ChessMove, MoveType};
use crate::engine::position::CastlingRights;
use crate::engine::{
    movegen::castling_config::{
        CastlingConfiguration, BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE,
    },
    movegen::check::{is_check, is_under_attack},
    piece::Color,
    position::Position,
};

pub fn get_black_castling_moves(position: &Position) -> Vec<ChessMove> {
    get_castling_moves(position, [BLACK_KINGSIDE, BLACK_QUEENSIDE])
}

pub fn get_white_castling_moves(position: &Position) -> Vec<ChessMove> {
    get_castling_moves(position, [WHITE_KINGSIDE, WHITE_QUEENSIDE])
}

fn get_castling_moves(
    position: &Position,
    castling_configurations: [CastlingConfiguration; 2],
) -> Vec<ChessMove> {
    let mut chess_moves: Vec<ChessMove> = Vec::new();
    for castling_config in castling_configurations {
        if let Some(chess_move) = get_castling_move(position, castling_config) {
            chess_moves.push(chess_move);
        }
    }
    chess_moves
}

pub fn get_castling_move(
    position: &Position,
    castling: CastlingConfiguration,
) -> Option<ChessMove> {
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

    if !is_save_passage(position, castling.empty_path_squares, castling.color) {
        return None;
    }

    if is_check(position, castling.color) {
        return None;
    }

    let new_position = position
        .remove_piece(castling.king_from)
        .remove_piece(castling.rook_from)
        .put_piece(castling.king, castling.king_to)
        .put_piece(castling.rook, castling.rook_to)
        .toggle_player()
        .reset_en_passant()
        .disallow_castling_for_color(castling.color);

    if !is_check(&new_position, castling.color) {
        let chess_move: ChessMove = ChessMove {
            move_type: MoveType::Castling,
            piece: castling.king,
            from: castling.king_from,
            to: castling.king_to,
            capture: None,
            pormotion: None,
            position: new_position,
        };
        return Some(chess_move);
    }
    None
}

fn is_save_passage(position: &Position, sqares: &[Square], color: Color) -> bool {
    for square in sqares {
        if is_under_attack(position, *square, color) {
            return false;
        }
    }
    true
}

fn is_empty_path(position: &Position, sqares: &[Square]) -> bool {
    for square in sqares {
        if position.is_occupied(*square) {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests;
