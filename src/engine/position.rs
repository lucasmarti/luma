use crate::engine::{
    directions::squares::*,
    piece::{Color, Piece, Typ},
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
    castling_rights: [bool; 4],
    en_passant: Option<Square>,
    player: Color,
}
impl Position {
    pub fn new_starting_position() -> Position {
        let mut position = Position::default();

        position[(Color::White, Typ::King)] = Bitboard::from(E1);
        position[(Color::White, Typ::Queen)] = Bitboard::from(D1);
        position[(Color::White, Typ::Rook)] = Bitboard::from_vec(vec![A1, H1]);
        position[(Color::White, Typ::Bishop)] = Bitboard::from_vec(vec![C1, F1]);
        position[(Color::White, Typ::Knight)] = Bitboard::from_vec(vec![B1, G1]);
        position[(Color::White, Typ::Pawn)] =
            Bitboard::from_vec(vec![A2, B2, C2, D2, E2, F2, G2, H2]);

        position[(Color::Black, Typ::King)] = Bitboard::from(E8);
        position[(Color::Black, Typ::Queen)] = Bitboard::from(D8);
        position[(Color::Black, Typ::Rook)] = Bitboard::from_vec(vec![A8, H8]);
        position[(Color::Black, Typ::Bishop)] = Bitboard::from_vec(vec![C8, F8]);
        position[(Color::Black, Typ::Knight)] = Bitboard::from_vec(vec![B8, G8]);
        position[(Color::Black, Typ::Pawn)] =
            Bitboard::from_vec(vec![A7, B7, C7, D7, E7, F7, G7, H7]);

        position
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
        self.get_all().contains(square)
    }

    pub fn is_occupied_by_color(&self, square: Square, color: Color) -> bool {
        match color {
            Color::Black => self.get_black().contains(square),
            Color::White => self.get_white().contains(square),
        }
    }
    pub fn is_occupied_by_piece(&self, square: Square, piece: Piece) -> bool {
        self.get_squares(piece).contains(square)
    }

    pub fn count_pieces(&self, piece: Piece) -> u32 {
        self.get_squares(piece).count_ones()
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
    fn get_black(&self) -> Bitboard {
        self.get_color(Color::Black)
    }

    fn get_white(&self) -> Bitboard {
        self.get_color(Color::White)
    }

    fn get_color(&self, color: Color) -> Bitboard {
        let mut bitboard = Bitboard::default();

        for typ in Typ::ALL {
            bitboard = bitboard | self[(color, typ)];
        }
        bitboard
    }
    fn get_all(&self) -> Bitboard {
        self.get_black() | self.get_white()
    }

    pub fn get_all_pieces(&self) -> Vec<(Square, Piece)> {
        let mut all_pieces: Vec<(Square, Piece)> = Vec::new();
        for square in self.get_all().iter() {
            if let Some(piece) = self.get_piece_at(square) {
                all_pieces.push((square, piece));
            }
        }
        all_pieces
    }

    pub fn get_squares(&self, piece: Piece) -> Bitboard {
        self[(piece.get_color(), piece.get_type())]
    }

    pub fn put_piece(mut self, piece: Piece, square: Square) -> Position {
        self[(piece.get_color(), piece.get_type())].set_bit(square);
        self
    }

    pub fn remove_piece(mut self, square: Square) -> Position {
        for color in Color::ALL {
            for typ in Typ::ALL {
                self[(color, typ)].remove_bit(square);
            }
        }
        self
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
