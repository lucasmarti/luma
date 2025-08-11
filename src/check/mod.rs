use crate::{
    directions::{self, DirectionFn},
    piece::{Color, Piece, Typ, *},
    position::Position,
    possible_moves::configurations::*,
};
/// Check if the king of the given color is in check
pub fn is_check(position: &Position, color: Color) -> bool {
    let king_square = position.get_king_square(color);
    is_under_attack(position, king_square, color)
}

/// Check if a square is under attack by the opponent
pub fn is_under_attack(position: &Position, square: u32, color: Color) -> bool {
    let opponent = color.get_opponent_color();

    // Check horizontal/vertical attacks (rook/queen)
    for direction in HORIZONTAL_VERTICAL_DIRECTIONS {
        let mut current_square = square;

        // Move along the direction until we hit a piece or board edge
        while let Some(next_square) = direction(current_square) {
            current_square = next_square;

            if position.is_occupied(current_square) {
                // Check if it's an opponent rook or queen
                if position.is_occupied_by_piece(
                    current_square,
                    Piece {
                        typ: Typ::Rook,
                        color: opponent,
                    },
                ) || position.is_occupied_by_piece(
                    current_square,
                    Piece {
                        typ: Typ::Queen,
                        color: opponent,
                    },
                ) {
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
                if position.is_occupied_by_piece(
                    current_square,
                    Piece {
                        typ: Typ::Bishop,
                        color: opponent,
                    },
                ) || position.is_occupied_by_piece(
                    current_square,
                    Piece {
                        typ: Typ::Queen,
                        color: opponent,
                    },
                ) {
                    return true;
                }
                // Blocked by another piece
                break;
            }
        }
    }

    // Check knight attacks
    for knight_direction in KNIGHT_DIRECTIONS {
        if let Some(knight_square) = knight_direction(square) {
            if position.is_occupied_by_piece(
                knight_square,
                Piece {
                    typ: Typ::Knight,
                    color: opponent,
                },
            ) {
                return true;
            }
        }
    }

    // Check king attacks
    for king_direction in KING_DIRECTIONS {
        if let Some(king_square) = king_direction(square) {
            if position.is_occupied_by_piece(
                king_square,
                Piece {
                    typ: Typ::King,
                    color: opponent,
                },
            ) {
                return true;
            }
        }
    }

    // Check pawn attacks
    if opponent == Color::White {
        // Check for white pawns that could attack this square
        if let Some(pawn_square) = directions::down_left(square) {
            if position.is_occupied_by_piece(pawn_square, WHITE_PAWN) {
                return true;
            }
        }
        if let Some(pawn_square) = directions::down_right(square) {
            if position.is_occupied_by_piece(pawn_square, WHITE_PAWN) {
                return true;
            }
        }
    } else {
        // Check for black pawns that could attack this square
        if let Some(pawn_square) = directions::up_left(square) {
            if position.is_occupied_by_piece(pawn_square, BLACK_PAWN) {
                return true;
            }
        }
        if let Some(pawn_square) = directions::up_right(square) {
            if position.is_occupied_by_piece(pawn_square, BLACK_PAWN) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests;
