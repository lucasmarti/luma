use crate::{
    chess_moves::{en_passant, progess, promote},
    directions::{self, DirectionFn},
    piece::{
        Piece, BLACK_BISHOP, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
        WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    position::{self, Position},
};

pub fn get_possible_moves(position: &Position, from: u32, piece: Piece) -> Vec<Position> {
    match piece {
        WHITE_PAWN => get_possible_white_moves(position, from),
        BLACK_PAWN => get_possible_black_moves(position, from),
        _ => Vec::new(),
    }
}

fn get_possible_white_moves(position: &Position, from: u32) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
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
            positions.push(chess_move);
        }
    }
    positions.extend(get_moves_white_promotion(position, from));
    positions.extend(get_moves_white_promotion_left_capture(position, from));
    positions.extend(get_moves_white_promotion_right_capture(position, from));
    positions
}

fn get_possible_black_moves(position: &Position, from: u32) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
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
            positions.push(chess_move);
        }
    }
    positions.extend(get_moves_black_promotion(position, from));
    positions.extend(get_moves_black_promotion_left_capture(position, from));
    positions.extend(get_moves_black_promotion_right_capture(position, from));
    positions
}
type MoveFn = fn(&Position, u32) -> Option<Position>;

const WHITE_PROMOTION_PIECES: [Piece; 4] = [WHITE_QUEEN, WHITE_ROOK, WHITE_BISHOP, WHITE_KNIGHT];
const BLACK_PROMOTION_PIECES: [Piece; 4] = [BLACK_QUEEN, BLACK_ROOK, BLACK_BISHOP, BLACK_KNIGHT];

fn get_move_capture(
    position: &Position,
    from: u32,
    direction: DirectionFn,
    piece: Piece,
) -> Option<Position> {
    if let Some(to) = direction(from) {
        if position.is_occupied_by_color(to, piece.color.get_opponent_color()) {
            return Some(progess(position, piece, from, to));
        }
    }
    None
}
fn get_move_white_left_capture(position: &Position, from: u32) -> Option<Position> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_7(from) {
        return None;
    }
    return get_move_capture(position, from, directions::up_left, WHITE_PAWN);
}

fn get_move_white_right_capture(position: &Position, from: u32) -> Option<Position> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_7(from) {
        return None;
    }
    return get_move_capture(position, from, directions::up_right, WHITE_PAWN);
}

fn get_move_black_left_capture(position: &Position, from: u32) -> Option<Position> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_2(from) {
        return None;
    }
    return get_move_capture(position, from, directions::down_left, BLACK_PAWN);
}

fn get_move_black_right_capture(position: &Position, from: u32) -> Option<Position> {
    // Exclude promotion rank captures (handled by promotion capture functions)
    if directions::is_in_row_2(from) {
        return None;
    }
    return get_move_capture(position, from, directions::down_right, BLACK_PAWN);
}

fn get_move_white_right_en_passant(position: &Position, from: u32) -> Option<Position> {
    if directions::right(from) == position.en_passant {
        if let (Some(to), Some(capture)) = (directions::up_right(from), directions::right(from)) {
            return Some(en_passant(position, WHITE_PAWN, from, to, capture));
        }
    }
    None
}

fn get_move_white_left_en_passant(position: &Position, from: u32) -> Option<Position> {
    if directions::left(from) == position.en_passant {
        if let (Some(to), Some(capture)) = (directions::up_left(from), directions::left(from)) {
            return Some(en_passant(position, WHITE_PAWN, from, to, capture));
        }
    }
    None
}

fn get_move_black_right_en_passant(position: &Position, from: u32) -> Option<Position> {
    if directions::right(from) == position.en_passant {
        if let (Some(to), Some(capture)) = (directions::down_right(from), directions::right(from)) {
            return Some(en_passant(position, BLACK_PAWN, from, to, capture));
        }
    }
    None
}
fn get_move_black_left_en_passant(position: &Position, from: u32) -> Option<Position> {
    if directions::left(from) == position.en_passant {
        if let (Some(to), Some(capture)) = (directions::down_left(from), directions::left(from)) {
            return Some(en_passant(position, BLACK_PAWN, from, to, capture));
        }
    }
    None
}

fn get_moves_white_promotion(position: &Position, from: u32) -> Vec<Position> {
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

fn get_moves_black_promotion(position: &Position, from: u32) -> Vec<Position> {
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

fn get_moves_white_promotion_left_capture(position: &Position, from: u32) -> Vec<Position> {
    if directions::is_in_row_7(from) {
        return get_promotion_capture(
            position,
            from,
            directions::up_left,
            WHITE_PAWN,
            WHITE_PROMOTION_PIECES.to_vec(),
        );
    }
    Vec::new()
}

fn get_moves_white_promotion_right_capture(position: &Position, from: u32) -> Vec<Position> {
    if directions::is_in_row_7(from) {
        return get_promotion_capture(
            position,
            from,
            directions::up_right,
            WHITE_PAWN,
            WHITE_PROMOTION_PIECES.to_vec(),
        );
    }
    Vec::new()
}

fn get_moves_black_promotion_left_capture(position: &Position, from: u32) -> Vec<Position> {
    if directions::is_in_row_2(from) {
        return get_promotion_capture(
            position,
            from,
            directions::down_left,
            BLACK_PAWN,
            BLACK_PROMOTION_PIECES.to_vec(),
        );
    }
    Vec::new()
}

fn get_moves_black_promotion_right_capture(position: &Position, from: u32) -> Vec<Position> {
    if directions::is_in_row_2(from) {
        return get_promotion_capture(
            position,
            from,
            directions::down_right,
            BLACK_PAWN,
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
) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    if let Some(to) = direction(from) {
        if !position.is_occupied(to) {
            for promotion_piece in promotion_set {
                positions.push(promote(position, from, to, promotion_piece));
            }
        }
    }
    positions
}

fn get_promotion_capture(
    position: &Position,
    from: u32,
    direction: DirectionFn,
    piece: Piece,
    promotion_set: Vec<Piece>,
) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    if let Some(to) = direction(from) {
        if position.is_occupied_by_color(to, piece.color.get_opponent_color()) {
            for promotion_piece in promotion_set {
                positions.push(promote(position, from, to, promotion_piece));
            }
        }
    }
    positions
}

fn get_move_white_forward(position: &Position, from: u32) -> Option<Position> {
    if !directions::is_in_last_or_second_last_row(from) {
        return get_move_forward(position, from, WHITE_PAWN, directions::up);
    }
    None
}

fn get_move_black_forward(position: &Position, from: u32) -> Option<Position> {
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
) -> Option<Position> {
    if let Some(to) = direction(from) {
        if !position.is_occupied(to) {
            return Some(progess(position, piece, from, to));
        }
    }
    None
}

fn get_move_white_two_forward(position: &Position, from: u32) -> Option<Position> {
    if directions::is_in_row_2(from) {
        return get_move_two_forward(position, from, WHITE_PAWN, directions::up);
    }
    None
}

fn get_move_black_two_forward(position: &Position, from: u32) -> Option<Position> {
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
) -> Option<Position> {
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

mod tests;
