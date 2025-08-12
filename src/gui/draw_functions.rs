use flo_canvas::{Color, GraphicsContext, GraphicsPrimitives};
use std::io::Cursor;

use crate::engine::piece::Piece;

use super::{
    assets,
    chess_board_canvas::{CanvasCoordinate, FIELD_SIZE},
};
pub const BACKGROUND_COLOR: Color = Color::Rgba(0.5, 0.5, 0.5, 1.0);
const BLACK_FIELD_COLOR: Color = Color::Rgba(202.0 / 255.0, 207.0 / 255.0, 184.0 / 255.0, 1.0);
const WHITE_FIELD_COLOR: Color = Color::Rgba(224.0 / 255.0, 218.0 / 255.0, 193.0 / 255.0, 1.0);
const DROP_TARGET_COLOR: Color = Color::Rgba(1.0, 0.0, 0.0, 5.0);
const SELECTED_FIELD_COLOR: Color = Color::Rgba(1.0, 1.0, 1.0, 5.0);

pub trait DrawFunctions {
    fn draw_board(&mut self);
    fn draw_field(&mut self, coordinate: CanvasCoordinate, color: Color);
    fn draw_selected_field(&mut self, coordinate: CanvasCoordinate);
    fn draw_drop_target(&mut self, coordinate: CanvasCoordinate);
    fn draw_piece(&mut self, coordinate: CanvasCoordinate, piece: Piece);
}
impl DrawFunctions for &mut Vec<flo_canvas::Draw> {
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

    fn draw_piece(&mut self, coordinate: CanvasCoordinate, piece: Piece) {
        self.new_path();
        self.rect(
            coordinate.x,
            coordinate.y,
            coordinate.x + FIELD_SIZE,
            coordinate.y + FIELD_SIZE,
        );
        self.load_texture(
            assets::get_texture_id(piece),
            Cursor::new(assets::get_bytes(piece)),
        );
        self.fill_texture(
            assets::get_texture_id(piece),
            0.0,
            FIELD_SIZE,
            FIELD_SIZE,
            0.0,
        );
        self.fill();
    }
}
