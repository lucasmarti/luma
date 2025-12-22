use crate::{
    engine::directions::squares::Square,
    gui::{configuration::FIELD_SIZE, ui_element::CanvasCoordinate},
};

fn get_field_from_canvas(canvas: f64) -> Option<u32> {
    let field: i64 = ((canvas / FIELD_SIZE as f64).floor() as i64) + 1;
    if field > 0 {
        Some(field as u32)
    } else {
        None
    }
}
pub fn get_canvas_from_square(square: Square) -> CanvasCoordinate {
    let index = square.as_index();
    if index > 63 {
        panic!("ungültiger Index {:?}", index);
    }
    let pos_x: f32 = (index % 8) as f32 * FIELD_SIZE;
    let pos_y: f32 = ((index - (index % 8)) / 8) as f32 * FIELD_SIZE;
    CanvasCoordinate { x: pos_x, y: pos_y }
}

pub fn get_square_from_canvas(location: (f64, f64)) -> Option<Square> {
    let (canvas_x, canvas_y) = location;
    if let (Some(column), Some(row)) = (
        get_field_from_canvas(canvas_x),
        get_field_from_canvas(canvas_y),
    ) {
        return Square::new((row - 1) * 8 + (column - 1));
    }
    None
}
