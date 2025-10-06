use crate::engine::position::Position;

struct Score {
    material: f32,
    squares: f32,
    mobility: f32,
}

pub fn heuristic(position: &Position) -> f32 {
    let mut white_score: Score = Score {
        material: 0.0,
        squares: 0.0,
        mobility: 0.0,
    };
    let mut black_score: Score = Score {
        material: 0.0,
        squares: 0.0,
        mobility: 0.0,
    };
    white_score.material = material::count_white(position);
    white_score.squares = squares::count_white(position);
    //white_score.mobility = mobility::count_white(position);
    //black_score.material = count_black(position);
    0.0
}

/*
f(p) = 200(K-K')
       + 9(Q-Q')
       + 5(R-R')
       + 3(B-B' + N-N')
       + 1(P-P')

pub fn heuristic(position: &Position) -> f32 {
    get_score_for_pieces(position, WHITE_PIECES) - get_score_for_pieces(position, BLACK_PIECES)
}
*/

mod material;
mod mobility;
mod squares;
#[cfg(test)]
mod tests;
