use flo_canvas::{Draw, GraphicsContext, GraphicsPrimitives};

use crate::{
    engine::piece::{Color, Piece},
    gui::{
        configuration::*,
        icon::*,
        ui_button::UIButton,
        ui_element::{CanvasCoordinate, UIElement, UIEvent},
    },
};

pub struct UIMenu {
    white_promotion_buttons: [UIButton; 4],
    black_promotion_buttons: [UIButton; 4],
    new_game_white_button: UIButton,
    new_game_black_button: UIButton,
}

impl UIMenu {
    pub fn new() -> Self {
        UIMenu {
            white_promotion_buttons: [
                UIButton::new(
                    CanvasCoordinate {
                        x: WHITE_BISHOP_BUTTON_POS_X,
                        y: MENU_POS_Y,
                    },
                    Icon::WHITE_BISHOP,
                    UIEvent::PromoteToButtonClicked(Piece::WhiteBishop),
                    true,
                ),
                UIButton::new(
                    CanvasCoordinate {
                        x: WHITE_KNIGHT_BUTTON_POS_X,
                        y: MENU_POS_Y,
                    },
                    Icon::WHITE_KNIGHT,
                    UIEvent::PromoteToButtonClicked(Piece::WhiteKnight),
                    true,
                ),
                UIButton::new(
                    CanvasCoordinate {
                        x: WHITE_QUEEN_BUTTON_POS_X,
                        y: MENU_POS_Y,
                    },
                    Icon::WHITE_QUEEN,
                    UIEvent::PromoteToButtonClicked(Piece::WhiteQueen),
                    true,
                ),
                UIButton::new(
                    CanvasCoordinate {
                        x: WHITE_ROOK_BUTTON_POS_X,
                        y: MENU_POS_Y,
                    },
                    Icon::WHITE_ROOK,
                    UIEvent::PromoteToButtonClicked(Piece::WhiteRook),
                    true,
                ),
            ],
            black_promotion_buttons: [
                UIButton::new(
                    CanvasCoordinate {
                        x: BLACK_BISHOP_BUTTON_POS_X,
                        y: MENU_POS_Y,
                    },
                    Icon::BLACK_BISHOP,
                    UIEvent::PromoteToButtonClicked(Piece::BlackBishop),
                    true,
                ),
                UIButton::new(
                    CanvasCoordinate {
                        x: BLACK_KNIGHT_BUTTON_POS_X,
                        y: MENU_POS_Y,
                    },
                    Icon::BLACK_KNIGHT,
                    UIEvent::PromoteToButtonClicked(Piece::BlackKnight),
                    true,
                ),
                UIButton::new(
                    CanvasCoordinate {
                        x: BLACK_QUEEN_BUTTON_POS_X,
                        y: MENU_POS_Y,
                    },
                    Icon::BLACK_QUEEN,
                    UIEvent::PromoteToButtonClicked(Piece::BlackQueen),
                    true,
                ),
                UIButton::new(
                    CanvasCoordinate {
                        x: BLACK_ROOK_BUTTON_POS_X,
                        y: MENU_POS_Y,
                    },
                    Icon::BLACK_ROOK,
                    UIEvent::PromoteToButtonClicked(Piece::BlackRook),
                    true,
                ),
            ],

            new_game_black_button: UIButton::new(
                CanvasCoordinate {
                    x: 3.25 * FIELD_SIZE,
                    y: MENU_POS_Y,
                },
                Icon::NEW_GAME_BLACK,
                UIEvent::NewGameAsButtonClicked(Color::Black),
                false,
            ),
            new_game_white_button: UIButton::new(
                CanvasCoordinate {
                    x: 4.25 * FIELD_SIZE,
                    y: MENU_POS_Y,
                },
                Icon::NEW_GAME_WHITE,
                UIEvent::NewGameAsButtonClicked(Color::White),
                false,
            ),
        }
    }
    pub fn set_black_promotion_buttons_disabled(&mut self, disabled: bool) {
        for button in self.black_promotion_buttons.iter_mut() {
            button.set_disabled(disabled);
        }
    }
    pub fn set_white_promotion_buttons_disabled(&mut self, disabled: bool) {
        for button in self.white_promotion_buttons.iter_mut() {
            button.set_disabled(disabled);
        }
    }
}

impl UIElement for UIMenu {
    fn dispatch_event(&self, canvas_coordinate: CanvasCoordinate) -> Option<UIEvent> {
        for button in self.white_promotion_buttons.iter().clone() {
            if let Some(event) = button.dispatch_event(canvas_coordinate) {
                return Some(event);
            }
        }
        for button in self.black_promotion_buttons.iter().clone() {
            if let Some(event) = button.dispatch_event(canvas_coordinate) {
                return Some(event);
            }
        }
        if let Some(event) = self.new_game_white_button.dispatch_event(canvas_coordinate) {
            return Some(event);
        }
        if let Some(event) = self.new_game_black_button.dispatch_event(canvas_coordinate) {
            return Some(event);
        }
        None
    }

    fn draw(&self, gc: &mut Vec<Draw>) {
        gc.new_path();
        gc.fill_color(WHITE_FIELD_COLOR);
        gc.rect(MENU_POS_X, MENU_POS_Y, MENU_WIDTH, MENU_POS_Y + MENU_HEIGHT);
        gc.fill();
        for button in self.black_promotion_buttons.iter().clone() {
            button.draw(gc);
        }
        for button in self.white_promotion_buttons.iter().clone() {
            button.draw(gc);
        }
        self.new_game_black_button.draw(gc);
        self.new_game_white_button.draw(gc);
    }
}
