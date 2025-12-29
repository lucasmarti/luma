use crate::engine::{
    check::is_check,
    chess_moves::get_current_player_moves,
    heuristic::heuristic,
    piece::Color,
    position::Position,
    search_algorithms::{alpha_beta, Player, MAX_VALUE, MIN_VALUE},
};

pub trait Node {
    fn evaluate(&self) -> f32;
    fn is_game_over(&self) -> bool;
    fn get_children(&self) -> &Vec<Self>
    where
        Self: Sized;
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChessNode {
    pub position: Position,
    pub children: Vec<ChessNode>,
}

impl Node for ChessNode {
    fn evaluate(&self) -> f32 {
        if get_current_player_moves(&self.position).is_empty() {
            if is_check(&self.position, self.position.get_player()) {
                match self.position.get_player() {
                    Color::Black => MAX_VALUE,
                    Color::White => MIN_VALUE,
                }
            } else {
                0.0
            }
        } else {
            heuristic(&self.position).total
        }
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
