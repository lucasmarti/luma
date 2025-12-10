use flo_canvas::{Draw, GraphicsContext, GraphicsPrimitives};

use crate::{
    engine::piece::{Color, Piece},
    gui::{
        configuration::*,
        icon::*,
        ui_button::{Group, UIButton},
        ui_container::Container,
        ui_element::{CanvasCoordinate, UIElement, UIEvent},
        ui_layout::*,
    },
};

pub struct UIMenu {
    container: Container,
    buttons: Vec<UIButton>,
}

impl UIMenu {
    pub fn new(container: Container) -> Self {
        let layout = MenuLayout::new(container);
        let menu = UIMenu {
            container,
            buttons: vec![
                UIButton::new(
                    layout.get(Column::Id_1),
                    Icon::NEW_GAME_BLACK,
                    UIEvent::NewGameAsButtonClicked(Color::Black),
                ),
                UIButton::new(
                    layout.get(Column::Id_2),
                    Icon::NEW_GAME_WHITE,
                    UIEvent::NewGameAsButtonClicked(Color::White),
                ),
                UIButton::new(
                    layout.get(Column::Id_3),
                    Icon::TURN_BOARD,
                    UIEvent::TurnBoardClicked,
                ),
                UIButton::new(
                    layout.get(Column::Id_13),
                    Icon::WHITE_BISHOP,
                    UIEvent::PromoteToButtonClicked(Piece::WhiteBishop),
                )
                .disabled(true)
                .group(Group::WhitePromotionButtons),
                UIButton::new(
                    layout.get(Column::Id_14),
                    Icon::WHITE_KNIGHT,
                    UIEvent::PromoteToButtonClicked(Piece::WhiteKnight),
                )
                .disabled(true)
                .group(Group::WhitePromotionButtons),
                UIButton::new(
                    layout.get(Column::Id_15),
                    Icon::WHITE_QUEEN,
                    UIEvent::PromoteToButtonClicked(Piece::WhiteQueen),
                )
                .disabled(true)
                .group(Group::WhitePromotionButtons),
                UIButton::new(
                    layout.get(Column::Id_16),
                    Icon::WHITE_ROOK,
                    UIEvent::PromoteToButtonClicked(Piece::WhiteRook),
                )
                .disabled(true)
                .group(Group::WhitePromotionButtons),
                UIButton::new(
                    layout.get(Column::Id_13),
                    Icon::BLACK_BISHOP,
                    UIEvent::PromoteToButtonClicked(Piece::BlackBishop),
                )
                .group(Group::BlackPromotionButtons)
                .disabled(true),
                UIButton::new(
                    layout.get(Column::Id_14),
                    Icon::BLACK_KNIGHT,
                    UIEvent::PromoteToButtonClicked(Piece::BlackKnight),
                )
                .disabled(true)
                .group(Group::BlackPromotionButtons),
                UIButton::new(
                    layout.get(Column::Id_15),
                    Icon::BLACK_QUEEN,
                    UIEvent::PromoteToButtonClicked(Piece::BlackQueen),
                )
                .disabled(true)
                .group(Group::BlackPromotionButtons),
                UIButton::new(
                    layout.get(Column::Id_16),
                    Icon::BLACK_ROOK,
                    UIEvent::PromoteToButtonClicked(Piece::BlackRook),
                )
                .disabled(true)
                .group(Group::BlackPromotionButtons),
            ],
        };
        menu
    }

    fn set_group_disabled(&mut self, disabled: bool, group: Group) {
        for button in self
            .buttons
            .iter_mut()
            .filter(|button| button.get_group() == group)
        {
            button.set_disabled(disabled);
        }
    }
    pub fn set_black_promotion_buttons_disabled(&mut self, disabled: bool) {
        self.set_group_disabled(disabled, Group::BlackPromotionButtons);
    }

    pub fn set_white_promotion_buttons_disabled(&mut self, disabled: bool) {
        self.set_group_disabled(disabled, Group::WhitePromotionButtons);
    }
}

impl UIElement for UIMenu {
    fn dispatch_event(&self, canvas_coordinate: CanvasCoordinate) -> Option<UIEvent> {
        for button in self.buttons.iter().clone() {
            if let Some(event) = button.dispatch_event(canvas_coordinate) {
                return Some(event);
            }
        }
        None
    }

    fn draw(&self, gc: &mut Vec<Draw>) {
        gc.new_path();
        gc.fill_color(WHITE_FIELD_COLOR);
        gc.rect(
            self.container.x_horizontal_min,
            self.container.y_vertical_min,
            self.container.x_horizontal_max,
            self.container.y_vertical_max,
        );
        gc.fill();
        for button in self.buttons.iter().clone() {
            button.draw(gc);
        }
    }
}
