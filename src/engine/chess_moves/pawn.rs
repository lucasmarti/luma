use crate::engine::{
    chess_moves::{self, common::progess, ChessMove, MoveType},
    directions::{self, squares::Square, DirectionFn, RowFn},
    piece::{Color, Piece, Typ},
    position::Position,
};

struct EnPassantConfig {
    players_pawn: Piece,
    opponents_pawn: Piece,
    next_fn: DirectionFn,
    diagonal_fn: DirectionFn,
}

const WHITE_LEFT_EN_PASSANT: EnPassantConfig = EnPassantConfig {
    players_pawn: Piece::WhitePawn,
    opponents_pawn: Piece::BlackPawn,
    next_fn: directions::left,
    diagonal_fn: directions::up_left,
};

const WHITE_RIGHT_EN_PASSANT: EnPassantConfig = EnPassantConfig {
    players_pawn: Piece::WhitePawn,
    opponents_pawn: Piece::BlackPawn,
    next_fn: directions::right,
    diagonal_fn: directions::up_right,
};

const BLACK_LEFT_EN_PASSANT: EnPassantConfig = EnPassantConfig {
    players_pawn: Piece::BlackPawn,
    opponents_pawn: Piece::WhitePawn,
    next_fn: directions::left,
    diagonal_fn: directions::down_left,
};

const BLACK_RIGHT_EN_PASSANT: EnPassantConfig = EnPassantConfig {
    players_pawn: Piece::BlackPawn,
    opponents_pawn: Piece::WhitePawn,
    next_fn: directions::right,
    diagonal_fn: directions::down_right,
};

const WHITE_MOVE_FUNCTIONS: [MoveFn; 6] = [
    get_move_white_forward,
    get_move_white_two_forward,
    get_move_white_left_capture,
    get_move_white_right_capture,
    get_move_white_left_en_passant,
    get_move_white_right_en_passant,
];
const BLACK_MOVE_FUNCTIONS: [MoveFn; 6] = [
    get_move_black_forward,
    get_move_black_two_forward,
    get_move_black_left_capture,
    get_move_black_right_capture,
    get_move_black_left_en_passant,
    get_move_black_right_en_passant,
];

type MoveFn = fn(&Position, Square) -> Option<ChessMove>;

const WHITE_PROMOTION_PIECES: [Piece; 4] = [
    Piece::WhiteQueen,
    Piece::WhiteRook,
    Piece::WhiteBishop,
    Piece::WhiteKnight,
];
const BLACK_PROMOTION_PIECES: [Piece; 4] = [
    Piece::BlackQueen,
    Piece::BlackRook,
    Piece::BlackBishop,
    Piece::BlackKnight,
];

struct PromotionConfig {
    piece: Piece,
    is_in_row_fn: RowFn,
    direction_fn: DirectionFn,
    promotion_set: [Piece; 4],
}

const WHITE_PROMOTION_CONFIG: PromotionConfig = PromotionConfig {
    piece: Piece::WhitePawn,
    is_in_row_fn: directions::is_in_row_7,
    direction_fn: directions::up,
    promotion_set: WHITE_PROMOTION_PIECES,
};

const WHITE_PROMOTION_LEFT_CONFIG: PromotionConfig = PromotionConfig {
    piece: Piece::WhitePawn,
    is_in_row_fn: directions::is_in_row_7,
    direction_fn: directions::up_left,
    promotion_set: WHITE_PROMOTION_PIECES,
};

const WHITE_PROMOTION_RIGHT_CONFIG: PromotionConfig = PromotionConfig {
    piece: Piece::WhitePawn,
    is_in_row_fn: directions::is_in_row_7,
    direction_fn: directions::up_right,
    promotion_set: WHITE_PROMOTION_PIECES,
};

const BLACK_PROMOTION_CONFIG: PromotionConfig = PromotionConfig {
    piece: Piece::BlackPawn,
    is_in_row_fn: directions::is_in_row_2,
    direction_fn: directions::down,
    promotion_set: BLACK_PROMOTION_PIECES,
};

const BLACK_PROMOTION_LEFT_CONFIG: PromotionConfig = PromotionConfig {
    piece: Piece::BlackPawn,
    is_in_row_fn: directions::is_in_row_2,
    direction_fn: directions::down_left,
    promotion_set: BLACK_PROMOTION_PIECES,
};

const BLACK_PROMOTION_RIGHT_CONFIG: PromotionConfig = PromotionConfig {
    piece: Piece::BlackPawn,
    is_in_row_fn: directions::is_in_row_2,
    direction_fn: directions::down_right,
    promotion_set: BLACK_PROMOTION_PIECES,
};

pub fn get_pawn_moves(position: &Position, piece: Piece, square: Square) -> Vec<ChessMove> {
    match piece.get_color() {
        Color::Black => get_black_pawn_moves(position, square),
        Color::White => get_white_pawn_moves(position, square),
    }
}

