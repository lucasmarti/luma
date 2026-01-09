use std::sync::Mutex;

use crate::engine::{
    cache::Cache, chess_moves::ChessMove, position::Position,
    search_algorithms::alpha_beta::alpha_beta,
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

pub fn get_best_move(position: Position) -> Option<ChessMove> {
    let cache = &mut Cache::new();
    let depth = 4;
    //let tree = build_tree(position, depth);
    let minimx_player = match position.get_player() {
        crate::engine::piece::Color::Black => Player::Min,
        crate::engine::piece::Color::White => Player::Max,
    };
    let best_move =
        alpha_beta(&position, minimx_player, MIN_VALUE, MAX_VALUE, depth, cache).best_move;
    let mut hits = 0;
    for evaluation in cache.values() {
        hits += evaluation.hits;
    }
    println!("Cache size = {:?}", cache.len());
    println!("Number of hits = {:?}", hits);
    best_move
}
pub mod alpha_beta;
pub mod minimax;
pub mod node;
mod tests;
