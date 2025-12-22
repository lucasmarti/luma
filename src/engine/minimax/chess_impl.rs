use crate::engine::{
    chess_moves::get_current_player_moves,
    heuristic::heuristic,
    minimax::{evaluate, Minimax, Player},
    position::Position,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    pub position: Position,
    pub children: Vec<Node>,
}

impl Minimax for Node {
    fn evaluate(&self) -> f32 {
        heuristic(&self.position)
    }

    fn is_game_over(&self) -> bool {
        self.children.is_empty()
    }

    fn get_children(&self) -> &Vec<Self>
    where
        Self: Sized,
    {
        &self.children
    }
}
pub fn get_best_move(position: Position) -> Option<Position> {
    let depth = 3;
    let tree = build_tree(position, depth);
    let minimx_player = match tree.position.get_player() {
        crate::engine::piece::Color::Black => Player::Min,
        crate::engine::piece::Color::White => Player::Max,
    };
    let result = evaluate(&tree, minimx_player, depth);
    result.0.map(|node| node.position)
}
fn build_tree(position: Position, depth: u8) -> Node {
    let mut node = Node {
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
#[cfg(test)]
mod tests;
