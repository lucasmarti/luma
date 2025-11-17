use flo_canvas::{Draw, GraphicsContext, GraphicsPrimitives};
use std::io::Cursor;

use crate::gui::{
    configuration::{MENU_ICON_HEIGHT, MENU_ICON_WIDTH},
    icon::Icon,
    ui_element::{CanvasCoordinate, UIElement, UIEvent},
};

pub struct UIButton {
    icon: Icon,
    event: UIEvent,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    disabled: bool,
}

impl UIButton {
    pub fn new(
        canvas_coordinate: CanvasCoordinate,
        icon: Icon,
        event: UIEvent,
        disabled: bool,
    ) -> Self {
        UIButton {
            icon,
            event,
            x_min: canvas_coordinate.x,
            x_max: canvas_coordinate.x + MENU_ICON_WIDTH,
            y_min: canvas_coordinate.y,
            y_max: canvas_coordinate.y + MENU_ICON_HEIGHT,
            disabled,
        }
    }
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}
impl UIElement for UIButton {
    fn dispatch_event(&self, canvas_coordinate: CanvasCoordinate) -> Option<UIEvent> {
        if self.disabled {
            return None;
        }
        if canvas_coordinate.x >= self.x_min
            && canvas_coordinate.x <= self.x_max
            && canvas_coordinate.y >= self.y_min
            && canvas_coordinate.y <= self.y_max
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
        gc.rect(self.x_min, self.y_min, self.x_max, self.y_max);
        gc.load_texture(self.icon.texture_id, Cursor::new(self.icon.bytes));
        gc.fill_texture(
            self.icon.texture_id,
            self.x_min,
            self.y_max,
            self.x_max,
            self.y_min,
        );
        gc.fill();
    }
}
