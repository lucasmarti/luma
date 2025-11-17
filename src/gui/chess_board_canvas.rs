use crate::engine::directions::squares::Square;
use crate::engine::piece::{self, *};
use crate::engine::position::bitboard::Bitboard;
use crate::engine::position::Position;
use crate::engine::{self};
use crate::gui::coordinate_mapper::get_square_from_canvas;
use crate::gui::ui_element::UIComponents;
use crate::gui::ui_menu::{self, UIMenu};
use crate::gui::{configuration::*, ui_board};

use super::coordinate_mapper;
use super::draw_functions::DrawFunctions;
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
    selected_square: Option<Square>,
    computer: piece::Color,
    check_square: Option<Square>,
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
        self.computer = piece::Color::Black;
        self.drop_targets.clear();
        self.selected_square = None;
        self.check_square = None;
        self.position = Position::new_starting_position();
        self.draw();
    }
    fn new_game_black(&mut self) {
        self.computer = piece::Color::White;
        self.drop_targets.clear();
        self.selected_square = None;
        self.check_square = None;
        if let Some(position) = engine::get_next_move(&Position::new_starting_position()) {
            self.position = position;
        }
        self.draw();
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
            if let Some(action) = ui_menu::get_action(location_in_canvas.0, location_in_canvas.1) {
                match action {
                    Action::NewGameAsWhite => self.new_game_white(),
                    Action::NewGameAsBlack => self.new_game_black(),
                    Action::PromoteWhiteQueen => self.promote(Piece::WhiteQueen),
                    Action::PromoteWhiteRook => self.promote(Piece::WhiteRook),
                    Action::PromoteWhiteBishop => self.promote(Piece::WhiteBishop),
                    Action::PromoteWhiteKnight => self.promote(Piece::WhiteKnight),
                    Action::PromoteBlackQueen => self.promote(Piece::BlackQueen),
                    Action::PromoteBlackRook => self.promote(Piece::BlackRook),
                    Action::PromoteBlackBishop => self.promote(Piece::BlackBishop),
                    Action::PromoteBlackKnight => self.promote(Piece::BlackKnight),
                }
                return;
            }

            if let Some(coordinate) = get_square_from_canvas(location_in_canvas) {
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

    fn drag_n_drop(&mut self, square: Square) -> Option<Position> {
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
        let ui_components: UIComponents = UIComponents::new();
        self.canvas.draw(|mut gc| {
            gc.clear_canvas(BACKGROUND_COLOR);
            gc.canvas_height(8.0 * FIELD_SIZE + 2.0 * MENU_HEIGHT);
            gc.center_region(
                0.0,
                0.0,
                8.0 * FIELD_SIZE,
                8.0 * FIELD_SIZE + 2.0 * MENU_HEIGHT,
            );
            ui_components.draw(gc);

            if let Some(square) = self.selected_square {
                gc.draw_selected_field(coordinate_mapper::get_canvas_from_square(square));
            }
            if let Some(square) = self.check_square {
                gc.draw_check_field(coordinate_mapper::get_canvas_from_square(square));
            }

            for (coordinate, piece) in self::get_all_pieces(&self.position) {
                gc.draw_piece(coordinate, piece, FIELD_SIZE);
            }
            if let Some(square) = self.position.get_from_square() {
                gc.draw_from_to_field(coordinate_mapper::get_canvas_from_square(square));
            }
            if let Some(square) = self.position.get_to_square() {
                gc.draw_from_to_field(coordinate_mapper::get_canvas_from_square(square));
            }
            for target in &self.drop_targets {
                if let Some(square) = target.get_to_square() {
                    gc.draw_drop_target(coordinate_mapper::get_canvas_from_square(square));
                }
            }
        });
    }
}
