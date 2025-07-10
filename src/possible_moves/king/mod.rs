use crate::{
    check::{is_in_check, is_square_attacked},
    chess_moves::ChessMove,
    directions::{self, DirectionFn, C1, C8, D1, D8, E1, E8, F1, F8, G1, G8},
    piece::{Color, Piece, BLACK_KING, WHITE_KING},
    position::Position,
    possible_moves::common::get_single_step_moves,
};

const KING_DIRECTIONS: [DirectionFn; 8] = [
    directions::left,
    directions::right,
    directions::up,
    directions::down,
    directions::up_left,
    directions::up_right,
    directions::down_left,
    directions::down_right,
];

pub fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::Black)
}
pub fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    get_possible_moves(position, from, Color::White)
}

fn get_possible_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let king = get_king(color);
    let mut moves = get_single_step_moves(position, from, color, &KING_DIRECTIONS, king);
    moves.extend(get_castle_moves(position, from, color));
    moves
}

fn get_king(color: Color) -> Piece {
    match color {
        Color::Black => BLACK_KING,
        Color::White => WHITE_KING,
    }
}

fn get_castle_moves(position: &Position, from: u32, color: Color) -> Vec<ChessMove> {
    let mut moves = Vec::new();

    // Only check castling if king is on starting square
    let king_start_square = match color {
        Color::White => E1,
        Color::Black => E8,
    };

    if from != king_start_square {
        return moves;
    }

    // Check kingside castling
    if can_castle_kingside(position, color) {
        match color {
            Color::White => moves.push(ChessMove::WhiteKingsideCastle),
            Color::Black => moves.push(ChessMove::BlackKingsideCastle),
        }
    }

    // Check queenside castling
    if can_castle_queenside(position, color) {
        match color {
            Color::White => moves.push(ChessMove::WhiteQueensideCastle),
            Color::Black => moves.push(ChessMove::BlackQueensideCastle),
        }
    }

    moves
}

fn can_castle_kingside(position: &Position, color: Color) -> bool {
    match color {
        Color::White => {
            // Check castle flag
            if !position.white_kingside_castle {
                return false;
            }

            // Check if squares between king and rook are empty
            let all_pieces = position.get_all();
            if all_pieces.contains(F1) || all_pieces.contains(G1) {
                return false;
            }

            // Check if king is in check
            if is_in_check(position, Color::White) {
                return false;
            }

            // Check if king passes through or ends in check
            let opponent_color = Color::Black;
            if is_square_attacked(position, F1, opponent_color)
                || is_square_attacked(position, G1, opponent_color)
            {
                return false;
            }

            true
        }
        Color::Black => {
            // Check castle flag
            if !position.black_kingside_castle {
                return false;
            }

            // Check if squares between king and rook are empty
            let all_pieces = position.get_all();
            if all_pieces.contains(F8) || all_pieces.contains(G8) {
                return false;
            }

            // Check if king is in check
            if is_in_check(position, Color::Black) {
                return false;
            }

            // Check if king passes through or ends in check
            let opponent_color = Color::White;
            if is_square_attacked(position, F8, opponent_color)
                || is_square_attacked(position, G8, opponent_color)
            {
                return false;
            }

            true
        }
    }
}

fn can_castle_queenside(position: &Position, color: Color) -> bool {
    match color {
        Color::White => {
            // Check castle flag
            if !position.white_queenside_castle {
                return false;
            }

            // Check if squares between king and rook are empty
            let all_pieces = position.get_all();
            if all_pieces.contains(D1) || all_pieces.contains(C1) {
                return false;
            }

            // Check if king is in check
            if is_in_check(position, Color::White) {
                return false;
            }

            // Check if king passes through or ends in check
            let opponent_color = Color::Black;
            if is_square_attacked(position, D1, opponent_color)
                || is_square_attacked(position, C1, opponent_color)
            {
                return false;
            }

            true
        }
        Color::Black => {
            // Check castle flag
            if !position.black_queenside_castle {
                return false;
            }

            // Check if squares between king and rook are empty
            let all_pieces = position.get_all();
            if all_pieces.contains(D8) || all_pieces.contains(C8) {
                return false;
            }

            // Check if king is in check
            if is_in_check(position, Color::Black) {
                return false;
            }

            // Check if king passes through or ends in check
            let opponent_color = Color::White;
            if is_square_attacked(position, D8, opponent_color)
                || is_square_attacked(position, C8, opponent_color)
            {
                return false;
            }

            true
        }
    }
}

mod tests;
