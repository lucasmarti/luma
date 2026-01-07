use crate::engine::{
    cache::Cache,
    search_algorithms::{node::Node, Player, MAX_VALUE, MIN_VALUE},
};

#[allow(dead_code)]
pub fn minimax<N: Node + Clone>(node: N, player: Player, cache: &mut Cache) -> (Option<N>, f32) {
    let mut best_node: Option<N> = None;
    if node.is_leaf() {
        (None, node.evaluate(cache))
    } else {
        match player {
            Player::Max => {
                let mut best_value = MIN_VALUE;
                for child in node.get_children() {
                    let (_, child_value) = minimax(child.clone(), Player::Min, cache);
                    if child_value >= best_value {
                        best_value = child_value;
                        best_node = Some(child);
                    }
                }
                (best_node, best_value)
            }
            Player::Min => {
                let mut best_value = MAX_VALUE;
                for child in node.get_children() {
                    let (_, child_value) = minimax(child.clone(), Player::Max, cache);
                    if child_value <= best_value {
                        best_value = child_value;
                        best_node = Some(child);
                    }
                }
                (best_node, best_value)
            }
        }
    }
}
