use std::collections::HashMap;

use crate::engine::piece::{self, *};
use crate::engine::position::bitboard::Bitboard;
use crate::engine::position::Position;
use crate::engine::{self, position};
use crate::gui::configuration::*;
use crate::gui::coordinate_mapper::get_index_from_canvas;
use crate::gui::menu;

use super::coordinate_mapper;
use super::draw_functions::{DrawFunctions, BACKGROUND_COLOR};
use flo_canvas::*;

#[derive(Debug, Clone, Copy)]
pub struct CanvasCoordinate {
    pub x: f32,
    pub y: f32,
}
#[derive(Clone)]
pub struct ChessBoardCanvas {
    canvas: DrawingTarget,
    position: Position,
    drop_targets: Vec<Position>,
    selected_square: Option<u32>,
    computer: piece::Color,
    check_square: Option<u32>,
    show_white_promotion_buttons: bool,
    show_black_promotion_buttons: bool,
}

impl ChessBoardCanvas {
    pub fn new(canvas: DrawingTarget) -> ChessBoardCanvas {
        ChessBoardCanvas {
            canvas,
            position: Position::new_starting_position(),
            drop_targets: Vec::new(),
            selected_square: None,
            computer: piece::Color::Black,
            check_square: None,
            show_white_promotion_buttons: false,
            show_black_promotion_buttons: false,
        }
    }

    fn new_game_white(&mut self) {
        print!("New game white");
        self.computer = piece::Color::Black;
        self.drop_targets.clear();
        self.selected_square = None;
        self.check_square = None;
        self.position = Position::new_starting_position();
        self.draw();
    }
    fn new_game_black(&mut self) {
        print!("New game black");
        self.computer = piece::Color::White;
        self.drop_targets.clear();
        self.selected_square = None;
        self.check_square = None;
        if let Some(position) = engine::get_next_move(&Position::new_starting_position()) {
            self.position = position;
        }
    }
    fn promote(&mut self, piece: Piece) {
        self.drop_targets
            .retain(|&position| position.get_promotion_piece() == Some(piece));

        if let Some(position) = self.drop_targets.pop() {
            self.position = position;
            self.check_square = engine::get_check(&position);
            self.disable_promotion_buttons();

            self.draw();
            if let Some(computer_position) = engine::get_next_move(&position) {
                self.position = computer_position;
                self.check_square = engine::get_check(&computer_position);
            }
        }
        self.draw();
    }

    pub fn handle_click_event(&mut self, location_in_canvas: Option<(f64, f64)>) {
        if let Some(location_in_canvas) = location_in_canvas {
            if let Some(action) = menu::get_action(location_in_canvas.0, location_in_canvas.1) {
                match action {
                    Action::NewGameAsWhite => self.new_game_white(),
                    Action::NewGameAsBlack => self.new_game_black(),
                    Action::PromoteWhiteQueen => self.promote(WHITE_QUEEN),
                    Action::PromoteWhiteRook => self.promote(WHITE_ROOK),
                    Action::PromoteWhiteBishop => self.promote(WHITE_BISHOP),
                    Action::PromoteWhiteKnight => self.promote(WHITE_KNIGHT),
                    Action::PromoteBlackQueen => self.promote(BLACK_QUEEN),
                    Action::PromoteBlackRook => self.promote(BLACK_ROOK),
                    Action::PromoteBlackBishop => self.promote(BLACK_BISHOP),
                    Action::PromoteBlackKnight => self.promote(BLACK_KNIGHT),
                }
                return;
            }

            if let Some(coordinate) = get_index_from_canvas(location_in_canvas) {
                if let Some(position) = self.drag_n_drop(coordinate) {
                    self.position = position;
                    self.check_square = engine::get_check(&position);
                    self.draw();
                    if let Some(computer_position) = engine::get_next_move(&position) {
                        self.position = computer_position;
                        self.check_square = engine::get_check(&computer_position);
                    }
                }
                self.draw();
            }
        }
    }

