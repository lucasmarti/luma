use flo_canvas::TextureId;

use crate::engine::piece::{
    Piece, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
    WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};

pub const BK_BYTES: &[u8] = include_bytes!["assets/black-king.png"];
pub const BQ_BYTES: &[u8] = include_bytes!["assets/black-queen.png"];
pub const BR_BYTES: &[u8] = include_bytes!["assets/black-rook.png"];
pub const BP_BYTES: &[u8] = include_bytes!["assets/black-pawn.png"];
pub const BN_BYTES: &[u8] = include_bytes!["assets/black-knight.png"];
pub const BB_BYTES: &[u8] = include_bytes!["assets/black-bishop.png"];

pub const WK_BYTES: &[u8] = include_bytes!["assets/white-king.png"];
pub const WQ_BYTES: &[u8] = include_bytes!["assets/white-queen.png"];
pub const WR_BYTES: &[u8] = include_bytes!["assets/white-rook.png"];
pub const WP_BYTES: &[u8] = include_bytes!["assets/white-pawn.png"];
pub const WN_BYTES: &[u8] = include_bytes!["assets/white-knight.png"];
pub const WB_BYTES: &[u8] = include_bytes!["assets/white-bishop.png"];

pub fn get_texture_id(piece: Piece) -> TextureId {
    match piece {
        WHITE_KING => TextureId(0),
        WHITE_QUEEN => TextureId(1),
        WHITE_ROOK => TextureId(2),
        WHITE_PAWN => TextureId(3),
        WHITE_KNIGHT => TextureId(4),
        WHITE_BISHOP => TextureId(5),
        BLACK_KING => TextureId(6),
        BLACK_QUEEN => TextureId(7),
        BLACK_ROOK => TextureId(8),
        BLACK_PAWN => TextureId(9),
        BLACK_KNIGHT => TextureId(10),
        BLACK_BISHOP => TextureId(11),
    }
}
pub fn get_bytes(piece: Piece) -> &'static [u8] {
    match piece {
        WHITE_KING => WK_BYTES,
        WHITE_QUEEN => WQ_BYTES,
        WHITE_ROOK => WR_BYTES,
        WHITE_PAWN => WP_BYTES,
        WHITE_KNIGHT => WN_BYTES,
        WHITE_BISHOP => WB_BYTES,
        BLACK_KING => BK_BYTES,
        BLACK_QUEEN => BQ_BYTES,
        BLACK_ROOK => BR_BYTES,
        BLACK_PAWN => BP_BYTES,
        BLACK_KNIGHT => BN_BYTES,
        BLACK_BISHOP => BB_BYTES,
    }
}
