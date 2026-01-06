use std::sync::Mutex;

use crate::engine::{
    chess_moves::get_current_player_moves,
    evaluation::Evaluation,
    position::{print::Print, Position},
    search_algorithms::{alpha_beta::alpha_beta, node::ChessNode},
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

pub fn get_best_move(position: Position) -> Option<Position> {
    let depth = 4;
    let tree = build_tree(position, depth);
    let minimx_player = match tree.position.get_player() {
        crate::engine::piece::Color::Black => Player::Min,
        crate::engine::piece::Color::White => Player::Max,
    };
    let alpha_beta_result = alpha_beta(tree, minimx_player, MIN_VALUE, MAX_VALUE);
    if let Some(leaf) = alpha_beta_result.leaf.clone().map(|node| node.position) {
        leaf.print_board();
        println!("{:?}", Evaluation::new(&leaf));
    }
    alpha_beta_result.best_node.map(|node| node.position)
}

fn build_tree(position: Position, depth: u8) -> ChessNode {
    let mut node = ChessNode {
        position,
        children: Vec::new(),
    };
    if depth > 0 {
        for child in get_current_player_moves(&position) {
            node.children.push(build_tree(child, depth - 1));
        }
    }
    node
}
pub mod alpha_beta;
pub mod minimax;
pub mod node;
mod tests;
