use crate::{
    bitboard::Bitboard,
    chess_moves::{ChessMove, EnPassant, Progress, Promotion},
    directions::{self, DirectionFn},
    piece::{
        Piece, BLACK_BISHOP, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
        WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    position::Position,
};

fn get_possible_white_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    let move_functions: Vec<MoveFn> = vec![
        get_move_white_forward,
        get_move_white_two_forward,
        get_move_white_left_capture,
        get_move_white_right_capture,
        get_move_white_left_en_passant,
        get_move_white_right_en_passant,
    ];
    for move_function in move_functions {
        if let Some(chess_move) = move_function(position, from) {
            moves.push(chess_move);
        }
    }
    moves.extend(get_moves_white_promotion(position, from));
    moves.extend(get_moves_white_promotion_left_capture(position, from));
    moves.extend(get_moves_white_promotion_right_capture(position, from));
    moves
}

fn get_possible_black_moves(position: &Position, from: u32) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    let move_functions: Vec<MoveFn> = vec![
        get_move_black_forward,
        get_move_black_two_forward,
        get_move_black_left_capture,
        get_move_black_right_capture,
        get_move_black_left_en_passant,
        get_move_black_right_en_passant,
    ];
    for move_function in move_functions {
        if let Some(chess_move) = move_function(position, from) {
            moves.push(chess_move);
        }
    }
    moves.extend(get_moves_black_promotion(position, from));
    moves.extend(get_moves_black_promotion_left_capture(position, from));
    moves.extend(get_moves_black_promotion_right_capture(position, from));
    moves
}
type MoveFn = fn(&Position, u32) -> Option<ChessMove>;

const WHITE_PROMOTION_PIECES: [Piece; 4] = [WHITE_QUEEN, WHITE_ROOK, WHITE_BISHOP, WHITE_KNIGHT];
const BLACK_PROMOTION_PIECES: [Piece; 4] = [BLACK_QUEEN, BLACK_ROOK, BLACK_BISHOP, BLACK_KNIGHT];

fn get_move_capture(
    position: &Position,
    from: u32,
    direction: DirectionFn,
    piece: Piece,
    opponent: Bitboard,
) -> Option<ChessMove> {
    if let Some(to) = direction(from) {
        if opponent.contains(to) {
            return Some(ChessMove::Progress(Progress { piece, from, to }));
        }
    }
    None
}
fn get_move_white_left_capture(position: &Position, from: u32) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_7(from) {
        return None;
    }
    return get_move_capture(
        position,
        from,
        directions::up_left,
        WHITE_PAWN,
        position.get_black(),
    );
}

fn get_move_white_right_capture(position: &Position, from: u32) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_7(from) {
        return None;
    }
    return get_move_capture(
        position,
        from,
        directions::up_right,
        WHITE_PAWN,
        position.get_black(),
    );
}

fn get_move_black_left_capture(position: &Position, from: u32) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_2(from) {
        return None;
    }
    return get_move_capture(
        position,
        from,
        directions::down_left,
        BLACK_PAWN,
        position.get_white(),
    );
}

fn get_move_black_right_capture(position: &Position, from: u32) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_2(from) {
        return None;
    }
    return get_move_capture(
        position,
        from,
        directions::down_right,
        BLACK_PAWN,
        position.get_white(),
    );
}

fn get_move_white_right_en_passant(position: &Position, from: u32) -> Option<ChessMove> {
    if directions::right(from) == position.en_passant {
        if let (Some(to), Some(capture)) = (directions::up_right(from), directions::right(from)) {
            return Some(ChessMove::EnPassant(EnPassant {
                piece: WHITE_PAWN,
                from,
                to,
                capture,
            }));
        }
    }
    None
}

fn get_move_white_left_en_passant(position: &Position, from: u32) -> Option<ChessMove> {
    if directions::left(from) == position.en_passant {
        if let (Some(to), Some(capture)) = (directions::up_left(from), directions::left(from)) {
            return Some(ChessMove::EnPassant(EnPassant {
                piece: WHITE_PAWN,
                from,
                to,
                capture,
            }));
        }
    }
    None
}

fn get_move_black_right_en_passant(position: &Position, from: u32) -> Option<ChessMove> {
    if directions::right(from) == position.en_passant {
        if let (Some(to), Some(capture)) = (directions::down_right(from), directions::right(from)) {
            return Some(ChessMove::EnPassant(EnPassant {
                piece: BLACK_PAWN,
                from,
                to,
                capture,
            }));
        }
    }
    None
}
fn get_move_black_left_en_passant(position: &Position, from: u32) -> Option<ChessMove> {
    if directions::left(from) == position.en_passant {
        if let (Some(to), Some(capture)) = (directions::down_left(from), directions::left(from)) {
            return Some(ChessMove::EnPassant(EnPassant {
                piece: BLACK_PAWN,
                from,
                to,
                capture,
            }));
        }
    }
    None
}

