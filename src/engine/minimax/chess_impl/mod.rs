use crate::engine::{
    check::is_check,
    chess_moves::get_current_player_moves,
    get_next_move,
    heuristic::heuristic,
    minimax::{evaluate, Minimax, Player},
    position::{self, print::Print, Position},
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
        self.children.len() == 0
    }

    fn get_children(&self) -> &Vec<Self>
    where
        Self: Sized,
    {
        &self.children
    }
}
pub fn get_best_move(position: Position) -> Option<Position> {
    let depth = 4;
    let tree = build_tree(position, depth);
    let minimx_player = match tree.position.get_player() {
        crate::engine::piece::Color::Black => Player::MIN,
        crate::engine::piece::Color::White => Player::MAX,
    };
    let result = evaluate(&tree, minimx_player, depth);
    if let Some(node) = result.0 {
        //node.position.print_board();
        println!("Evaluation best Score = {:?}", result.1);
    }
    match result.0 {
        Some(node) => Some(node.position),
        None => None,
    }
}
fn build_tree(position: Position, depth: u8) -> Node {
    let mut node = Node {
        position: position,
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
