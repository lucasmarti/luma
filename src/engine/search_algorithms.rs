use crate::engine::{
    chess_moves::get_current_player_moves, position::Position, search_algorithms::node::ChessNode,
};

pub const MAX_VALUE: f32 = f32::MAX;
pub const MIN_VALUE: f32 = f32::MIN;

#[derive(Debug)]
pub enum Player {
    #[allow(unused)]
    Min,
    Max,
}

pub fn get_best_move(position: Position) -> Option<Position> {
    let depth = 2;
    let tree = build_tree(position, depth);
    let minimx_player = match tree.position.get_player() {
        crate::engine::piece::Color::Black => Player::Min,
        crate::engine::piece::Color::White => Player::Max,
    };
    let result = alpha_beta::evaluate(&tree, minimx_player, depth);
    result.0.map(|node| node.position)
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
