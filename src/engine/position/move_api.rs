use crate::engine::{position::CastlingRights, Piece, Position, Square};

impl Position {
    pub fn make_move(&self, from: Square, to: Square, piece: Piece) -> Position {
        let mut new_position = self
            .remove_piece(from)
            .remove_piece(to)
            .put_piece(piece, to)
            .set_en_passant(None)
            .toggle_player();
        new_position.remove_castling_right(CastlingRights::from(from));
        new_position
    }
}
