use crate::engine::{
    bitboard::Bitboard,
    chess_move::MoveType,
    movegen::{
        directions::{self, DirectionFn},
        Square, RANK_1, RANK_2, RANK_7, RANK_8,
    },
    piece::{Color, Piece, *},
    position::{Mve, Position},
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
    generate_move_white_forward,
    generate_move_white_two_forward,
    generate_move_white_left_capture,
    generate_move_white_right_capture,
    generate_move_white_left_en_passant,
    generate_move_white_right_en_passant,
];
const BLACK_MOVE_FUNCTIONS: [MoveFn; 6] = [
    generate_move_black_forward,
    generate_move_black_two_forward,
    generate_move_black_left_capture,
    generate_move_black_right_capture,
    generate_move_black_left_en_passant,
    generate_move_black_right_en_passant,
];

type MoveFn = fn(&Position, Square, &mut Vec<Mve>);

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

pub fn generate_pawn_moves(
    position: &Position,
    piece: Piece,
    square: Square,
    moves: &mut Vec<Mve>,
) {
    match piece.color {
        Color::Black => generate_black_pawn_moves(position, square, moves),
        Color::White => generate_white_pawn_moves(position, square, moves),
    }
}

fn generate_white_pawn_moves(position: &Position, square: Square, moves: &mut Vec<Mve>) {
    for move_function in WHITE_MOVE_FUNCTIONS {
        move_function(position, square, moves);
    }

    generate_promotion(position, square, WHITE_PROMOTION_CONFIG, moves);
    generate_promotion_capture(position, square, WHITE_PROMOTION_LEFT_CONFIG, moves);
    generate_promotion_capture(position, square, WHITE_PROMOTION_RIGHT_CONFIG, moves);
}

fn generate_black_pawn_moves(position: &Position, square: Square, moves: &mut Vec<Mve>) {
    for move_function in BLACK_MOVE_FUNCTIONS {
        move_function(position, square, moves);
    }

    generate_promotion(position, square, BLACK_PROMOTION_CONFIG, moves);
    generate_promotion_capture(position, square, BLACK_PROMOTION_LEFT_CONFIG, moves);
    generate_promotion_capture(position, square, BLACK_PROMOTION_RIGHT_CONFIG, moves);
}

fn generate_move_capture(
    position: &Position,
    from: Square,
    direction: DirectionFn,
    piece: Piece,
    moves: &mut Vec<Mve>,
) {
    if let Some(to) = direction(from) {
        if position.is_occupied_by_color(to, piece.color.get_opponent_color()) {
            progress(position, piece, from, to, moves);
        }
    }
}
fn generate_move_white_left_capture(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    if from.intersects(RANK_7) {
        return;
    }
    generate_move_capture(position, from, directions::up_left, WHITE_PAWN, moves);
}

fn generate_move_white_right_capture(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    if from.intersects(RANK_7) {
        return;
    }
    generate_move_capture(position, from, directions::up_right, WHITE_PAWN, moves);
}

fn generate_move_black_left_capture(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    if from.intersects(RANK_2) {
        return;
    }
    generate_move_capture(position, from, directions::down_left, BLACK_PAWN, moves);
}

fn generate_move_black_right_capture(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    if from.intersects(RANK_2) {
        return;
    }
    generate_move_capture(position, from, directions::down_right, BLACK_PAWN, moves);
}

fn generate_move_en_passant(
    position: &Position,
    from: Square,
    config: EnPassantConfig,
    moves: &mut Vec<Mve>,
) {
    if let (Some(next_square), Some(diagonal_square), Some(en_passant_square)) = (
        (config.next_fn)(from),
        (config.diagonal_fn)(from),
        position.get_en_passant(),
    ) {
        if diagonal_square == en_passant_square
            && position.is_occupied_by_piece(next_square, config.opponents_pawn)
        {
            moves.push(Mve {
                piece: config.players_pawn,
                from,
                to: diagonal_square,
                move_type: MoveType::EnPassant(next_square),
            });
        }
    }
}

