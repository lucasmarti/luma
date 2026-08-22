use std::{assert_eq, println};
#[cfg(test)]
mod tests;

use crate::engine::{
    chess_move,
    position::{self, print::Print, CastlingRights},
    ChessMove, Color, MoveType, Piece, Position, Square, Typ,
};

impl Position {
    pub fn make_move(&mut self, mve: Mve) -> Undo {
        let mut undo = Undo {
            en_passant: self.en_passant,
            castling_rights: self.castling_rights,
            capture: self.get_piece_at(mve.to),
            player: self.player,
        };

        self.en_passant = None;

        self.remove_castling_right(CastlingRights::from(mve.from));
        if let Some(capture) = undo.capture {
            if capture.typ == Typ::Rook {
                self.remove_castling_right(CastlingRights::from(mve.to));
            }
        }
        match mve.move_type {
            MoveType::DoublePawnPush(en_passant) => {
                self.remove_piece(mve.from);
                self.put_piece(mve.piece, mve.to);
                self.en_passant = Some(en_passant);
            }
            MoveType::Quiet => {
                self.remove_piece(mve.from);
                self.put_piece(mve.piece, mve.to);
            }
            MoveType::Capture => {
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
            MoveType::Quiet => {
                self.remove_piece(mve.to);
                self.put_piece(mve.piece, mve.from);
            }
            MoveType::DoublePawnPush(_) => {
                self.remove_piece(mve.to);
                self.put_piece(mve.piece, mve.from);
            }
            MoveType::Capture => {
                self.remove_piece(mve.to);
                self.put_piece(mve.piece, mve.from);

                let capture = undo
                    .capture
                    .expect("MoveType::Capture muss ein Capture haben");
                self.put_piece(capture, mve.to);
            }

            MoveType::Castling(castling_type) => {
                let castling = castling_type.config();

                self.remove_piece(castling.rook_to);
                self.put_piece(castling.rook, castling.rook_from);

                self.remove_piece(mve.to);
                self.put_piece(mve.piece, mve.from);
            }

            MoveType::Promotion(_) | MoveType::PromotionCapture(_) => {
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

pub fn test_make_unmake(position: &Position, chess_move: &ChessMove) {
    let mut mum_position = *position;
    let mve = chess_move.mve;

    let undo = mum_position.make_move(mve);
    assert_eq!(mum_position, chess_move.position, "make missmatch");
    mum_position.unmake(mve, undo);
    assert_eq!(mum_position, *position, "unmake missmatch");
}
pub struct Undo {
    en_passant: Option<Square>,
    castling_rights: CastlingRights,
    capture: Option<Piece>,
    player: Color,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Mve {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub move_type: MoveType,
}