fn get_moves_white_promotion(position: &Position, from: u32) -> Vec<ChessMove> {
    if directions::is_in_row_7(from) {
        return get_promotion(
            position,
            from,
            directions::up,
            WHITE_PAWN,
            WHITE_PROMOTION_PIECES.to_vec(),
        );
    }
    Vec::new()
}

fn get_moves_black_promotion(position: &Position, from: u32) -> Vec<ChessMove> {
    if directions::is_in_row_2(from) {
        return get_promotion(
            position,
            from,
            directions::down,
            BLACK_PAWN,
            BLACK_PROMOTION_PIECES.to_vec(),
        );
    }
    Vec::new()
}

fn get_moves_white_promotion_left_capture(position: &Position, from: u32) -> Vec<ChessMove> {
    if directions::is_in_row_7(from) {
        return get_promotion_capture(
            position,
            from,
            directions::up_left,
            WHITE_PAWN,
            position.get_black(),
            WHITE_PROMOTION_PIECES.to_vec(),
        );
    }
    Vec::new()
}

fn get_moves_white_promotion_right_capture(position: &Position, from: u32) -> Vec<ChessMove> {
    if directions::is_in_row_7(from) {
        return get_promotion_capture(
            position,
            from,
            directions::up_right,
            WHITE_PAWN,
            position.get_black(),
            WHITE_PROMOTION_PIECES.to_vec(),
        );
    }
    Vec::new()
}

fn get_moves_black_promotion_left_capture(position: &Position, from: u32) -> Vec<ChessMove> {
    if directions::is_in_row_2(from) {
        return get_promotion_capture(
            position,
            from,
            directions::down_left,
            BLACK_PAWN,
            position.get_white(),
            BLACK_PROMOTION_PIECES.to_vec(),
        );
    }
    Vec::new()
}

fn get_moves_black_promotion_right_capture(position: &Position, from: u32) -> Vec<ChessMove> {
    if directions::is_in_row_2(from) {
        return get_promotion_capture(
            position,
            from,
            directions::down_right,
            BLACK_PAWN,
            position.get_white(),
            BLACK_PROMOTION_PIECES.to_vec(),
        );
    }
    Vec::new()
}
fn get_promotion(
    position: &Position,
    from: u32,
    direction: DirectionFn,
    piece: Piece,
    promotion_set: Vec<Piece>,
) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    if let Some(to) = direction(from) {
        if !position.get_all().contains(to) {
            for promotion_piece in promotion_set {
                moves.push(ChessMove::Promotion(Promotion {
                    piece,
                    from,
                    to,
                    new_piece: promotion_piece,
                }));
            }
        }
    }
    moves
}

fn get_promotion_capture(
    position: &Position,
    from: u32,
    direction: DirectionFn,
    piece: Piece,
    opponent: Bitboard,
    promotion_set: Vec<Piece>,
) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    if let Some(to) = direction(from) {
        if opponent.contains(to) {
            for promotion_piece in promotion_set {
                moves.push(ChessMove::Promotion(Promotion {
                    piece,
                    from,
                    to,
                    new_piece: promotion_piece,
                }));
            }
        }
    }
    moves
}

fn get_move_white_forward(position: &Position, from: u32) -> Option<ChessMove> {
    if !directions::is_in_last_or_second_last_row(from) {
        return get_move_forward(position, from, WHITE_PAWN, directions::up);
    }
    None
}

fn get_move_black_forward(position: &Position, from: u32) -> Option<ChessMove> {
    if !directions::is_in_first_or_second_row(from) {
        return get_move_forward(position, from, BLACK_PAWN, directions::down);
    }
    None
}

fn get_move_forward(
    position: &Position,
    from: u32,
    piece: Piece,
    direction: DirectionFn,
) -> Option<ChessMove> {
    if let Some(to) = direction(from) {
        if !position.get_all().contains(to) {
            return Some(ChessMove::Progress(Progress {
                piece: piece,
                from: from,
                to: to,
            }));
        }
    }
    None
}

fn get_move_white_two_forward(position: &Position, from: u32) -> Option<ChessMove> {
    if directions::is_in_row_2(from) {
        return get_move_two_forward(position, from, WHITE_PAWN, directions::up);
    }
    None
}

fn get_move_black_two_forward(position: &Position, from: u32) -> Option<ChessMove> {
    if directions::is_in_row_7(from) {
        return get_move_two_forward(position, from, BLACK_PAWN, directions::down);
    }
    None
}

fn get_move_two_forward(
    position: &Position,
    from: u32,
    piece: Piece,
    direction: DirectionFn,
) -> Option<ChessMove> {
    if let Some(one_forward) = direction(from) {
        if !position.get_all().contains(one_forward) {
            if let Some(two_forward) = direction(one_forward) {
                if !position.get_all().contains(two_forward) {
                    return Some(ChessMove::Progress(Progress {
                        piece,
                        from,
                        to: two_forward,
                    }));
                }
            }
        }
    }
    None
}

mod tests;