fn get_white_pawn_moves(position: &Position, square: Square) -> Vec<ChessMove> {
    let mut chess_moves: Vec<ChessMove> = Vec::new();
    for move_function in WHITE_MOVE_FUNCTIONS {
        if let Some(chess_move) = move_function(position, square) {
            chess_moves.push(chess_move);
        }
    }
    chess_moves.extend(get_moves_white_promotion(position, square));
    chess_moves.extend(get_moves_white_promotion_left_capture(position, square));
    chess_moves.extend(get_moves_white_promotion_right_capture(position, square));
    chess_moves
}

fn get_black_pawn_moves(position: &Position, square: Square) -> Vec<ChessMove> {
    let mut chess_moves: Vec<ChessMove> = Vec::new();
    for move_function in BLACK_MOVE_FUNCTIONS {
        if let Some(chess_move) = move_function(position, square) {
            chess_moves.push(chess_move);
        }
    }
    chess_moves.extend(get_moves_black_promotion(position, square));
    chess_moves.extend(get_moves_black_promotion_left_capture(position, square));
    chess_moves.extend(get_moves_black_promotion_right_capture(position, square));
    chess_moves
}

fn get_move_capture(
    position: &Position,
    from: Square,
    direction: DirectionFn,
    piece: Piece,
) -> Option<ChessMove> {
    if let Some(to) = direction(from) {
        if position.is_occupied_by_color(to, piece.get_color().get_opponent_color()) {
            return Some(progess(position, piece, from, to));
        }
    }
    None
}
fn get_move_white_left_capture(position: &Position, from: Square) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_7(from) {
        return None;
    }
    get_move_capture(position, from, directions::up_left, Piece::WhitePawn)
}

fn get_move_white_right_capture(position: &Position, from: Square) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_7(from) {
        return None;
    }
    get_move_capture(position, from, directions::up_right, Piece::WhitePawn)
}

fn get_move_black_left_capture(position: &Position, from: Square) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_2(from) {
        return None;
    }
    get_move_capture(position, from, directions::down_left, Piece::BlackPawn)
}

fn get_move_black_right_capture(position: &Position, from: Square) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_2(from) {
        return None;
    }
    get_move_capture(position, from, directions::down_right, Piece::BlackPawn)
}

fn get_move_en_passant(
    position: &Position,
    from: Square,
    config: EnPassantConfig,
) -> Option<ChessMove> {
    if let (Some(next_square), Some(diagonal_square), Some(en_passant_square)) = (
        (config.next_fn)(from),
        (config.diagonal_fn)(from),
        position.get_en_passant(),
    ) {
        if next_square == en_passant_square
            && position.is_occupied_by_piece(next_square, config.opponents_pawn)
        {
            return Some(en_passant(
                position,
                config.players_pawn,
                from,
                diagonal_square,
                next_square,
            ));
        }
    }
    None
}

fn get_move_white_right_en_passant(position: &Position, from: Square) -> Option<ChessMove> {
    get_move_en_passant(position, from, WHITE_RIGHT_EN_PASSANT)
}

fn get_move_white_left_en_passant(position: &Position, from: Square) -> Option<ChessMove> {
    get_move_en_passant(position, from, WHITE_LEFT_EN_PASSANT)
}

fn get_move_black_right_en_passant(position: &Position, from: Square) -> Option<ChessMove> {
    get_move_en_passant(position, from, BLACK_RIGHT_EN_PASSANT)
}
fn get_move_black_left_en_passant(position: &Position, from: Square) -> Option<ChessMove> {
    get_move_en_passant(position, from, BLACK_LEFT_EN_PASSANT)
}

fn get_moves_white_promotion(position: &Position, from: Square) -> Vec<ChessMove> {
    get_promotion(position, from, WHITE_PROMOTION_CONFIG)
}

fn get_moves_black_promotion(position: &Position, from: Square) -> Vec<ChessMove> {
    get_promotion(position, from, BLACK_PROMOTION_CONFIG)
}

fn get_moves_white_promotion_left_capture(position: &Position, from: Square) -> Vec<ChessMove> {
    get_promotion_capture(position, from, WHITE_PROMOTION_LEFT_CONFIG)
}

fn get_moves_white_promotion_right_capture(position: &Position, from: Square) -> Vec<ChessMove> {
    get_promotion_capture(position, from, WHITE_PROMOTION_RIGHT_CONFIG)
}
fn get_moves_black_promotion_left_capture(position: &Position, from: Square) -> Vec<ChessMove> {
    get_promotion_capture(position, from, BLACK_PROMOTION_LEFT_CONFIG)
}

fn get_moves_black_promotion_right_capture(position: &Position, from: Square) -> Vec<ChessMove> {
    get_promotion_capture(position, from, BLACK_PROMOTION_RIGHT_CONFIG)
}

