use crate::engine::position::Position;

struct Score {
    material: f32,
    squares: f32,
    mobility: f32,
}

pub fn heuristic(position: &Position) -> f32 {
    let white_score: Score = Score {
        material: material::count_white(position),
        squares: squares::count_white(position),
        mobility: mobility::count_white(position),
    };

    let black_score: Score = Score {
        material: material::count_black(position),
        squares: squares::count_black(position),
        mobility: mobility::count_black(position),
    };

    white_score.material + white_score.mobility + white_score.squares
        - (black_score.material + black_score.mobility + black_score.squares)
}
mod material;
mod mobility;
mod squares;
#[cfg(test)]
mod tests;
