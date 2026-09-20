use crate::engine::{
    movegen::{
        CastlingConfiguration, BLACK_KINGSIDE, BLACK_QUEENSIDE, WHITE_KINGSIDE, WHITE_QUEENSIDE,
    },
    piece::Piece,
    Square,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum MoveType {
    Quiet,
    Capture,
    DoublePawnPush(Square),
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
