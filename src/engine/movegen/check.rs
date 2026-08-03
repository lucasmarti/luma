use crate::engine::{
    chess_move::ChessMove,
    movegen::{
        config::{
            DIAGONAL_DIRECTIONS, HORIZONTAL_VERTICAL_DIRECTIONS, KING_DIRECTIONS, KNIGHT_DIRECTIONS,
        },
        directions, Square,
    },
    piece::{Color, Typ},
    position::Position,
};
/// Check if the king of the given color is in check
pub fn is_check(position: &Position, color: Color) -> bool {
    let king_square = position.get_king_square(color);
    is_under_attack(position, king_square, color)
}

pub fn filter_checks(chess_moves: Vec<ChessMove>, color: Color) -> Vec<ChessMove> {
    chess_moves
        .into_iter()
        .filter(|chess_move| !is_check(&chess_move.position, color))
        .collect()
}

/// Check if a square is under attack by the opponent
pub fn is_under_attack(position: &Position, square: Square, color: Color) -> bool {
    let opponent = color.get_opponent_color();

    // Check horizontal/vertical attacks (rook/queen)
    for direction in HORIZONTAL_VERTICAL_DIRECTIONS {
        let mut current_square = square;

        // Move along the direction until we hit a piece or board edge
        while let Some(next_square) = direction(current_square) {
            current_square = next_square;

            if position.is_occupied(current_square) {
                // Check if it's an opponent rook or queen
                if position.is_occupied_by(current_square, opponent, Typ::Rook)
                    || position.is_occupied_by(current_square, opponent, Typ::Queen)
                {
                    return true;
                }
                // Blocked by another piece
                break;
            }
        }
    }

    // Check diagonal attacks (bishop/queen)
    for direction in DIAGONAL_DIRECTIONS {
        let mut current_square = square;

        // Move along the direction until we hit a piece or board edge
        while let Some(next_square) = direction(current_square) {
            current_square = next_square;

            if position.is_occupied(current_square) {
                // Check if it's an opponent bishop or queen
                if position.is_occupied_by(current_square, opponent, Typ::Bishop)
                    || position.is_occupied_by(current_square, opponent, Typ::Queen)
                {
                    return true;
                }
                // Blocked by another piece
                break;
            }
        }
    }

    // Check knight attacks
    for knight_direction in KNIGHT_DIRECTIONS {
        if let Some(square) = knight_direction(square) {
            if position.is_occupied_by(square, opponent, Typ::Knight) {
                return true;
            }
        }
    }

    // Check king attacks
    for king_direction in KING_DIRECTIONS {
        if let Some(square) = king_direction(square) {
            if position.is_occupied_by(square, opponent, Typ::King) {
                return true;
            }
        }
    }

    // Check pawn attacks
    let pawn_checks = if opponent == Color::White {
        [
            directions::down_left(square),
            directions::down_right(square),
        ]
    } else {
        [directions::up_left(square), directions::up_right(square)]
    };

    for pawn_square in pawn_checks.into_iter().flatten() {
        if position.is_occupied_by(pawn_square, opponent, Typ::Pawn) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests;
