use crate::{
    bitboard::Bitboard,
    chess_moves::{ChessMove, Progress},
    directions,
    piece::{Color, Piece, WHITE_QUEEN},
    position::Position,
};



pub fn get_possible_white_moves(position: &Position, index: u32) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    for field in directions::all_up(index) {
        if position.get_white().contains(field) {
            // collision with own
            return moves;
        } else if position.get_black().contains(field) {
            // capture
            moves.push(ChessMove::Progress(Progress {
                from: index,
                to: field,
                piece: WHITE_QUEEN,
            }));
            return moves;
        } else {
            // empty field
            moves.push(ChessMove::Progress(Progress {
                from: index,
                to: field,
                piece: WHITE_QUEEN,
            }));
        }
    }
    moves
}



mod tests;
