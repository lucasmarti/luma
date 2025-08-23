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
pub const NEW_WHITE_BYTES: &[u8] = include_bytes!["assets/new-white.png"];
pub const NEW_BLACK_BYTES: &[u8] = include_bytes!["assets/new-black.png"];

pub const WHITE_KING_TEXTURE_ID: TextureId = TextureId(0);
pub const WHITE_QUEEN_TEXTURE_ID: TextureId = TextureId(1);
pub const WHITE_ROOK_TEXTURE_ID: TextureId = TextureId(2);
pub const WHITE_PAWN_TEXTURE_ID: TextureId = TextureId(3);
pub const WHITE_KNIGHT_TEXTURE_ID: TextureId = TextureId(4);
pub const WHITE_BISHOP_TEXTURE_ID: TextureId = TextureId(5);
pub const BLACK_KING_TEXTURE_ID: TextureId = TextureId(6);
pub const BLACK_QUEEN_TEXTURE_ID: TextureId = TextureId(7);
pub const BLACK_ROOK_TEXTURE_ID: TextureId = TextureId(8);
pub const BLACK_PAWN_TEXTURE_ID: TextureId = TextureId(9);
pub const BLACK_KNIGHT_TEXTURE_ID: TextureId = TextureId(10);
pub const BLACK_BISHOP_TEXTURE_ID: TextureId = TextureId(11);

pub const NEW_WHITE_TEXTURE_ID: TextureId = TextureId(12);
pub const NEW_BLACK_TEXTURE_ID: TextureId = TextureId(13);

pub fn get_texture_id(piece: Piece) -> TextureId {
    match piece {
        WHITE_KING => WHITE_KING_TEXTURE_ID,
        WHITE_QUEEN => WHITE_QUEEN_TEXTURE_ID,
        WHITE_ROOK => WHITE_ROOK_TEXTURE_ID,
        WHITE_PAWN => WHITE_PAWN_TEXTURE_ID,
        WHITE_KNIGHT => WHITE_KNIGHT_TEXTURE_ID,
        WHITE_BISHOP => WHITE_BISHOP_TEXTURE_ID,
        BLACK_KING => BLACK_KING_TEXTURE_ID,
        BLACK_QUEEN => BLACK_QUEEN_TEXTURE_ID,
        BLACK_ROOK => BLACK_ROOK_TEXTURE_ID,
        BLACK_PAWN => BLACK_PAWN_TEXTURE_ID,
        BLACK_KNIGHT => BLACK_KNIGHT_TEXTURE_ID,
        BLACK_BISHOP => BLACK_BISHOP_TEXTURE_ID,
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
