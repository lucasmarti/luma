use crate::engine::{
    check::{is_check, is_under_attack},
    chess_moves::{
        castling::configurations::{
            CastlingConfiguration, BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE,
        },
        ChessMove, MoveType,
    },
    piece::Color,
    position::{CastlingType, Position},
};

pub fn get_black_castle_moves(position: &Position) -> Vec<Position> {
    get_castling_moves(position, [BLACK_KINGSIDE, BLACK_QUEENSIDE])
}

pub fn get_white_castle_moves(position: &Position) -> Vec<Position> {
    get_castling_moves(position, [WHITE_KINGSIDE, WHITE_QUEENSIDE])
}

fn get_castling_moves(
    position: &Position,
    castling_configurations: [CastlingConfiguration; 2],
) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    for castling_config in castling_configurations {
        if let Some(position) = get_castling_move(position, castling_config) {
            positions.push(position);
        }
    }
    positions
}

pub fn get_castling_move(position: &Position, castle: CastlingConfiguration) -> Option<Position> {
    if !position.get_castling_right(castle.castling_type) {
        return None;
    }

    if !is_empty_path(position, castle.empty_path_squares) {
        return None;
    }
    if !(position.is_occupied_by_piece(castle.rook_from, castle.rook)
        && position.is_occupied_by_piece(castle.king_from, castle.king))
    {
        return None;
    }

    if !is_save_passage(position, castle.empty_path_squares, castle.color) {
        return None;
    }

    if is_check(position, castle.color) {
        return None;
    }

    let chess_move: ChessMove = ChessMove {
        move_type: MoveType::Castling {
            castling_type: castle.castling_type,
        },
        piece: castle.king,
        from: castle.king_from,
        to: castle.king_to,
        capture: None,
        pormotion: None,
    };

    let new_position = position
        .remove_piece(castle.king_from)
        .remove_piece(castle.rook_from)
        .put_piece(castle.king, castle.king_to)
        .put_piece(castle.rook, castle.rook_to)
        .toggle_player()
        .reset_en_passant()
        .disallow_castle_for_color(castle.color)
        .set_chess_move(chess_move);
    if !is_check(&new_position, castle.color) {
        return Some(new_position);
    }
    None
}

pub fn remove_castling_rights_if_necessary(position: Position, from: u32) -> Position {
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

fn is_save_passage(position: &Position, sqares: &[u32], color: Color) -> bool {
    for square in sqares {
        if is_under_attack(position, *square, color) {
            return false;
        }
    }
    return true;
}

fn is_empty_path(position: &Position, sqares: &[u32]) -> bool {
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
