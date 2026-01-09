use crate::engine::{
    check::{is_check, is_under_attack},
    chess_moves::{
        castling::configurations::{
            CastlingConfiguration, BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE,
        },
        ChessMove, MoveType,
    },
    directions::squares::Square,
    piece::Color,
    position::{CastlingType, Position},
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
    if !position.get_castling_right(castling.castling_type) {
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
            move_type: MoveType::Castling {
                castling_type: castling.castling_type,
            },
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

pub fn remove_castling_rights_if_necessary(position: Position, from: Square) -> Position {
    let mut new_position = position;
    if from == WHITE_QUEENSIDE.king_from || from == WHITE_QUEENSIDE.rook_from {
        new_position = new_position.remove_castling_right(CastlingType::WhiteQueenside);
    }
    if from == WHITE_KINGSIDE.king_from || from == WHITE_KINGSIDE.rook_from {
        new_position = new_position.remove_castling_right(CastlingType::WhiteKingside);
    }
    if from == BLACK_QUEENSIDE.king_from || from == BLACK_QUEENSIDE.rook_from {
        new_position = new_position.remove_castling_right(CastlingType::BlackQueenside);
    }
    if from == BLACK_KINGSIDE.king_from || from == BLACK_KINGSIDE.rook_from {
        new_position = new_position.remove_castling_right(CastlingType::BlackKingside);
    }
    new_position
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

mod configurations;
#[cfg(test)]
mod tests;
