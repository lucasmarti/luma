use crate::engine::position::Position;
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Score {
    pub material: f32,
    pub squares: f32,
    pub mobility: f32,
    pub pawn_structure: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Evaluation {
    pub black_score: Score,
    pub white_score: Score,
    pub total: f32,
}

pub fn heuristic(position: &Position) -> Evaluation {
    let white_score: Score = Score {
        material: material::count_white(position),
        squares: squares::count_white(position),
        mobility: mobility::count_white(position),
        pawn_structure: pawn_structures::count_white(position),
    };

    let black_score: Score = Score {
        material: material::count_black(position),
        squares: squares::count_black(position),
        mobility: mobility::count_black(position),
        pawn_structure: pawn_structures::count_black(position),
    };

    let total = white_score.material
        + white_score.mobility
        + white_score.squares
        + white_score.pawn_structure
        - (black_score.material
            + black_score.mobility
            + black_score.squares
            + black_score.pawn_structure);

    let evaluation = Evaluation {
        black_score,
        white_score,
        total,
    };
    evaluation
}
mod material;
mod mobility;
mod pawn_structures;
mod squares;
#[cfg(test)]
mod tests;
