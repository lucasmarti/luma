use flo_canvas::TextureId;

use crate::{
    engine::piece::*,
    gui::{
        assets::{
            self, BB_BYTES, BLACK_BISHOP_TEXTURE_ID, BLACK_KNIGHT_TEXTURE_ID,
            BLACK_QUEEN_TEXTURE_ID, BLACK_ROOK_TEXTURE_ID, BN_BYTES, BQ_BYTES, BR_BYTES,
            NEW_BLACK_BYTES, NEW_BLACK_TEXTURE_ID, NEW_WHITE_BYTES, NEW_WHITE_TEXTURE_ID, WB_BYTES,
            WHITE_BISHOP_TEXTURE_ID, WHITE_KNIGHT_TEXTURE_ID, WHITE_QUEEN_TEXTURE_ID,
            WHITE_ROOK_TEXTURE_ID, WN_BYTES, WQ_BYTES, WR_BYTES,
        },
        chess_board_canvas::ChessBoardCanvas,
    },
};

pub const ALL_PIECES_SET: [Piece; 12] = [
    WHITE_KING,
    WHITE_ROOK,
    WHITE_KNIGHT,
    WHITE_BISHOP,
    WHITE_QUEEN,
    WHITE_PAWN,
    BLACK_KING,
    BLACK_ROOK,
    BLACK_KNIGHT,
    BLACK_BISHOP,
    BLACK_QUEEN,
    BLACK_PAWN,
];
pub const FIELD_SIZE: f32 = 100.0;
pub const MENU_HEIGHT: f32 = 50.0;
pub const MENU_POS_X: f32 = 0.0;
pub const MENU_POS_Y: f32 = 8.25 * FIELD_SIZE;
pub const WHITE_PROMOTION_OFFSET: f32 = 6.0 * FIELD_SIZE;
pub const BLACK_PROMOTION_OFFSET: f32 = 0.0;
pub const MENU_ICON_HEIGTH: f32 = MENU_HEIGHT;
pub const MENU_ICON_WIDTH: f32 = MENU_HEIGHT;
pub const MENU_WIDTH: f32 = 8.0 * FIELD_SIZE;

pub const BLACK_QUEEN_BUTTON_POS_X: f32 = BLACK_PROMOTION_OFFSET;
pub const BLACK_ROOK_BUTTON_POS_X: f32 = BLACK_PROMOTION_OFFSET + MENU_ICON_WIDTH;
pub const BLACK_BISHOP_BUTTON_POS_X: f32 = BLACK_PROMOTION_OFFSET + 2.0 * MENU_ICON_WIDTH;
pub const BLACK_KNIGHT_BUTTON_POS_X: f32 = BLACK_PROMOTION_OFFSET + 3.0 * MENU_ICON_WIDTH;

pub const WHITE_QUEEN_BUTTON_POS_X: f32 = WHITE_PROMOTION_OFFSET;
pub const WHITE_ROOK_BUTTON_POS_X: f32 = WHITE_PROMOTION_OFFSET + MENU_ICON_WIDTH;
pub const WHITE_BISHOP_BUTTON_POS_X: f32 = WHITE_PROMOTION_OFFSET + 2.0 * MENU_ICON_WIDTH;
pub const WHITE_KNIGHT_BUTTON_POS_X: f32 = WHITE_PROMOTION_OFFSET + 3.0 * MENU_ICON_WIDTH;

pub const MENU_BUTTONS: [Button; 10] = [
    NEW_GAME_WHITE_BUTTON,
    NEW_GAME_BLACK_BUTTON,
    PROMOTION_WHITE_BISHOP_BUTTON,
    PROMOTION_WHITE_KNIGHT_BUTTON,
    PROMOTION_WHITE_QUEEN_BUTTON,
    PROMOTION_WHITE_ROOK_BUTTON,
    PROMOTION_BLACK_BISHOP_BUTTON,
    PROMOTION_BLACK_KNIGHT_BUTTON,
    PROMOTION_BLACK_QUEEN_BUTTON,
    PROMOTION_BLACK_ROOK_BUTTON,
];

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

