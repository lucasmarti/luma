use std::{fs::File, io::Cursor};

use flo_canvas::{Color, Draw, DrawingTarget, GraphicsContext, GraphicsPrimitives};

use crate::{
    engine::directions::squares::Square,
    gui::{
        configuration::{DROP_TARGET_COLOR, FIELD_SIZE, FROM_TO_COLOR, SELECTED_FIELD_COLOR},
        icon::Icon,
        ui_element::{CanvasCoordinate, UIElement, UIEvent},
    },
};

#[derive(Clone)]
pub struct UISquare {
    id: Square,
    coord_x: f32,
    coord_y: f32,
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
            coord_x: 0.0,
            coord_y: 0.0,
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
    pub fn set_position(&mut self, coord_x: f32, coord_y: f32) {
        self.coord_x = coord_x;
        self.coord_y = coord_y;
    }

    fn draw_icon(&self, gc: &mut Vec<Draw>) {
        if let Some(icon) = &self.piece {
            gc.load_texture(icon.texture_id, Cursor::new(icon.bytes));
            gc.fill_texture(
                icon.texture_id,
                self.coord_x,
                self.coord_y + FIELD_SIZE,
                self.coord_x + FIELD_SIZE,
                self.coord_y,
            );
            gc.fill();
        }
    }
    fn draw_drop_target(&self, gc: &mut Vec<Draw>) {
        if self.drop_target {
            gc.new_path();
            let center_x = self.coord_x + (FIELD_SIZE / 2.0);
            let center_y = self.coord_y + (FIELD_SIZE / 2.0);
            gc.circle(center_x, center_y, FIELD_SIZE / 10.0);
            gc.line_width(2.0);
            gc.stroke_color(DROP_TARGET_COLOR);
            gc.stroke();
        }
    }

    fn draw_selected_field(&self, gc: &mut Vec<Draw>) {
        if self.selected {
            gc.new_path();
            gc.rect(
                self.coord_x,
                self.coord_y,
                self.coord_x + FIELD_SIZE,
                self.coord_y + FIELD_SIZE,
            );
            gc.line_width(2.0);
            gc.stroke_color(SELECTED_FIELD_COLOR);
            gc.stroke();
        }
    }
    fn draw_last_move_field(&self, gc: &mut Vec<Draw>) {
        if self.last_move {
            gc.new_path();
            let center_x = self.coord_x + (FIELD_SIZE / 2.0);
            let center_y = self.coord_y + (FIELD_SIZE / 2.0);
            gc.circle(center_x, center_y, FIELD_SIZE / 10.0);
            gc.line_width(2.0);
            gc.stroke_color(FROM_TO_COLOR);
            gc.stroke();
        }
    }
}

impl UIElement for UISquare {
    fn dispatch_event(&self, canvas_coordinate: CanvasCoordinate) -> Option<UIEvent> {
        if canvas_coordinate.x >= self.coord_x
            && canvas_coordinate.x <= self.coord_x + FIELD_SIZE
            && canvas_coordinate.y >= self.coord_y
            && canvas_coordinate.y <= self.coord_y + FIELD_SIZE
        {
            Some(UIEvent::SquareClicked(self.id))
        } else {
            None
        }
    }

    fn draw(&self, gc: &mut Vec<Draw>) {
        gc.new_path();
        gc.rect(
            self.coord_x,
            self.coord_y,
            self.coord_x + FIELD_SIZE,
            self.coord_y + FIELD_SIZE,
        );
        gc.fill_color(self.color);
        gc.fill();
        self.draw_icon(gc);
        self.draw_selected_field(gc);
        self.draw_drop_target(gc);
    }
}
