use crate::{position::Position, possible_moves::get_all_moves};

mod check;
mod chess_moves;
mod directions;
mod evaluation;
mod minimax;
mod piece;
mod position;
mod possible_moves;

fn main() {
    let position = Position::new_starting_position();
    get_all_moves(&position, piece::Color::White);
}
