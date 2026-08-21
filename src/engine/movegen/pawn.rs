use crate::engine::{
    bitboard::Bitboard,
    chess_move::{ChessMove, MoveType},
    movegen::{
        directions::{self, DirectionFn, RowFn},
        Square, FILE_G, FILE_H, RANK_1, RANK_2, RANK_4, RANK_5, RANK_7, RANK_8,
    },
    piece::{Color, Piece, Typ, *},
    position::{CastlingRights, Mve, Position},
};

struct EnPassantConfig {
    players_pawn: Piece,
    opponents_pawn: Piece,
    next_fn: DirectionFn,
    diagonal_fn: DirectionFn,
}

const WHITE_LEFT_EN_PASSANT: EnPassantConfig = EnPassantConfig {
    players_pawn: WHITE_PAWN,
    opponents_pawn: BLACK_PAWN,
    next_fn: directions::left,
    diagonal_fn: directions::up_left,
};

const WHITE_RIGHT_EN_PASSANT: EnPassantConfig = EnPassantConfig {
    players_pawn: WHITE_PAWN,
    opponents_pawn: BLACK_PAWN,
    next_fn: directions::right,
    diagonal_fn: directions::up_right,
};

const BLACK_LEFT_EN_PASSANT: EnPassantConfig = EnPassantConfig {
    players_pawn: BLACK_PAWN,
    opponents_pawn: WHITE_PAWN,
    next_fn: directions::left,
    diagonal_fn: directions::down_left,
};

const BLACK_RIGHT_EN_PASSANT: EnPassantConfig = EnPassantConfig {
    players_pawn: BLACK_PAWN,
    opponents_pawn: WHITE_PAWN,
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

const WHITE_PROMOTION_PIECES: [Piece; 4] = [WHITE_QUEEN, WHITE_ROOK, WHITE_BISHOP, WHITE_KNIGHT];
const BLACK_PROMOTION_PIECES: [Piece; 4] = [BLACK_QUEEN, BLACK_ROOK, BLACK_BISHOP, BLACK_KNIGHT];

struct PromotionConfig {
    piece: Piece,
    from_rank: Bitboard,
    direction_fn: DirectionFn,
    promotion_set: [Piece; 4],
}

const WHITE_PROMOTION_CONFIG: PromotionConfig = PromotionConfig {
    piece: WHITE_PAWN,
    from_rank: RANK_7,
    direction_fn: directions::up,
    promotion_set: WHITE_PROMOTION_PIECES,
};

const WHITE_PROMOTION_LEFT_CONFIG: PromotionConfig = PromotionConfig {
    piece: WHITE_PAWN,
    from_rank: RANK_7,
    direction_fn: directions::up_left,
    promotion_set: WHITE_PROMOTION_PIECES,
};

const WHITE_PROMOTION_RIGHT_CONFIG: PromotionConfig = PromotionConfig {
    piece: WHITE_PAWN,
    from_rank: RANK_7,
    direction_fn: directions::up_right,
    promotion_set: WHITE_PROMOTION_PIECES,
};

const BLACK_PROMOTION_CONFIG: PromotionConfig = PromotionConfig {
    piece: BLACK_PAWN,
    from_rank: RANK_2,
    direction_fn: directions::down,
    promotion_set: BLACK_PROMOTION_PIECES,
};

const BLACK_PROMOTION_LEFT_CONFIG: PromotionConfig = PromotionConfig {
    piece: BLACK_PAWN,
    from_rank: RANK_2,
    direction_fn: directions::down_left,
    promotion_set: BLACK_PROMOTION_PIECES,
};

const BLACK_PROMOTION_RIGHT_CONFIG: PromotionConfig = PromotionConfig {
    piece: BLACK_PAWN,
    from_rank: RANK_2,
    direction_fn: directions::down_right,
    promotion_set: BLACK_PROMOTION_PIECES,
};

pub fn get_pawn_moves(position: &Position, piece: Piece, square: Square) -> Vec<ChessMove> {
    match piece.color {
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
        if position.is_occupied_by_color(to, piece.color.get_opponent_color()) {
            return Some(progress(position, piece, from, to));
        }
    }
    None
}
fn get_move_white_left_capture(position: &Position, from: Square) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if from.intersects(RANK_7) {
        return None;
    }
    get_move_capture(position, from, directions::up_left, WHITE_PAWN)
}

fn get_move_white_right_capture(position: &Position, from: Square) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if from.intersects(RANK_7) {
        return None;
    }
    get_move_capture(position, from, directions::up_right, WHITE_PAWN)
}

fn get_move_black_left_capture(position: &Position, from: Square) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if from.intersects(RANK_2) {
        return None;
    }
    get_move_capture(position, from, directions::down_left, BLACK_PAWN)
}

