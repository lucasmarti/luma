use crate::engine::{
    directions::squares::*,
    piece::{Color, Typ, *},
    position::bitboard::Bitboard,
};
use std::{
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::EnumCount;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
struct Boards {
    pub boards: [[Bitboard; Typ::COUNT]; Color::COUNT],
}

impl Index<(Color, Typ)> for Boards {
    type Output = Bitboard;

    fn index(&self, (color, typ): (Color, Typ)) -> &Self::Output {
        &self.boards[color.idx()][typ.idx()]
    }
}
impl IndexMut<(Color, Typ)> for Boards {
    fn index_mut(&mut self, (color, typ): (Color, Typ)) -> &mut Self::Output {
        &mut self.boards[color.idx()][typ.idx()]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct CastlingRights {
    pub castling_rights: [bool; CastlingType::COUNT],
}
impl Index<CastlingType> for CastlingRights {
    type Output = bool;

    fn index(&self, castling_type: CastlingType) -> &Self::Output {
        &self.castling_rights[castling_type.idx()]
    }
}
impl IndexMut<CastlingType> for CastlingRights {
    fn index_mut(&mut self, castling_type: CastlingType) -> &mut Self::Output {
        &mut self.castling_rights[castling_type.idx()]
    }
}
impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            castling_rights: [true; CastlingType::COUNT],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
struct Occupancy {
    pub occupied: [Bitboard; Color::COUNT],
}

impl Index<Color> for Occupancy {
    type Output = Bitboard;

    fn index(&self, color: Color) -> &Self::Output {
        &self.occupied[color.idx()]
    }
}
impl IndexMut<Color> for Occupancy {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        &mut self.occupied[color.idx()]
    }
}
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
    pub fn new_starting_position() -> Position {
        Position::default()
            .put_piece(WHITE_KING, E1)
            .put_piece(WHITE_QUEEN, D1)
            .put_piece(WHITE_ROOK, A1)
            .put_piece(WHITE_ROOK, H1)
            .put_piece(WHITE_BISHOP, C1)
            .put_piece(WHITE_BISHOP, F1)
            .put_piece(WHITE_KNIGHT, B1)
            .put_piece(WHITE_KNIGHT, G1)
            .put_piece(WHITE_PAWN, A2)
            .put_piece(WHITE_PAWN, B2)
            .put_piece(WHITE_PAWN, C2)
            .put_piece(WHITE_PAWN, D2)
            .put_piece(WHITE_PAWN, E2)
            .put_piece(WHITE_PAWN, F2)
            .put_piece(WHITE_PAWN, G2)
            .put_piece(WHITE_PAWN, H2)
            .put_piece(BLACK_KING, E8)
            .put_piece(BLACK_QUEEN, D8)
            .put_piece(BLACK_ROOK, A8)
            .put_piece(BLACK_ROOK, H8)
            .put_piece(BLACK_BISHOP, C8)
            .put_piece(BLACK_BISHOP, F8)
            .put_piece(BLACK_KNIGHT, B8)
            .put_piece(BLACK_KNIGHT, G8)
            .put_piece(BLACK_PAWN, A7)
            .put_piece(BLACK_PAWN, B7)
            .put_piece(BLACK_PAWN, C7)
            .put_piece(BLACK_PAWN, D7)
            .put_piece(BLACK_PAWN, E7)
            .put_piece(BLACK_PAWN, F7)
            .put_piece(BLACK_PAWN, G7)
            .put_piece(BLACK_PAWN, H7)
    }

    pub fn disallow_castling_for_color(mut self, color: Color) -> Position {
        match color {
            Color::White => {
                self = self
                    .remove_castling_right(CastlingType::WhiteKingside)
                    .remove_castling_right(CastlingType::WhiteQueenside);
            }
            Color::Black => {
                self = self
                    .remove_castling_right(CastlingType::BlackKingside)
                    .remove_castling_right(CastlingType::BlackQueenside);
            }
        }
        self
    }

    pub fn get_castling_right(&self, castling_type: CastlingType) -> bool {
        self.castling_rights[castling_type]
    }

    pub fn remove_castling_right(mut self, castling_type: CastlingType) -> Position {
        self.castling_rights[castling_type] = false;
        self
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

    pub fn set_en_passant(mut self, square: Square) -> Position {
        assert!(
            A4 <= square && square <= H5,
            "Invalid en passant square {:?}",
            square
        );
        self.en_passant = Some(square);
        self
    }

    pub fn reset_en_passant(mut self) -> Position {
        self.en_passant = None;
        self
    }

    pub fn get_player(&self) -> Color {
        self.player
    }

    pub fn get_en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    pub fn toggle_player(mut self) -> Position {
        match self.player {
            Color::Black => self.player = Color::White,
            Color::White => self.player = Color::Black,
        }
        self
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

    pub fn put_piece(mut self, piece: Piece, square: Square) -> Position {
        let bit = Bitboard::from(square);

        self.boards[(piece.color, piece.typ)] |= bit;
        self.occupied[piece.color] |= bit;
        self.all |= bit;

        #[cfg(debug_assertions)]
        self.debug_assert_valid();

        self
    }

    pub fn remove_piece(mut self, square: Square) -> Position {
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

        self
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
            castling_rights: Default::default(),
            en_passant: None,
            player: Color::White,
        }
    }
}
#[repr(u8)]
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, EnumCount)]
pub enum CastlingType {
    BlackQueenside,
    BlackKingside,
    WhiteQueenside,
    WhiteKingside,
}
impl CastlingType {
    fn idx(&self) -> usize {
        *self as usize
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
pub mod bitboard;
pub mod print;

#[cfg(test)]
mod tests;
