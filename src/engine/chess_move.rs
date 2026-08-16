use crate::engine::{
    movegen::{
        CastlingConfiguration, BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE,
    },
    piece::Piece,
    position::Position,
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
    Promotion(Piece),
    PromotionCapture(Piece),
    EnPassant(Square),
    Castling(CastlingType),
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum CastlingType {
    BlackQueenside,
    BlackKingside,
    WhiteQueenside,
    WhiteKingside,
}
impl CastlingType {
    pub const fn config(self) -> &'static CastlingConfiguration {
        match self {
            Self::BlackQueenside => &BLACK_QUEENSIDE,
            Self::BlackKingside => &BLACK_KINGSIDE,
            Self::WhiteQueenside => &WHITE_QUEENSIDE,
            Self::WhiteKingside => &WHITE_KINGSIDE,
        }
    }
}
