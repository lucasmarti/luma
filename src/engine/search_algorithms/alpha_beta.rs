use crate::engine::{
    cache::Cache,
    chess_moves::ChessMove,
    position::Position,
    search_algorithms::{
        node::{evaluate, get_children},
        Player, MAX_VALUE, MIN_VALUE,
    },
};

pub struct AlphaBetaResult {
    pub value: f32,
    pub leaf: Option<Position>,
    pub best_move: Option<ChessMove>,
}
pub fn alpha_beta(
    position: &Position,
    player: Player,
    mut alpha: f32,
    mut beta: f32,
    mut depth: u8,
    cache: &mut Cache,
) -> AlphaBetaResult {
    let mut children = get_children(position);
    if depth == 0 || children.is_empty() {
        AlphaBetaResult {
            value: evaluate(position, cache),
            leaf: Some(*position),
            best_move: None,
        }
    } else {
        match player {
            Player::Max => {
                let mut max_value = MIN_VALUE;
                let mut max_move: Option<ChessMove> = None;
                let mut leaf: Option<Position> = None;
                for child in children {
                    let alpha_beta_result =
                        alpha_beta(&child.position, Player::Min, alpha, beta, depth - 1, cache);
                    if alpha_beta_result.value > max_value {
                        max_value = alpha_beta_result.value;
                        max_move = Some(child);
                        leaf = alpha_beta_result.leaf;
                    }
                    alpha = max(alpha, max_value);
                    if beta <= alpha {
                        break;
                    }
                }
                AlphaBetaResult {
                    value: max_value,
                    leaf,
                    best_move: max_move,
                }
            }
            Player::Min => {
                let mut min_value = MAX_VALUE;
                let mut min_move: Option<ChessMove> = None;
                let mut leaf: Option<Position> = None;
                for child in children {
                    let alpha_beta_result =
                        alpha_beta(&child.position, Player::Max, alpha, beta, depth - 1, cache);
                    if alpha_beta_result.value < min_value {
                        min_value = alpha_beta_result.value;
                        min_move = Some(child);
                        leaf = alpha_beta_result.leaf;
                    }
                    beta = min(beta, min_value);
                    if beta <= alpha {
                        break;
                    }
                }
                AlphaBetaResult {
                    value: min_value,
                    leaf,
                    best_move: min_move,
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
