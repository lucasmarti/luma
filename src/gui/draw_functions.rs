use flo_canvas::{Color, GraphicsContext, GraphicsPrimitives};
use std::io::Cursor;

use super::{icon, chess_board_canvas::CanvasCoordinate};
use crate::gui::configuration::{
    BLACK_FIELD_COLOR, CHECK_COLOR, DROP_TARGET_COLOR, FIELD_SIZE, FROM_TO_COLOR,
    SELECTED_FIELD_COLOR, WHITE_FIELD_COLOR,
};
use crate::{
    engine::piece::Piece,
    gui::configuration::{MENU_HEIGHT, MENU_POS_X, MENU_POS_Y, MENU_WIDTH},
};

pub trait DrawFunctions {
    fn draw_menu(&mut self);
    fn draw_board(&mut self);
    fn draw_field(&mut self, coordinate: CanvasCoordinate, color: Color);
    fn draw_selected_field(&mut self, coordinate: CanvasCoordinate);
    fn draw_drop_target(&mut self, coordinate: CanvasCoordinate);
    fn draw_from_to_field(&mut self, coordinate: CanvasCoordinate);
    fn draw_check_field(&mut self, coordinate: CanvasCoordinate);
    fn draw_piece(&mut self, coordinate: CanvasCoordinate, piece: Piece, size: f32);
}
impl DrawFunctions for &mut Vec<flo_canvas::Draw> {
    fn draw_menu(&mut self) {
        self.new_path();
        self.fill_color(WHITE_FIELD_COLOR);
        self.rect(MENU_POS_X, MENU_POS_Y, MENU_WIDTH, MENU_POS_Y + MENU_HEIGHT);

        self.fill();
    }
    fn draw_board(&mut self) {
        for x in 0..8 {
            for y in 0..8 {
                let coordinate = CanvasCoordinate {
                    x: x as f32 * FIELD_SIZE,
                    y: y as f32 * FIELD_SIZE,
                };
                let modulo = (x + y) % 2;
                if modulo == 1 {
                    self.draw_field(coordinate, WHITE_FIELD_COLOR);
                } else {
                    self.draw_field(coordinate, BLACK_FIELD_COLOR);
                }
            }
        }
    }

    fn draw_field(&mut self, coordinate: CanvasCoordinate, color: Color) {
        self.new_path();
        self.rect(
            coordinate.x,
            coordinate.y,
            coordinate.x + FIELD_SIZE,
            coordinate.y + FIELD_SIZE,
        );
        self.fill_color(color);
        self.fill();
    }

    fn draw_check_field(&mut self, coordinate: CanvasCoordinate) {
        self.new_path();
        self.rect(
            coordinate.x,
            coordinate.y,
            coordinate.x + FIELD_SIZE,
            coordinate.y + FIELD_SIZE,
        );
        self.line_width(2.0);
        self.stroke_color(CHECK_COLOR);
        self.stroke();
    }

    fn draw_from_to_field(&mut self, coordinate: CanvasCoordinate) {
        self.new_path();
        let center_x = coordinate.x + (FIELD_SIZE / 2.0);
        let center_y = coordinate.y + (FIELD_SIZE / 2.0);
        self.circle(center_x, center_y, FIELD_SIZE / 10.0);
        self.line_width(2.0);
        self.stroke_color(FROM_TO_COLOR);
        self.stroke();
    }

    fn draw_selected_field(&mut self, coordinate: CanvasCoordinate) {
        self.new_path();
        self.rect(
            coordinate.x,
            coordinate.y,
            coordinate.x + FIELD_SIZE,
            coordinate.y + FIELD_SIZE,
        );
        self.line_width(2.0);
        self.stroke_color(SELECTED_FIELD_COLOR);
        self.stroke();
    }

    fn draw_drop_target(&mut self, coordinate: CanvasCoordinate) {
        self.new_path();
        let center_x = coordinate.x + (FIELD_SIZE / 2.0);
        let center_y = coordinate.y + (FIELD_SIZE / 2.0);
        self.circle(center_x, center_y, FIELD_SIZE / 10.0);
        self.line_width(2.0);
        self.stroke_color(DROP_TARGET_COLOR);
        self.stroke();
    }

    fn draw_piece(&mut self, coordinate: CanvasCoordinate, piece: Piece, size: f32) {
        self.new_path();
        self.rect(
            coordinate.x + size,
            coordinate.y,
            coordinate.x,
            coordinate.y + size,
        );
        self.load_texture(
            icon::get_texture_id(piece),
            Cursor::new(icon::get_bytes(piece)),
        );
        self.fill_texture(
            icon::get_texture_id(piece),
            coordinate.x,
            coordinate.y + size,
            coordinate.x + size,
            coordinate.y,
        );
        self.fill();
    }
}
