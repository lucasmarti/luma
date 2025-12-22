use flo_canvas::Draw;

use crate::engine::{
    directions::squares::Square,
    piece::{Color, Piece},
};

#[derive(Debug, Clone, Copy)]
pub struct CanvasCoordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UIEvent {
    NewGameAs(Color),
    PromoteTo(Piece),
    Square(Square),
    TurnBoard,
}

pub trait UIElement {
    fn dispatch_event(&self, canvas_coordinate: CanvasCoordinate) -> Option<UIEvent>;
    fn draw(&self, gc: &mut Vec<Draw>);
}