fn generate_move_white_right_en_passant(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    generate_move_en_passant(position, from, WHITE_RIGHT_EN_PASSANT, moves)
}

fn generate_move_white_left_en_passant(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    generate_move_en_passant(position, from, WHITE_LEFT_EN_PASSANT, moves);
}

fn generate_move_black_right_en_passant(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    generate_move_en_passant(position, from, BLACK_RIGHT_EN_PASSANT, moves);
}
fn generate_move_black_left_en_passant(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    generate_move_en_passant(position, from, BLACK_LEFT_EN_PASSANT, moves);
}
fn generate_promotion(
    position: &Position,
    from: Square,
    config: PromotionConfig,
    moves: &mut Vec<Mve>,
) {
    if from.intersects(config.from_rank) {
        if let Some(to) = (config.direction_fn)(from) {
            if !position.is_occupied(to) {
                for promotion_piece in config.promotion_set {
                    moves.push(promote(position, from, to, promotion_piece));
                }
            }
        }
    }
}

fn generate_promotion_capture(
    position: &Position,
    from: Square,
    config: PromotionConfig,
    moves: &mut Vec<Mve>,
) {
    if from.intersects(config.from_rank) {
        if let Some(to) = (config.direction_fn)(from) {
            if position.is_occupied_by_color(to, config.piece.color.get_opponent_color()) {
                for promotion_piece in config.promotion_set {
                    moves.push(promote(position, from, to, promotion_piece));
                }
            }
        }
    }
}

fn generate_move_white_forward(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    if !from.intersects(RANK_7 | RANK_8) {
        generate_move_forward(position, from, WHITE_PAWN, directions::up, moves);
    }
}

fn generate_move_black_forward(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    if !from.intersects(RANK_1 | RANK_2) {
        generate_move_forward(position, from, BLACK_PAWN, directions::down, moves);
    }
}

fn generate_move_forward(
    position: &Position,
    from: Square,
    piece: Piece,
    direction: DirectionFn,
    moves: &mut Vec<Mve>,
) {
    if let Some(to) = direction(from) {
        if !position.is_occupied(to) {
            progress(position, piece, from, to, moves);
        }
    }
}

fn generate_move_white_two_forward(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    if from.intersects(RANK_2) {
        generate_move_two_forward(position, from, WHITE_PAWN, directions::up, moves);
    }
}

fn generate_move_black_two_forward(position: &Position, from: Square, moves: &mut Vec<Mve>) {
    if from.intersects(RANK_7) {
        generate_move_two_forward(position, from, BLACK_PAWN, directions::down, moves);
    }
}

fn generate_move_two_forward(
    position: &Position,
    from: Square,
    piece: Piece,
    direction: DirectionFn,
    moves: &mut Vec<Mve>,
) {
    if let Some(one_forward) = direction(from) {
        if !position.is_occupied(one_forward) {
            if let Some(two_forward) = direction(one_forward) {
                if !position.is_occupied(two_forward) {
                    progress(position, piece, from, two_forward, moves);
                }
            }
        }
    }
}

pub fn progress(position: &Position, piece: Piece, from: Square, to: Square, moves: &mut Vec<Mve>) {
    let capture = position.get_piece_at(to);

    let move_type = match get_en_passant(from, to) {
        Some(square) => MoveType::DoublePawnPush(square),
        None => match capture {
            Some(_) => MoveType::Capture,
            None => MoveType::Quiet,
        },
    };

    moves.push(Mve {
        piece,
        from,
        to,
        move_type,
    });
}
pub fn promote(position: &Position, from: Square, to: Square, new_piece: Piece) -> Mve {
    let tuple = match position.get_piece_at(to) {
        Some(piece) => (MoveType::PromotionCapture(new_piece), Some(piece)),
        None => (MoveType::Promotion(new_piece), None),
    };
    Mve {
        piece: match new_piece.color {
            Color::Black => BLACK_PAWN,
            Color::White => WHITE_PAWN,
        },
        from,
        to,
        move_type: tuple.0,
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
