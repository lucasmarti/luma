use crate::engine::{
    directions::squares::*,
    piece::{Color, Typ, *},
    position::bitboard::Bitboard,
};
use std::{
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};
use strum::EnumCount;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Position {
    boards: [[Bitboard; Typ::COUNT]; Color::COUNT],
    occupied: [Bitboard; Color::COUNT],
    all: Bitboard,
    castling_rights: [bool; 4],
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
        self.castling_rights[castling_type.as_index()]
    }

    pub fn remove_castling_right(mut self, castling_type: CastlingType) -> Position {
        self.castling_rights[castling_type.as_index()] = false;
        self
    }
    pub fn is_occupied(&self, square: Square) -> bool {
        self.all.contains(square)
    }

    pub fn is_occupied_by_color(&self, square: Square, color: Color) -> bool {
        self.occupied[color.idx()].contains(square)
    }
    pub fn is_occupied_by_piece(&self, square: Square, piece: Piece) -> bool {
        self[(piece.color, piece.typ)].contains(square)
    }
    pub fn is_occupied_by(&self, square: Square, color: Color, typ: Typ) -> bool {
        self[(color, typ)].contains(square)
    }

    pub fn count_pieces(&self, color: Color, typ: Typ) -> u32 {
        self[(color, typ)].count_ones()
    }

    pub fn get_king_square(&self, color: Color) -> Square {
        let king = self[(color, Typ::King)];
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
        self[(color, typ)]
    }

    pub fn put_piece(mut self, piece: Piece, square: Square) -> Position {
        let bit = Bitboard::from(square);

        self[(piece.color, piece.typ)] |= bit;
        self.occupied[piece.color.idx()] |= bit;
        self.all |= bit;

        #[cfg(debug_assertions)]
        self.debug_assert_valid();

        self
    }

    pub fn remove_piece(mut self, square: Square) -> Position {
        let bit = Bitboard::from(square);
        let mask = !bit;

        for color in Color::ALL {
            for typ in Typ::ALL {
                self[(color, typ)] &= mask;
            }

            self.occupied[color.idx()] &= mask;
        }

        self.all &= mask;

        #[cfg(debug_assertions)]
        self.debug_assert_valid();

        self
    }

    #[cfg(debug_assertions)]
    fn debug_assert_valid(&self) {
        let mut all = Bitboard::default();

        for color in Color::ALL {
            let mut occupied = Bitboard::default();

            for typ in Typ::ALL {
                occupied |= self[(color, typ)];
            }

            debug_assert_eq!(
                occupied,
                self.occupied[color.idx()],
                "occupied cache incorrect for {:?}",
                color,
            );

            all |= occupied;
        }

        debug_assert_eq!(all, self.all, "all cache incorrect");
    }
    pub fn get_piece_at(&self, square: Square) -> Option<Piece> {
        for color in Color::ALL {
            for typ in Typ::ALL {
                if self[(color, typ)].contains(square) {
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
            castling_rights: [true; 4],
            en_passant: None,
            player: Color::White,
        }
    }
}

impl Index<(Color, Typ)> for Position {
    type Output = Bitboard;

    fn index(&self, (color, typ): (Color, Typ)) -> &Self::Output {
        &self.boards[color.idx()][typ.idx()]
    }
}
impl IndexMut<(Color, Typ)> for Position {
    fn index_mut(&mut self, (color, typ): (Color, Typ)) -> &mut Self::Output {
        &mut self.boards[color.idx()][typ.idx()]
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum CastlingType {
    BlackQueenside = 0,
    BlackKingside = 1,
    WhiteQueenside = 2,
    WhiteKingside = 3,
}
impl CastlingType {
    fn as_index(&self) -> usize {
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