    fn drag_n_drop(&mut self, square: u32) -> Option<Position> {
        if self.position.get_player() == self.computer {
            return None;
        }
        if engine::is_valid_drag_square(&self.position, square) {
            self.selected_square = Some(square);
            self.drop_targets = engine::get_valid_drop_positions(&self.position, square);
            return None;
        }
        let mut position: Option<Position> = None;
        for target in &self.drop_targets {
            if target.get_to_square() == Some(square) {
                position = Some(target.clone());
            }
        }
        if let Some(position) = position {
            if position.is_promotion() {
                self.enable_promotion_buttons();
                self.drop_targets
                    .retain(|&position| position.get_to_square() == Some(square));
                return None;
            } else {
                self.drop_targets.clear();
                self.selected_square = None;
                return Some(position);
            }
        }
        None
    }
    fn enable_promotion_buttons(&mut self) {
        match self.position.get_player() {
            piece::Color::Black => self.show_black_promotion_buttons = true,
            piece::Color::White => self.show_white_promotion_buttons = true,
        }
    }

    fn disable_promotion_buttons(&mut self) {
        self.show_black_promotion_buttons = false;
        self.show_white_promotion_buttons = false;
    }

    pub fn draw(&mut self) {
        self.canvas.draw(|mut gc| {
            gc.clear_canvas(BACKGROUND_COLOR);
            gc.canvas_height(8.0 * FIELD_SIZE + 2.0 * MENU_HEIGHT);
            gc.center_region(
                0.0,
                0.0,
                8.0 * FIELD_SIZE,
                8.0 * FIELD_SIZE + 2.0 * MENU_HEIGHT,
            );
            gc.draw_board();
            if let Some(square) = self.selected_square {
                gc.draw_selected_field(coordinate_mapper::get_canvas_from_index(square));
            }
            if let Some(square) = self.check_square {
                gc.draw_check_field(coordinate_mapper::get_canvas_from_index(square));
            }

            for (coordinate, piece) in self::get_all_pieces(&self.position) {
                gc.draw_piece(coordinate, piece, FIELD_SIZE);
            }
            if let Some(square) = self.position.get_from_square() {
                gc.draw_from_to_field(coordinate_mapper::get_canvas_from_index(square));
            }
            if let Some(square) = self.position.get_to_square() {
                gc.draw_from_to_field(coordinate_mapper::get_canvas_from_index(square));
            }
            for target in &self.drop_targets {
                if let Some(square) = target.get_to_square() {
                    gc.draw_drop_target(coordinate_mapper::get_canvas_from_index(square));
                }
            }

            gc.draw_menu();
            gc.draw_button(&NEW_GAME_WHITE_BUTTON);
            gc.draw_button(&NEW_GAME_BLACK_BUTTON);
            if self.show_black_promotion_buttons {
                gc.draw_button(&PROMOTION_BLACK_QUEEN_BUTTON);
                gc.draw_button(&PROMOTION_BLACK_ROOK_BUTTON);
                gc.draw_button(&PROMOTION_BLACK_BISHOP_BUTTON);
                gc.draw_button(&PROMOTION_BLACK_KNIGHT_BUTTON);
            }
            if self.show_white_promotion_buttons {
                gc.draw_button(&PROMOTION_WHITE_QUEEN_BUTTON);
                gc.draw_button(&PROMOTION_WHITE_ROOK_BUTTON);
                gc.draw_button(&PROMOTION_WHITE_BISHOP_BUTTON);
                gc.draw_button(&PROMOTION_WHITE_KNIGHT_BUTTON);
            }
        });
    }
}

pub fn get_all_pieces(position: &Position) -> Vec<(CanvasCoordinate, Piece)> {
    let mut vec: Vec<(CanvasCoordinate, Piece)> = Vec::new();
    for piece in ALL_PIECES_SET {
        vec.append(&mut get_pieces(position.get_squares(piece), piece));
    }
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
