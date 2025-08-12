use crate::gui::{
    algebraic_notation::{Column, Coordinate, Row},
    chess_board_canvas::{self, CanvasCoordinate},
};

fn get_field_from_canvas(canvas: f64) -> Option<u32> {
    let field: i64 = ((canvas / chess_board_canvas::FIELD_SIZE as f64).floor() as i64) + 1;
    if field >= 0 {
        Some(field as u32)
    } else {
        None
    }
}
pub fn get_canvas_from_index(index: u32) -> CanvasCoordinate {
    if (index < 0 || index > 63) {
        panic!("ungültiger Index {:?}", index);
    }
    let pos_x: f32 = (index % 8) as f32 * chess_board_canvas::FIELD_SIZE;
    let pos_y: f32 = ((index - (index % 8)) / 8) as f32 * chess_board_canvas::FIELD_SIZE;
    return CanvasCoordinate { x: pos_x, y: pos_y };
}

pub fn get_index_from_canvas(location: (f64, f64)) -> Option<u32> {
    let (canvas_x, canvas_y) = location;
    if let (Some(column), Some(row)) = (
        get_field_from_canvas(canvas_x),
        get_field_from_canvas(canvas_y),
    ) {
        return Some(((row - 1) * 8) + (column - 1));
    }
    None
}
/*
pub fn to_algebraic_notation(location: (f64, f64)) -> Option<Coordinate> {
    let (canvas_x, canvas_y) = location;

    let column = get_column_from_canvas(canvas_x);
    let row = get_row_from_canvas(canvas_y);

    if let Some(column) = column {
        if let Some(row) = row {
            return Some(Coordinate { row, column });
        }
    }
    None
}

fn get_row_from_canvas(canvas_y: f64) -> Option<Row> {
    if let Some(field_y) = get_index_from_canvas(canvas_y) {
        Row::from(field_y)
    } else {
        None
    }
}

fn get_column_from_canvas(canvas_x: f64) -> Option<Column> {
    if let Some(field_x) = get_index_from_canvas(canvas_x) {
        Column::from(field_x)
    } else {
        None
    }
}

pub fn to_canvas(coordinate: &Coordinate) -> CanvasCoordinate {
    let column: u32;
    match coordinate.column {
        Column::A => column = 1,
        Column::B => column = 2,
        Column::C => column = 3,
        Column::D => column = 4,
        Column::E => column = 5,
        Column::F => column = 6,
        Column::G => column = 7,
        Column::H => column = 8,
    }
    let pos_x: f32 = (column - 1) as f32 * chess_board_canvas::FIELD_SIZE;

    let row: u32;
    match coordinate.row {
        Row::_1 => row = 1,
        Row::_2 => row = 2,
        Row::_3 => row = 3,
        Row::_4 => row = 4,
        Row::_5 => row = 5,
        Row::_6 => row = 6,
        Row::_7 => row = 7,
        Row::_8 => row = 8,
    }
    let pos_y: f32 = (row - 1) as f32 * chess_board_canvas::FIELD_SIZE;
    return CanvasCoordinate { x: pos_x, y: pos_y };
}
    */
