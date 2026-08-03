use crate::engine::{
    piece::Piece,
    position::{CastlingType, Position},
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
