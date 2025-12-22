use std::io::Cursor;

use flo_canvas::{Color, Draw, GraphicsContext, GraphicsPrimitives};

use crate::{
    engine::directions::squares::Square,
    gui::{
        configuration::{DROP_TARGET_COLOR, FROM_TO_COLOR, SELECTED_FIELD_COLOR},
        icon::Icon,
        ui_container::Container,
        ui_element::{CanvasCoordinate, UIElement, UIEvent},
    },
};

#[derive(Clone)]
pub struct UISquare {
    id: Square,
    container: Container,
    color: Color,
    piece: Option<Icon>,
    selected: bool,
    drop_target: bool,
    last_move: bool,
    check: bool,
}

impl UISquare {
    pub fn new(id: Square, color: Color) -> Self {
        Self {
            container: Container::default(),
            id,
            color,
            piece: None,
            selected: false,
            drop_target: false,
            last_move: false,
            check: false,
        }
    }

    pub fn reset(&mut self) {
        self.piece = None;
        self.selected = false;
        self.drop_target = false;
        self.last_move = false;
        self.check = false;
    }
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn set_drop_target(&mut self, drop_target: bool) {
        self.drop_target = drop_target;
    }

    pub fn set_last_move(&mut self, last_move: bool) {
        self.last_move = last_move;
    }

    pub fn set_icon(&mut self, piece: Icon) {
        self.piece = Some(piece);
    }
    pub fn set_container(&mut self, container: Container) {
        self.container = container;
    }

    fn draw_icon(&self, gc: &mut Vec<Draw>) {
        if let Some(icon) = &self.piece {
            gc.load_texture(icon.texture_id, Cursor::new(icon.bytes));
            gc.fill_texture(
                icon.texture_id,
                self.container.x_horizontal_min,
                self.container.y_vertical_max,
                self.container.x_horizontal_max,
                self.container.y_vertical_min,
            );
            gc.fill();
        }
    }
    fn draw_drop_target(&self, gc: &mut Vec<Draw>) {
        if self.drop_target {
            gc.new_path();
            gc.circle(
                self.container.get_center_x(),
                self.container.get_center_y(),
                self.container.get_height() / 10.0,
            );
            gc.line_width(2.0);
            gc.stroke_color(DROP_TARGET_COLOR);
            gc.stroke();
        }
    }

    fn draw_selected_field(&self, gc: &mut Vec<Draw>) {
        if self.selected {
            gc.new_path();
            gc.rect(
                self.container.x_horizontal_min,
                self.container.y_vertical_max,
                self.container.x_horizontal_max,
                self.container.y_vertical_min,
            );
            gc.line_width(2.0);
            gc.stroke_color(SELECTED_FIELD_COLOR);
            gc.stroke();
        }
    }
    fn draw_last_move_field(&self, gc: &mut Vec<Draw>) {
        if self.last_move {
            gc.new_path();
            gc.circle(
                self.container.get_center_x(),
                self.container.get_center_y(),
                self.container.get_height() / 10.0,
            );
            gc.line_width(2.0);
            gc.stroke_color(FROM_TO_COLOR);
            gc.stroke();
        }
    }
}

impl UIElement for UISquare {
    fn dispatch_event(&self, canvas_coordinate: CanvasCoordinate) -> Option<UIEvent> {
        if canvas_coordinate.x >= self.container.x_horizontal_min
            && canvas_coordinate.x <= self.container.x_horizontal_max
            && canvas_coordinate.y >= self.container.y_vertical_min
            && canvas_coordinate.y <= self.container.y_vertical_max
        {
            Some(UIEvent::Square(self.id))
        } else {
            None
        }
    }

    fn draw(&self, gc: &mut Vec<Draw>) {
        gc.new_path();
        gc.rect(
            self.container.x_horizontal_min,
            self.container.y_vertical_max,
            self.container.x_horizontal_max,
            self.container.y_vertical_min,
        );
        gc.fill_color(self.color);
        gc.fill();
        self.draw_icon(gc);
        self.draw_selected_field(gc);
        self.draw_drop_target(gc);
    }
}
