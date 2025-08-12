use crate::engine::piece::{
    Piece, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
    WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};
use crate::engine::position::bitboard::Bitboard;
use crate::engine::position::Position;
use crate::gui::coordinate_mapper::get_index_from_canvas;

use super::chess_board_model::ChessBoardModel;
use super::coordinate_mapper;
use super::draw_functions::{DrawFunctions, BACKGROUND_COLOR};
use flo_canvas::*;

pub const FIELD_SIZE: f32 = 100.0;

#[derive(Debug, Clone, Copy)]
pub struct CanvasCoordinate {
    pub x: f32,
    pub y: f32,
}
#[derive(Clone)]
pub struct ChessBoardCanvas {
    canvas: DrawingTarget,
    model: ChessBoardModel,
}

impl ChessBoardCanvas {
    pub fn new(canvas: DrawingTarget) -> ChessBoardCanvas {
        ChessBoardCanvas {
            canvas,
            model: ChessBoardModel::new(),
        }
    }

    pub fn handle_click_event(&mut self, location_in_canvas: Option<(f64, f64)>) {
        if let Some(location_in_canvas) = location_in_canvas {
            if let Some(coordinate) = get_index_from_canvas(location_in_canvas) {
                self.model.handle_click_event(coordinate);
                self.draw();
            }
        }
    }

    pub fn draw(&mut self) {
        self.canvas.draw(|mut gc| {
            gc.clear_canvas(BACKGROUND_COLOR);
            gc.canvas_height(8.0 * FIELD_SIZE);
            gc.center_region(0.0, 0.0, 8.0 * FIELD_SIZE, 8.0 * FIELD_SIZE);
            gc.draw_board();
            match self.model.selected_square {
                Some(square) => {
                    gc.draw_selected_field(coordinate_mapper::get_canvas_from_index(square));
                }
                None => {}
            }
            for (coordinate, piece) in self::get_all_pieces(&self.model.position) {
                gc.draw_piece(coordinate, piece);
            }
            for target in &self.model.drop_targets {
                gc.draw_drop_target(coordinate_mapper::get_canvas_from_index(*target.0));
            }
        });
    }
}

pub fn get_all_pieces(position: &Position) -> Vec<(CanvasCoordinate, Piece)> {
    let mut vec: Vec<(CanvasCoordinate, Piece)> = Vec::new();

    vec.append(&mut get_pieces(
        position.get_squares(WHITE_BISHOP),
        WHITE_BISHOP,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(WHITE_KING),
        WHITE_KING,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(WHITE_QUEEN),
        WHITE_QUEEN,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(WHITE_KNIGHT),
        WHITE_KNIGHT,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(WHITE_PAWN),
        WHITE_PAWN,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(WHITE_ROOK),
        WHITE_ROOK,
    ));

    vec.append(&mut get_pieces(
        position.get_squares(BLACK_BISHOP),
        BLACK_BISHOP,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(BLACK_KING),
        BLACK_KING,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(BLACK_QUEEN),
        BLACK_QUEEN,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(BLACK_KNIGHT),
        BLACK_KNIGHT,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(BLACK_PAWN),
        BLACK_PAWN,
    ));
    vec.append(&mut get_pieces(
        position.get_squares(BLACK_ROOK),
        BLACK_ROOK,
    ));

    vec
}

fn get_pieces(bitboard: Bitboard, piece: Piece) -> Vec<(CanvasCoordinate, Piece)> {
    let mut vec: Vec<(CanvasCoordinate, Piece)> = Vec::new();
    for index in bitboard.iter() {
        let coordinate = coordinate_mapper::get_canvas_from_index(index);
        vec.push((coordinate, piece));
    }
    vec
}
