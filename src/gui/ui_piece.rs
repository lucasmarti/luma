use crate::{engine::piece::*, gui::icon::Icon};

pub fn get_icon(piece: Piece) -> Icon {
    match piece {
        BLACK_QUEEN => Icon::BLACK_QUEEN,
        BLACK_KING => Icon::BLACK_KING,
        BLACK_BISHOP => Icon::BLACK_BISHOP,
        BLACK_ROOK => Icon::BLACK_ROOK,
        BLACK_PAWN => Icon::BLACK_PAWN,
        BLACK_KNIGHT => Icon::BLACK_KNIGHT,
        WHITE_QUEEN => Icon::WHITE_QUEEN,
        WHITE_KING => Icon::WHITE_KING,
        WHITE_BISHOP => Icon::WHITE_BISHOP,
        WHITE_ROOK => Icon::WHITE_ROOK,
        WHITE_PAWN => Icon::WHITE_PAWN,
        WHITE_KNIGHT => Icon::WHITE_KNIGHT,
    }
}