#[derive(Clone)]
pub struct Button {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub texture_id: TextureId,
    pub data: &'static [u8],
    pub action: Action,
}

pub const NEW_GAME_WHITE_BUTTON: Button = Button {
    x_min: 4.25 * FIELD_SIZE,
    x_max: 4.75 * FIELD_SIZE,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: NEW_WHITE_TEXTURE_ID,
    data: NEW_WHITE_BYTES,
    action: Action::NewGameAsWhite,
};
pub const NEW_GAME_BLACK_BUTTON: Button = Button {
    x_min: 3.25 * FIELD_SIZE,
    x_max: 3.75 * FIELD_SIZE,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: NEW_BLACK_TEXTURE_ID,
    data: NEW_BLACK_BYTES,
    action: Action::NewGameAsBlack,
};

pub const PROMOTION_BLACK_QUEEN_BUTTON: Button = Button {
    x_min: BLACK_QUEEN_BUTTON_POS_X,
    x_max: BLACK_QUEEN_BUTTON_POS_X + MENU_ICON_WIDTH,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: BLACK_QUEEN_TEXTURE_ID,
    data: BQ_BYTES,
    action: Action::PromoteBlackQueen,
};

pub const PROMOTION_BLACK_ROOK_BUTTON: Button = Button {
    x_min: BLACK_ROOK_BUTTON_POS_X,
    x_max: BLACK_ROOK_BUTTON_POS_X + MENU_ICON_WIDTH,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: BLACK_ROOK_TEXTURE_ID,
    data: BR_BYTES,
    action: Action::PromoteBlackRook,
};

pub const PROMOTION_BLACK_BISHOP_BUTTON: Button = Button {
    x_min: BLACK_BISHOP_BUTTON_POS_X,
    x_max: BLACK_BISHOP_BUTTON_POS_X + MENU_ICON_WIDTH,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: BLACK_BISHOP_TEXTURE_ID,
    data: BB_BYTES,
    action: Action::PromoteBlackBishop,
};

pub const PROMOTION_BLACK_KNIGHT_BUTTON: Button = Button {
    x_min: BLACK_KNIGHT_BUTTON_POS_X,
    x_max: BLACK_KNIGHT_BUTTON_POS_X + MENU_ICON_WIDTH,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: BLACK_KNIGHT_TEXTURE_ID,
    data: BN_BYTES,
    action: Action::PromoteBlackKnight,
};

pub const PROMOTION_WHITE_QUEEN_BUTTON: Button = Button {
    x_min: WHITE_QUEEN_BUTTON_POS_X,
    x_max: WHITE_QUEEN_BUTTON_POS_X + MENU_ICON_WIDTH,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: WHITE_QUEEN_TEXTURE_ID,
    data: WQ_BYTES,
    action: Action::PromoteWhiteQueen,
};

pub const PROMOTION_WHITE_ROOK_BUTTON: Button = Button {
    x_min: WHITE_ROOK_BUTTON_POS_X,
    x_max: WHITE_ROOK_BUTTON_POS_X + MENU_ICON_WIDTH,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: WHITE_ROOK_TEXTURE_ID,
    data: WR_BYTES,
    action: Action::PromoteWhiteRook,
};

pub const PROMOTION_WHITE_BISHOP_BUTTON: Button = Button {
    x_min: WHITE_BISHOP_BUTTON_POS_X,
    x_max: WHITE_BISHOP_BUTTON_POS_X + MENU_ICON_WIDTH,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: WHITE_BISHOP_TEXTURE_ID,
    data: WB_BYTES,
    action: Action::PromoteWhiteBishop,
};

pub const PROMOTION_WHITE_KNIGHT_BUTTON: Button = Button {
    x_min: WHITE_KNIGHT_BUTTON_POS_X,
    x_max: WHITE_KNIGHT_BUTTON_POS_X + MENU_ICON_WIDTH,
    y_min: MENU_POS_Y,
    y_max: MENU_POS_Y + MENU_HEIGHT,
    texture_id: WHITE_KNIGHT_TEXTURE_ID,
    data: WN_BYTES,
    action: Action::PromoteWhiteKnight,
};
