use crate::engine::{
    check::is_check,
    chess_moves::{get_current_player_moves, ChessMove},
    evaluation::Evaluation,
    piece::Color,
    position::Position,
    search_algorithms::{Player, MAX_VALUE, MIN_VALUE},
};

#[allow(dead_code)]
pub fn minimax(position: &Position, player: Player, depth: u8) -> (Option<ChessMove>, f32) {
    let mut best_move: Option<ChessMove> = None;
    let children = get_children(position);
    if depth == 0 || children.is_empty() {
        (None, evaluate(position))
    } else {
        match player {
            Player::Max => {
                let mut best_value = MIN_VALUE;
                for child in children {
                    let (_, child_value) = minimax(&child.position, Player::Min, depth - 1);
                    if child_value >= best_value {
                        best_value = child_value;
                        best_move = Some(child);
                    }
                }
                (best_move, best_value)
            }
            Player::Min => {
                let mut best_value = MAX_VALUE;
                for child in children {
                    let (_, child_value) = minimax(&child.position, Player::Max, depth - 1);
                    if child_value <= best_value {
                        best_value = child_value;
                        best_move = Some(child);
                    }
                }
                (best_move, best_value)
            }
        }
    }
}

pub fn evaluate(position: &Position) -> f32 {
    if get_current_player_moves(position).is_empty() {
        if is_check(position, position.get_player()) {
            match position.get_player() {
                Color::Black => MAX_VALUE,
                Color::White => MIN_VALUE,
            }
        } else {
            0.0
        }
    } else {
        let evaluation = Evaluation::new(position);
        evaluation.score
    }
}

pub fn get_children(position: &Position) -> Vec<ChessMove> {
    get_current_player_moves(position)
}
