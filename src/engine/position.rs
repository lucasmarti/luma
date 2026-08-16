use boards::Boards;
mod boards;
mod castling_rights;
pub(super) use castling_rights::CastlingRights;
mod move_api;
pub(super) use move_api::*;
mod occupancy;
pub(super) mod print;
mod starting_config;

use crate::engine::{
    bitboard::Bitboard,
    movegen::Square,
    piece::{Color, Typ, *},
    position::{occupancy::Occupancy, starting_config::STARTING_CONFIG},
};
use std::hash::{Hash, Hasher};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Position {
    boards: Boards,
    occupied: Occupancy,
    all: Bitboard,
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    player: Color,
}

impl Position {
    pub fn starting() -> Position {
        let mut position = Position::default();
        for config in STARTING_CONFIG.iter() {
            position.put_piece(config.0, config.1);
        }
        position
    }

    pub fn has_castling_rights(&self, rights: CastlingRights) -> bool {
        self.castling_rights.has(rights)
    }

    pub fn remove_castling_right(&mut self, rights: CastlingRights) {
        self.castling_rights.remove(rights);
    }
    pub fn is_occupied(&self, square: Square) -> bool {
        self.all.contains(square)
    }

    pub fn is_occupied_by_color(&self, square: Square, color: Color) -> bool {
        self.occupied[color].contains(square)
    }
    pub fn is_occupied_by_piece(&self, square: Square, piece: Piece) -> bool {
        self.boards[(piece.color, piece.typ)].contains(square)
    }
    pub fn is_occupied_by(&self, square: Square, color: Color, typ: Typ) -> bool {
        self.boards[(color, typ)].contains(square)
    }

    pub fn count_pieces(&self, color: Color, typ: Typ) -> u32 {
        self.boards[(color, typ)].count_ones()
    }

    pub fn get_king_square(&self, color: Color) -> Square {
        let king = self.boards[(color, Typ::King)];
        assert_eq!(
            king.count_ones(),
            1,
            "Expected exactly one {:?} king",
            color
        );

        king.iter().next().unwrap()
    }

    pub fn set_en_passant(&mut self, square: Option<Square>) {
        self.en_passant = square;
    }

    pub fn get_player(&self) -> Color {
        self.player
    }

    pub fn get_en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    pub fn toggle_player(&mut self) {
        self.player = self.player.get_opponent_color();
    }

    pub fn get_all_pieces(&self) -> Vec<(Square, Piece)> {
        let mut all_pieces: Vec<(Square, Piece)> = Vec::new();
        for square in self.all.iter() {
            if let Some(piece) = self.get_piece_at(square) {
                all_pieces.push((square, piece));
            }
        }
        all_pieces
    }

    pub fn get_squares(&self, color: Color, typ: Typ) -> Bitboard {
        self.boards[(color, typ)]
    }

    pub fn put_piece(&mut self, piece: Piece, square: Square) {
        let bit = Bitboard::from(square);

        self.boards[(piece.color, piece.typ)] |= bit;
        self.occupied[piece.color] |= bit;
        self.all |= bit;

        #[cfg(debug_assertions)]
        self.debug_assert_valid();
    }

    pub fn with_piece(mut self, piece: Piece, square: Square) -> Position {
        self.put_piece(piece, square);
        self
    }

    pub fn with_en_passant(mut self, square: Option<Square>) -> Position {
        self.set_en_passant(square);
        self
    }

    pub fn remove_piece(&mut self, square: Square) {
        let mask = !Bitboard::from(square);
        for color in Color::iter() {
            for typ in Typ::iter() {
                self.boards[(color, typ)] &= mask;
            }

            self.occupied[color] &= mask;
        }

        self.all &= mask;

        #[cfg(debug_assertions)]
        self.debug_assert_valid();
    }

    #[cfg(debug_assertions)]
    fn debug_assert_valid(&self) {
        let mut all = Bitboard::default();

        for color in Color::iter() {
            let mut occupied = Bitboard::default();

            for typ in Typ::iter() {
                occupied |= self.boards[(color, typ)];
            }

            debug_assert_eq!(
                occupied, self.occupied[color],
                "occupied cache incorrect for {:?}",
                color,
            );

            all |= occupied;
        }

        debug_assert_eq!(all, self.all, "all cache incorrect");
    }
    pub fn get_piece_at(&self, square: Square) -> Option<Piece> {
        for color in Color::iter() {
            for typ in Typ::iter() {
                if self.boards[(color, typ)].contains(square) {
                    return Some(Piece::new(color, typ));
                }
            }
        }
        None
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            boards: Default::default(),
            occupied: Default::default(),
            all: Default::default(),
            castling_rights: CastlingRights::all(),
            en_passant: None,
            player: Color::White,
        }
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.boards.hash(state);
        self.castling_rights.hash(state);
        self.en_passant.hash(state);
        self.player.hash(state);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.boards == other.boards
            && self.castling_rights == other.castling_rights
            && self.en_passant == other.en_passant
            && self.player == other.player
    }
}

#[cfg(test)]
mod tests;
