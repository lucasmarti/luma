use crate::engine::piece::*;
use flo_canvas::Color;

pub const BACKGROUND_COLOR: Color = Color::Rgba(0.5, 0.5, 0.5, 1.0);
pub const BLACK_FIELD_COLOR: Color = Color::Rgba(202.0 / 255.0, 207.0 / 255.0, 184.0 / 255.0, 1.0);
pub const WHITE_FIELD_COLOR: Color = Color::Rgba(224.0 / 255.0, 218.0 / 255.0, 193.0 / 255.0, 1.0);
pub const FROM_TO_COLOR: Color = Color::Rgba(100.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0, 1.0);
pub const DROP_TARGET_COLOR: Color = Color::Rgba(1.0, 0.0, 0.0, 5.0);
pub const SELECTED_FIELD_COLOR: Color = Color::Rgba(1.0, 1.0, 1.0, 5.0);
pub const CHECK_COLOR: Color = Color::Rgba(1.0, 0.0, 0.0, 5.0);

pub const ALL_PIECES_SET: [Piece; 12] = [
    Piece::WhiteKing,
    Piece::WhiteRook,
    Piece::WhiteKnight,
    Piece::WhiteBishop,
    Piece::WhiteQueen,
    Piece::WhitePawn,
    Piece::BlackKing,
    Piece::BlackRook,
    Piece::BlackKnight,
    Piece::BlackBishop,
    Piece::BlackQueen,
    Piece::BlackPawn,
];
pub const FIELD_SIZE: f32 = 100.0;
pub const MENU_HEIGHT: f32 = 50.0;
pub const MENU_POS_X: f32 = 0.0;
pub const MENU_POS_Y: f32 = 8.25 * FIELD_SIZE;
pub const WHITE_PROMOTION_OFFSET: f32 = 6.0 * FIELD_SIZE;
pub const BLACK_PROMOTION_OFFSET: f32 = 0.0;
pub const MENU_ICON_WIDTH: f32 = MENU_HEIGHT;
pub const MENU_ICON_HEIGHT: f32 = MENU_HEIGHT;
pub const MENU_WIDTH: f32 = 8.0 * FIELD_SIZE;

pub const BLACK_QUEEN_BUTTON_POS_X: f32 = BLACK_PROMOTION_OFFSET;
pub const BLACK_ROOK_BUTTON_POS_X: f32 = BLACK_PROMOTION_OFFSET + MENU_ICON_WIDTH;
pub const BLACK_BISHOP_BUTTON_POS_X: f32 = BLACK_PROMOTION_OFFSET + 2.0 * MENU_ICON_WIDTH;
pub const BLACK_KNIGHT_BUTTON_POS_X: f32 = BLACK_PROMOTION_OFFSET + 3.0 * MENU_ICON_WIDTH;

pub const WHITE_QUEEN_BUTTON_POS_X: f32 = WHITE_PROMOTION_OFFSET;
pub const WHITE_ROOK_BUTTON_POS_X: f32 = WHITE_PROMOTION_OFFSET + MENU_ICON_WIDTH;
pub const WHITE_BISHOP_BUTTON_POS_X: f32 = WHITE_PROMOTION_OFFSET + 2.0 * MENU_ICON_WIDTH;
pub const WHITE_KNIGHT_BUTTON_POS_X: f32 = WHITE_PROMOTION_OFFSET + 3.0 * MENU_ICON_WIDTH;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    NewGameAsWhite,
    NewGameAsBlack,
    PromoteWhiteQueen,
    PromoteWhiteRook,
    PromoteWhiteBishop,
    PromoteWhiteKnight,
    PromoteBlackQueen,
    PromoteBlackRook,
    PromoteBlackBishop,
    PromoteBlackKnight,
}
