use crate::engine::search_algorithms::{node::Node, Player, MAX_VALUE, MIN_VALUE};

pub struct AlphaBetaResult<N: Node + Clone> {
    pub value: f32,
    pub leaf: Option<N>,
    pub best_node: Option<N>,
}
pub fn alpha_beta<N: Node + Clone>(
    node: N,
    player: Player,
    mut alpha: f32,
    mut beta: f32,
) -> AlphaBetaResult<N> {
    if node.is_leaf() {
        AlphaBetaResult {
            value: node.evaluate(),
            leaf: Some(node),
            best_node: None,
        }
        //(None, node.evaluate())
    } else {
        match player {
            Player::Max => {
                let mut max_value = MIN_VALUE;
                let mut max_node: Option<N> = None;
                let mut leaf: Option<N> = None;
                for child in node.get_children() {
                    let alpha_beta_result = alpha_beta(child.clone(), Player::Min, alpha, beta);
                    if alpha_beta_result.value > max_value {
                        max_value = alpha_beta_result.value;
                        max_node = Some(child);
                        leaf = alpha_beta_result.leaf;
                    }
                    alpha = max(alpha, max_value);
                    if beta <= alpha {
                        break;
                    }
                }
                AlphaBetaResult {
                    value: max_value,
                    leaf: leaf,
                    best_node: max_node,
                }
            }
            Player::Min => {
                let mut min_value = MAX_VALUE;
                let mut min_node: Option<N> = None;
                let mut leaf: Option<N> = None;
                for child in node.get_children() {
                    let alpha_beta_result = alpha_beta(child.clone(), Player::Max, alpha, beta);
                    if alpha_beta_result.value < min_value {
                        min_value = alpha_beta_result.value;
                        min_node = Some(child);
                        leaf = alpha_beta_result.leaf;
                    }
                    beta = min(beta, min_value);
                    if beta <= alpha {
                        break;
                    }
                }
                AlphaBetaResult {
                    value: min_value,
                    leaf: leaf,
                    best_node: min_node,
                }
            }
        }
    }
}

fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
