const MAX_VALUE: i32 = std::i32::MAX;
const MIN_VALUE: i32 = std::i32::MIN;
const DEPTH: u8 = 2;

pub enum Player {
    MIN,
    MAX,
}
pub trait Minimax {
    fn evaluate(&self) -> i32;
    fn is_game_over(&self) -> bool;
    fn get_children(&self) -> &Vec<Self>
    where
        Self: Sized;
}

pub fn evaluate<P: Minimax>(position: &P, player: Player) -> (Option<&P>, i32) {
    let mut best_position: Option<&P> = None;
    let mut alpha: i32 = MIN_VALUE;
    let mut beta: i32 = MAX_VALUE;
    let depth = DEPTH;

    match player {
        Player::MAX => {
            let mut best_value = MIN_VALUE;
            if depth == 0 || position.is_game_over() {
                return (None, position.evaluate());
            } else {
                for child in position.get_children() {
                    let child_value = minimise(child, depth, alpha, beta);
                    if child_value >= best_value {
                        best_value = child_value;
                        best_position = Some(child);
                    }
                    alpha = max(alpha, child_value);
                    if beta <= alpha {
                        break;
                    }
                }
                return (best_position, best_value);
            }
        }
        Player::MIN => {
            if depth == 0 || position.is_game_over() {
                return (None, position.evaluate());
            } else {
                let mut best_value = MAX_VALUE;
                for child in position.get_children() {
                    let child_value = maximise(child, depth, alpha, beta);
                    if child_value <= best_value {
                        best_value = child_value;
                        best_position = Some(child);
                    }
                    beta = min(beta, child_value);
                    if beta <= alpha {
                        break;
                    }
                }
                return (best_position, best_value);
            }
        }
    }
}

fn minimise<P: Minimax>(position: &P, depth: u8, alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 || position.is_game_over() {
        return position.evaluate();
    } else {
        let mut best_value = MAX_VALUE;
        for child in position.get_children() {
            let child_value = maximise(child, depth - 1, alpha, beta);
            best_value = min(best_value, child_value);
            beta = min(beta, child_value);
            if beta <= alpha {
                break;
            }
        }
        return best_value;
    }
}

fn maximise<P: Minimax>(position: &P, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    let mut best_value = MIN_VALUE;
    if depth == 0 || position.is_game_over() {
        best_value = position.evaluate();
    } else {
        for child in position.get_children() {
            let child_value = minimise(child, depth - 1, alpha, beta);
            best_value = max(best_value, child_value);
            alpha = max(alpha, child_value);
            if beta <= alpha {
                break;
            }
        }
    }
    best_value
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests;
