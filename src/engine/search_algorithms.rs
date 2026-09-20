mod alpha_beta;
mod cache;
mod minimax;
mod node;
use std::sync::Mutex;

use crate::engine::{
    position::Position,
    search_algorithms::{alpha_beta::alpha_beta, cache::Cache},
    Mve,
};
lazy_static::lazy_static! {
    pub static ref CALL_COUNT: Mutex<u64> = Mutex::new(0);
}
pub const MAX_VALUE: f32 = f32::MAX;
pub const MIN_VALUE: f32 = f32::MIN;

#[derive(Debug, Clone, Copy)]
pub enum Player {
    #[allow(unused)]
    Min,
    Max,
}

pub fn search_best_move(position: Position) -> Option<Mve> {
    let cache = &mut Cache::new();
    let depth = 2;
    let minimx_player = match position.get_player() {
        crate::engine::piece::Color::Black => Player::Min,
        crate::engine::piece::Color::White => Player::Max,
    };
    alpha_beta(
        &mut position.clone(),
        minimx_player,
        MIN_VALUE,
        MAX_VALUE,
        depth,
        cache,
    )
    .best_move
}

mod tests;