fn get_promotion(position: &Position, from: Square, config: PromotionConfig) -> Vec<ChessMove> {
    let mut chess_moves: Vec<ChessMove> = Vec::new();
    if (config.is_in_row_fn)(from) {
        if let Some(to) = (config.direction_fn)(from) {
            if !position.is_occupied(to) {
                for promotion_piece in config.promotion_set {
                    chess_moves.push(promote(position, from, to, promotion_piece));
                }
            }
        }
    }
    chess_moves
}

fn get_promotion_capture(
    position: &Position,
    from: Square,
    config: PromotionConfig,
) -> Vec<ChessMove> {
    let mut chess_moves: Vec<ChessMove> = Vec::new();
    if (config.is_in_row_fn)(from) {
        if let Some(to) = (config.direction_fn)(from) {
            if position.is_occupied_by_color(to, config.piece.get_color().get_opponent_color()) {
                for promotion_piece in config.promotion_set {
                    chess_moves.push(promote(position, from, to, promotion_piece));
                }
            }
        }
    }
    chess_moves
}

fn get_move_white_forward(position: &Position, from: Square) -> Option<ChessMove> {
    if !directions::is_in_last_or_second_last_row(from) {
        return get_move_forward(position, from, Piece::WhitePawn, directions::up);
    }
    None
}

fn get_move_black_forward(position: &Position, from: Square) -> Option<ChessMove> {
    if !directions::is_in_first_or_second_row(from) {
        return get_move_forward(position, from, Piece::BlackPawn, directions::down);
    }
    None
}

fn get_move_forward(
    position: &Position,
    from: Square,
    piece: Piece,
    direction: DirectionFn,
) -> Option<ChessMove> {
    if let Some(to) = direction(from) {
        if !position.is_occupied(to) {
            return Some(progess(position, piece, from, to));
        }
    }
    None
}

fn get_move_white_two_forward(position: &Position, from: Square) -> Option<ChessMove> {
    if directions::is_in_row_2(from) {
        return get_move_two_forward(position, from, Piece::WhitePawn, directions::up);
    }
    None
}

fn get_move_black_two_forward(position: &Position, from: Square) -> Option<ChessMove> {
    if directions::is_in_row_7(from) {
        return get_move_two_forward(position, from, Piece::BlackPawn, directions::down);
    }
    None
}

fn get_move_two_forward(
    position: &Position,
    from: Square,
    piece: Piece,
    direction: DirectionFn,
) -> Option<ChessMove> {
    if let Some(one_forward) = direction(from) {
        if !position.is_occupied(one_forward) {
            if let Some(two_forward) = direction(one_forward) {
                if !position.is_occupied(two_forward) {
                    return Some(progess(position, piece, from, two_forward));
                }
            }
        }
    }
    None
}

pub fn en_passant(
    position: &Position,
    piece: Piece,
    from: Square,
    to: Square,
    capture: Square,
) -> ChessMove {
    let new_position = position
        .remove_piece(from)
        .remove_piece(capture)
        .remove_piece(to)
        .put_piece(piece, to)
        .toggle_player()
        .reset_en_passant();

    ChessMove {
        move_type: MoveType::EnPassant,
        piece,
        from,
        to,
        capture: position.get_piece_at(capture),
        pormotion: None,
        position: new_position,
    }
}

pub fn promote(position: &Position, from: Square, to: Square, new_piece: Piece) -> ChessMove {
    let tuple = match position.get_piece_at(to) {
        Some(piece) => (MoveType::PromotionCapture, Some(piece)),
        None => (MoveType::Promotion, None),
    };
    let new_position = position
        .remove_piece(from)
        .remove_piece(to)
        .put_piece(new_piece, to)
        .toggle_player()
        .reset_en_passant();

    ChessMove {
        move_type: tuple.0,
        piece: match new_piece.get_color() {
            Color::Black => Piece::BlackPawn,
            Color::White => Piece::WhitePawn,
        },
        from,
        to,
        capture: tuple.1,
        pormotion: Some(new_piece),
        position: new_position,
    }
}

pub fn set_en_passant_if_necessary(
    position: Position,
    piece: Piece,
    from: Square,
    to: Square,
) -> Position {
    if is_pawn_two_rows_forward(piece, from, to) {
        return position.set_en_passant(to);
    }
    position
}

pub fn is_pawn_two_rows_forward(piece: Piece, from: Square, to: Square) -> bool {
    if !(piece.get_type() == Typ::Pawn) {
        return false;
    }
    match piece.get_color() {
        Color::White => {
            if directions::is_in_row_2(from) && directions::is_in_row_4(to) {
                return true;
            }
        }
        Color::Black => {
            if directions::is_in_row_7(from) && directions::is_in_row_5(to) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests;
