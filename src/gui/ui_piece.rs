use crate::{engine::piece::Piece, gui::icon::Icon};

pub fn get_icon(piece: Piece) -> Icon {
    match piece {
        Piece::BlackQueen => Icon::BLACK_QUEEN,
        Piece::BlackKing => Icon::BLACK_KING,
        Piece::BlackBishop => Icon::BLACK_BISHOP,
        Piece::BlackRook => Icon::BLACK_ROOK,
        Piece::BlackPawn => Icon::BLACK_PAWN,
        Piece::BlackKnight => Icon::BLACK_KNIGHT,
        Piece::WhiteQueen => Icon::WHITE_QUEEN,
        Piece::WhiteKing => Icon::WHITE_KING,
        Piece::WhiteBishop => Icon::WHITE_BISHOP,
        Piece::WhiteRook => Icon::WHITE_ROOK,
        Piece::WhitePawn => Icon::WHITE_PAWN,
        Piece::WhiteKnight => Icon::WHITE_KNIGHT,
    }
}
