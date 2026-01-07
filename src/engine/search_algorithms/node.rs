use crate::engine::{
    cache::Cache,
    check::is_check,
    chess_moves::get_current_player_moves,
    evaluation::{self, Evaluation},
    piece::Color,
    position::Position,
    search_algorithms::{MAX_VALUE, MIN_VALUE},
};

pub trait Node {
    fn evaluate(&self, cache: &mut Cache) -> f32;
    fn is_leaf(&self) -> bool;
    fn get_children(&self) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChessNode {
    pub position: Position,
    pub children: Vec<ChessNode>,
}

impl Node for ChessNode {
    fn evaluate(&self, cache: &mut Cache) -> f32 {
        if get_current_player_moves(&self.position).is_empty() {
            if is_check(&self.position, self.position.get_player()) {
                println!("Is Check");
                match self.position.get_player() {
                    Color::Black => MAX_VALUE,
                    Color::White => MIN_VALUE,
                }
            } else {
                println!("Is Draw");
                0.0
            }
        } else {
            if let Some(evaluation) = cache.get_mut(&self.position) {
                evaluation.hits += 1;
                return evaluation.score;
            } else {
                let evaluation = Evaluation::new(&self.position);
                cache.insert(self.position, evaluation);
                return evaluation.score;
            }
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn get_children(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        self.children.clone()
    }
}
