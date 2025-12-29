use crate::engine::search_algorithms::{node::Node, Player, MAX_VALUE, MIN_VALUE};

pub fn evaluate<P: Node>(position: &P, player: Player, depth: u8) -> (Option<&P>, f32) {
    let mut best_position: Option<&P> = None;
    match player {
        Player::Max => {
            let mut best_value = MIN_VALUE;
            if depth == 0 || position.is_game_over() {
                (None, position.evaluate())
            } else {
                for child in position.get_children() {
                    let child_value = minimise(child, depth - 1);
                    if child_value >= best_value {
                        best_value = child_value;
                        best_position = Some(child);
                    }
                }
                (best_position, best_value)
            }
        }
        Player::Min => {
            if depth == 0 || position.is_game_over() {
                (None, position.evaluate())
            } else {
                let mut best_value = MAX_VALUE;
                for child in position.get_children() {
                    let child_value = maximise(child, depth - 1);
                    if child_value <= best_value {
                        best_value = child_value;
                        best_position = Some(child);
                    }
                }
                (best_position, best_value)
            }
        }
    }
}

fn minimise<P: Node>(position: &P, depth: u8) -> f32 {
    if depth == 0 || position.is_game_over() {
        position.evaluate()
    } else {
        let mut best_value = MAX_VALUE;
        for child in position.get_children() {
            let child_value = maximise(child, depth - 1);
            best_value = min(best_value, child_value);
        }
        best_value
    }
}

fn maximise<P: Node>(position: &P, depth: u8) -> f32 {
    let mut best_value = MIN_VALUE;
    if depth == 0 || position.is_game_over() {
        best_value = position.evaluate();
    } else {
        for child in position.get_children() {
            let child_value = minimise(child, depth - 1);
            best_value = max(best_value, child_value);
        }
    }
    best_value
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
