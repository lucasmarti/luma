use flo_canvas::{Draw, GraphicsContext, GraphicsPrimitives};
use std::io::Cursor;

use crate::gui::{
    icon::Icon,
    ui_container::Container,
    ui_element::{CanvasCoordinate, UIElement, UIEvent},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Group {
    Default,
    BlackPromotionButtons,
    WhitePromotionButtons,
}

#[derive(Clone, Copy, PartialEq, Debug)]

pub struct UIButton {
    container: Container,
    icon: Icon,
    event: UIEvent,
    disabled: bool,
    group: Group,
}

impl UIButton {
    pub fn new(container: Container, icon: Icon, event: UIEvent) -> Self {
        UIButton {
            container,
            icon,
            event,
            disabled: false,
            group: Group::Default,
        }
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub fn group(mut self, group: Group) -> Self {
        self.group = group;
        self
    }
    pub fn get_group(&self) -> Group {
        self.group
    }
}
impl UIElement for UIButton {
    fn dispatch_event(&self, canvas_coordinate: CanvasCoordinate) -> Option<UIEvent> {
        if self.disabled {
            return None;
        }
        if canvas_coordinate.x >= self.container.x_horizontal_min
            && canvas_coordinate.x <= self.container.x_horizontal_max
            && canvas_coordinate.y >= self.container.y_vertical_min
            && canvas_coordinate.y <= self.container.y_vertical_max
        {
            Some(self.event)
        } else {
            None
        }
    }

    fn draw(&self, gc: &mut Vec<Draw>) {
        if self.disabled {
            return;
        }
        gc.new_path();
        gc.rect(
            self.container.x_horizontal_min,
            self.container.y_vertical_min,
            self.container.x_horizontal_max,
            self.container.y_vertical_max,
        );
        gc.load_texture(self.icon.texture_id, Cursor::new(self.icon.bytes));
        gc.fill_texture(
            self.icon.texture_id,
            self.container.x_horizontal_min,
            self.container.y_vertical_max,
            self.container.x_horizontal_max,
            self.container.y_vertical_min,
        );
        gc.fill();
    }
}
