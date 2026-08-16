use std::assert_eq;
#[cfg(test)]
mod tests;

use crate::engine::{position::CastlingRights, Color, MoveType, Piece, Position, Square, Typ};

impl Position {
    pub fn make_move(&mut self, mve: Mve) -> Undo {
        let mut undo = Undo {
            en_passant: self.en_passant,
            castling_rights: self.castling_rights,
            capture: self.get_piece_at(mve.to),
            player: self.player,
        };

        self.remove_castling_right(CastlingRights::from(mve.from));
        if let Some(capture) = undo.capture {
            if capture.typ == Typ::Rook {
                self.remove_castling_right(CastlingRights::from(mve.to));
            }
        }

        self.en_passant = mve.en_passant;

        match mve.move_type {
            MoveType::Quiet | MoveType::Capture => {
                self.remove_piece(mve.from);
                self.remove_piece(mve.to);
                self.put_piece(mve.piece, mve.to);
            }
            MoveType::Castling(castling_type) => {
                let castling = castling_type.config();
                self.remove_piece(mve.from);
                self.put_piece(mve.piece, mve.to);
                self.remove_piece(castling.rook_from);
                self.put_piece(castling.rook, castling.rook_to);
            }
            MoveType::Promotion(piece) | MoveType::PromotionCapture(piece) => {
                self.remove_piece(mve.from);
                self.remove_piece(mve.to);
                self.put_piece(piece, mve.to);
            }
            MoveType::EnPassant(square) => {
                undo.capture = self.get_piece_at(square);
                self.remove_piece(mve.from);
                self.remove_piece(square);
                self.put_piece(mve.piece, mve.to);
            }
        }
        self.toggle_player();

        undo
    }

    pub fn unmake(&mut self, mve: Mve, undo: Undo) {
        self.en_passant = undo.en_passant;
        self.castling_rights = undo.castling_rights;
        self.player = undo.player;

        match mve.move_type {
            MoveType::Quiet | MoveType::Capture => {
                self.remove_piece(mve.to);
                self.put_piece(mve.piece, mve.from);

                if let Some(capture) = undo.capture {
                    self.put_piece(capture, mve.to);
                }
            }

            MoveType::Castling(castling_type) => {
                let castling = castling_type.config();

                self.remove_piece(castling.rook_to);
                self.put_piece(castling.rook, castling.rook_from);

                self.remove_piece(mve.to);
                self.put_piece(mve.piece, mve.from);
            }

            MoveType::Promotion(piece) | MoveType::PromotionCapture(piece) => {
                self.remove_piece(mve.to);
                self.put_piece(mve.piece, mve.from);

                if let Some(capture) = undo.capture {
                    self.put_piece(capture, mve.to);
                }
            }

            MoveType::EnPassant(square) => {
                let capture = undo.capture.expect("En passant muss einen Capture haben");

                self.remove_piece(mve.to);
                self.put_piece(mve.piece, mve.from);
                self.put_piece(capture, square);
            }
        }
    }
}
pub struct Undo {
    en_passant: Option<Square>,
    castling_rights: CastlingRights,
    capture: Option<Piece>,
    player: Color,
}

#[derive(Clone, Copy)]
pub struct Mve {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub move_type: MoveType,
    pub en_passant: Option<Square>,
}
