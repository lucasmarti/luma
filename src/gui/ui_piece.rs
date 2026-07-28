use crate::{engine::piece::Piece, gui::icon::Icon};

pub fn get_icon(piece: Piece) -> Icon {
    match piece {
        Piece::BLACK_QUEEN => Icon::BLACK_QUEEN,
        Piece::BLACK_KING => Icon::BLACK_KING,
        Piece::BLACK_BISHOP => Icon::BLACK_BISHOP,
        Piece::BLACK_ROOK => Icon::BLACK_ROOK,
        Piece::BLACK_PAWN => Icon::BLACK_PAWN,
        Piece::BLACK_KNIGHT => Icon::BLACK_KNIGHT,
        Piece::WHITE_QUEEN => Icon::WHITE_QUEEN,
        Piece::WHITE_KING => Icon::WHITE_KING,
        Piece::WHITE_BISHOP => Icon::WHITE_BISHOP,
        Piece::WHITE_ROOK => Icon::WHITE_ROOK,
        Piece::WHITE_PAWN => Icon::WHITE_PAWN,
        Piece::WHITE_KNIGHT => Icon::WHITE_KNIGHT,
    }
}
