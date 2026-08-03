use crate::engine::{
    evaluation::Evaluation,
    movegen::check::is_check,
    movegen::chess_move::{get_current_player_moves, ChessMove},
    piece::Color,
    position::Position,
    search_algorithms::{cache::Cache, MAX_VALUE, MIN_VALUE},
};

pub fn evaluate(position: &Position, cache: &mut Cache) -> f32 {
    if get_current_player_moves(position).is_empty() {
        if is_check(position, position.get_player()) {
            match position.get_player() {
                Color::Black => MAX_VALUE,
                Color::White => MIN_VALUE,
            }
        } else {
            0.0
        }
    } else if let Some(evaluation) = cache.get_mut(position) {
        evaluation.hits += 1;
        evaluation.score
    } else {
        let evaluation = Evaluation::new(position);
        cache.insert(*position, evaluation);
        evaluation.score
    }
}

pub fn get_children(position: &Position) -> Vec<ChessMove> {
    get_current_player_moves(position)
}