fn get_move_black_right_capture(position: &Position, from: Square) -> Option<ChessMove> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if from.intersects(RANK_2) {
        return None;
    }
    get_move_capture(position, from, directions::down_right, BLACK_PAWN)
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
        if diagonal_square == en_passant_square
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
    if from.intersects(config.from_rank) {
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
    if from.intersects(config.from_rank) {
        if let Some(to) = (config.direction_fn)(from) {
            if position.is_occupied_by_color(to, config.piece.color.get_opponent_color()) {
                for promotion_piece in config.promotion_set {
                    chess_moves.push(promote(position, from, to, promotion_piece));
                }
            }
        }
    }
    chess_moves
}

fn get_move_white_forward(position: &Position, from: Square) -> Option<ChessMove> {
    if !from.intersects(RANK_7 | RANK_8) {
        return get_move_forward(position, from, WHITE_PAWN, directions::up);
    }
    None
}

fn get_move_black_forward(position: &Position, from: Square) -> Option<ChessMove> {
    if !from.intersects(RANK_1 | RANK_2) {
        return get_move_forward(position, from, BLACK_PAWN, directions::down);
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
            return Some(progress(position, piece, from, to));
        }
    }
    None
}

fn get_move_white_two_forward(position: &Position, from: Square) -> Option<ChessMove> {
    if from.intersects(RANK_2) {
        return get_move_two_forward(position, from, WHITE_PAWN, directions::up);
    }
    None
}

fn get_move_black_two_forward(position: &Position, from: Square) -> Option<ChessMove> {
    if from.intersects(RANK_7) {
        return get_move_two_forward(position, from, BLACK_PAWN, directions::down);
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
                    return Some(progress(position, piece, from, two_forward));
                }
            }
        }
    }
    None
}

pub fn progress(position: &Position, piece: Piece, from: Square, to: Square) -> ChessMove {
    let mut new_pos = *position;
    new_pos.remove_piece(from);
    new_pos.remove_piece(to);
    new_pos.put_piece(piece, to);
    new_pos.set_en_passant(get_en_passant(from, to));
    new_pos.toggle_player();

    let capture = position.get_piece_at(to);

    let move_type = match get_en_passant(from, to) {
        Some(square) => MoveType::DoublePawnPush(square),
        None => match capture {
            Some(_) => MoveType::Capture,
            None => MoveType::Quiet,
        },
    };

    let mve = Mve {
        piece,
        from,
        to,
        move_type,
    };
    ChessMove {
        move_type,
        piece,
        from,
        to,
        capture: capture,
        pormotion: None,
        position: new_pos,
        mve,
    }
}

pub fn en_passant(
    position: &Position,
    piece: Piece,
    from: Square,
    to: Square,
    capture: Square,
) -> ChessMove {
    let mut new_pos = *position;
    new_pos.remove_piece(from);
    new_pos.remove_piece(capture);
    new_pos.remove_piece(to);
    new_pos.put_piece(piece, to);
    new_pos.toggle_player();
    new_pos.set_en_passant(None);

    let mve = Mve {
        piece,
        from,
        to,
        move_type: MoveType::EnPassant(capture),
    };
    ChessMove {
        move_type: MoveType::EnPassant(capture),
        piece,
        from,
        to,
        capture: position.get_piece_at(capture),
        pormotion: None,
        position: new_pos,
        mve,
    }
}

pub fn promote(position: &Position, from: Square, to: Square, new_piece: Piece) -> ChessMove {
    let tuple = match position.get_piece_at(to) {
        Some(piece) => (MoveType::PromotionCapture(new_piece), Some(piece)),
        None => (MoveType::Promotion(new_piece), None),
    };
    let mut new_pos = *position;
    new_pos.remove_piece(from);
    new_pos.remove_piece(to);
    new_pos.put_piece(new_piece, to);
    new_pos.toggle_player();
    new_pos.set_en_passant(None);

    let mve = Mve {
        piece: match new_piece.color {
            Color::Black => BLACK_PAWN,
            Color::White => WHITE_PAWN,
        },
        from,
        to,
        move_type: tuple.0,
    };
    ChessMove {
        move_type: tuple.0,
        piece: match new_piece.color {
            Color::Black => BLACK_PAWN,
            Color::White => WHITE_PAWN,
        },
        from,
        to,
        capture: tuple.1,
        pormotion: Some(new_piece),
        position: new_pos,
        mve,
    }
}
pub fn get_en_passant(from: Square, to: Square) -> Option<Square> {
    if from.intersects(RANK_2) {
        if let Some(one) = directions::up(from) {
            if let Some(two) = directions::up(one) {
                if to == two {
                    return Some(one);
                }
            }
        }
    }

    if from.intersects(RANK_7) {
        if let Some(one) = directions::down(from) {
            if let Some(two) = directions::down(one) {
                if to == two {
                    return Some(one);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests;
