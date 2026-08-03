use crate::engine::{
    check::filter_checks,
    movegen::config::*,
    piece::{Color, Piece},
    position::{castling_rights::CastlingType, Position},
    Square,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct ChessMove {
    pub position: Position,
    pub move_type: MoveType,
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub capture: Option<Piece>,
    pub pormotion: Option<Piece>,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum MoveType {
    Quiet,
    Capture,
    Promotion,
    PromotionCapture,
    EnPassant,
    Castling { castling_type: CastlingType },
}

pub fn get_current_player_moves(position: &Position) -> Vec<ChessMove> {
    match position.get_player() {
        Color::Black => get_black_moves(position),
        Color::White => get_white_moves(position),
    }
}

pub fn get_black_mobility(position: &Position) -> Vec<ChessMove> {
    get_moves(position, BLACK_MOVE_CONFIG, true)
}

pub fn get_white_mobility(position: &Position) -> Vec<ChessMove> {
    get_moves(position, WHITE_MOVE_CONFIG, true)
}

pub fn get_white_moves(position: &Position) -> Vec<ChessMove> {
    get_moves(position, WHITE_MOVE_CONFIG, false)
}

pub fn get_black_moves(position: &Position) -> Vec<ChessMove> {
    get_moves(position, BLACK_MOVE_CONFIG, false)
}

fn get_moves(position: &Position, config: Config, ignore_checks: bool) -> Vec<ChessMove> {
    let mut new_chess_moves: Vec<ChessMove> = Vec::new();
    new_chess_moves.extend(get_new_positions(position, config.bishop, config.bishop_fn));
    new_chess_moves.extend(get_new_positions(position, config.king, config.king_fn));
    new_chess_moves.extend(get_new_positions(position, config.queen, config.queen_fn));
    new_chess_moves.extend(get_new_positions(position, config.rook, config.rook_fn));
    new_chess_moves.extend(get_new_positions(position, config.knight, config.knight_fn));
    new_chess_moves.extend(get_new_positions(position, config.pawn, config.pawn_fn));
    new_chess_moves.extend((config.castling_move_fn)(position));

    if ignore_checks {
        new_chess_moves
    } else {
        filter_checks(new_chess_moves, position.get_player())
    }
}
fn get_new_positions(position: &Position, piece: Piece, get_moves_fn: MovesFn) -> Vec<ChessMove> {
    let mut new_chess_moves: Vec<ChessMove> = Vec::new();
    for square in position.get_squares(piece.color, piece.typ).iter() {
        new_chess_moves.extend(get_moves_fn(position, piece, square));
    }
    new_chess_moves
}
#[cfg(test)]
mod tests;
