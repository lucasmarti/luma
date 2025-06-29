use crate::{
    chess_moves::{ChessMove, Progress},
    directions::{self, left},
    piece::{BLACK_KING, WHITE_KING},
    position::Position,
};

pub fn get_possible_black_moves(position: &Position, index: u32) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    for neighbour in get_neighbours(index) {
        if !position.get_black().contains(neighbour) {
            moves.push(ChessMove::Progress(Progress {
                from: index,
                to: neighbour,
                piece: BLACK_KING,
            }));
        }
    }
    moves
}

pub fn get_possible_white_moves(position: &Position, index: u32) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    for neighbour in get_neighbours(index) {
        if !position.get_white().contains(neighbour) {
            moves.push(ChessMove::Progress(Progress {
                from: index,
                to: neighbour,
                piece: WHITE_KING,
            }));
        }
    }
    moves
}

fn get_neighbours(index: u32) -> Vec<u32> {
    let mut neighbours: Vec<u32> = Vec::new();
    if let Some(neighbour) = directions::left(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::right(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::up(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::down(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::up_left(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::up_right(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::down_left(index) {
        neighbours.push(neighbour);
    }
    if let Some(neighbour) = directions::down_right(index) {
        neighbours.push(neighbour);
    }
    neighbours
}
mod tests;
