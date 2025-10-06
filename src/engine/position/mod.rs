use core::borrow;

use crate::engine::{
    chess_moves::{self, ChessMove, MoveType},
    directions::squares::*,
    piece::{
        Color, Piece, Typ, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN,
        BLACK_ROOK, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    position::bitboard::Bitboard,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    white_king: Bitboard,
    white_queen: Bitboard,
    white_rooks: Bitboard,
    white_bishops: Bitboard,
    white_knights: Bitboard,
    white_pawns: Bitboard,
    black_king: Bitboard,
    black_queen: Bitboard,
    black_rooks: Bitboard,
    black_bishops: Bitboard,
    black_knights: Bitboard,
    black_pawns: Bitboard,
    castling_rights: [bool; 4],
    en_passant: Option<u32>,
    player: Color,
    chess_move: Option<ChessMove>,
}
impl Position {
    pub fn get_promotion_piece(&self) -> Option<Piece> {
        match self.chess_move {
            Some(chess_move) => chess_move.pormotion,
            None => None,
        }
    }
    pub fn set_chess_move(mut self, chess_move: ChessMove) -> Position {
        self.chess_move = Some(chess_move);
        self
    }

    pub fn get_to_square(&self) -> Option<u32> {
        match self.chess_move {
            Some(chess_move) => Some(chess_move.to),
            None => None,
        }
    }

    pub fn get_from_square(&self) -> Option<u32> {
        match self.chess_move {
            Some(chess_move) => Some(chess_move.from),
            None => None,
        }
    }

    pub fn is_promotion(&self) -> bool {
        match self.chess_move {
            Some(chess_move) => match chess_move.move_type {
                MoveType::Promotion => true,
                MoveType::PromotionCapture => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn new_starting_position() -> Position {
        Position {
            white_king: Bitboard::from(E1),
            white_queen: Bitboard::from(D1),
            white_rooks: Bitboard::from_vec(vec![A1, H1]),
            white_bishops: Bitboard::from_vec(vec![C1, F1]),
            white_knights: Bitboard::from_vec(vec![B1, G1]),
            white_pawns: Bitboard::from_vec(vec![A2, B2, C2, D2, E2, F2, G2, H2]),
            black_king: Bitboard::from(E8),
            black_queen: Bitboard::from(D8),
            black_rooks: Bitboard::from_vec(vec![A8, H8]),
            black_bishops: Bitboard::from_vec(vec![C8, F8]),
            black_knights: Bitboard::from_vec(vec![B8, G8]),
            black_pawns: Bitboard::from_vec(vec![A7, B7, C7, D7, E7, F7, G7, H7]),
            ..Default::default()
        }
    }

    pub fn disallow_castle_for_color(mut self, color: Color) -> Position {
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
    pub fn is_occupied(&self, index: u32) -> bool {
        self.get_all().contains(index)
    }

    pub fn is_occupied_by_color(&self, index: u32, color: Color) -> bool {
        match color {
            Color::Black => self.get_black().contains(index),
            Color::White => self.get_white().contains(index),
        }
    }
    pub fn is_occupied_by_piece(&self, index: u32, piece: Piece) -> bool {
        self.get_squares(piece).contains(index)
    }

    pub fn count_pieces(&self, piece: Piece) -> u32 {
        self.get_squares(piece).count_ones()
    }

    pub fn get_king_square(&self, color: Color) -> u32 {
        for square in self
            .get_squares(Piece {
                typ: Typ::King,
                color: color,
            })
            .iter()
        {
            return square;
        }
        panic!("No King found {:?}", color);
    }

    pub fn set_en_passant(mut self, index: u32) -> Position {
        if A4 <= index && index <= H5 {
            self.en_passant = Some(index);
            self
        } else {
            panic!("Invalid en passant square {:?}", index);
        }
    }
    pub fn reset_en_passant(mut self) -> Position {
        self.en_passant = None;
        self
    }

    pub fn get_player(&self) -> Color {
        self.player
    }

    pub fn get_en_passant(&self) -> Option<u32> {
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
        self.black_king
            | self.black_queen
            | self.black_rooks
            | self.black_knights
            | self.black_pawns
            | self.black_bishops
    }

    fn get_white(&self) -> Bitboard {
        self.white_king
            | self.white_queen
            | self.white_rooks
            | self.white_knights
            | self.white_pawns
            | self.white_bishops
    }

    fn get_all(&self) -> Bitboard {
        self.get_black() | self.get_white()
    }

    pub fn get_squares(&self, piece: Piece) -> Bitboard {
        match piece.color {
            Color::Black => match piece.typ {
                Typ::King => self.black_king,
                Typ::Queen => self.black_queen,
                Typ::Rook => self.black_rooks,
                Typ::Pawn => self.black_pawns,
                Typ::Knight => self.black_knights,
                Typ::Bishop => self.black_bishops,
            },
            Color::White => match piece.typ {
                Typ::King => self.white_king,
                Typ::Queen => self.white_queen,
                Typ::Rook => self.white_rooks,
                Typ::Pawn => self.white_pawns,
                Typ::Knight => self.white_knights,
                Typ::Bishop => self.white_bishops,
            },
        }
    }

    pub fn put_piece(mut self, piece: Piece, index: u32) -> Position {
        match piece.color {
            Color::Black => match piece.typ {
                Typ::King => self.black_king.set_bit(index),
                Typ::Queen => self.black_queen.set_bit(index),
                Typ::Rook => self.black_rooks.set_bit(index),
                Typ::Pawn => self.black_pawns.set_bit(index),
                Typ::Knight => self.black_knights.set_bit(index),
                Typ::Bishop => self.black_bishops.set_bit(index),
            },
            Color::White => match piece.typ {
                Typ::King => self.white_king.set_bit(index),
                Typ::Queen => self.white_queen.set_bit(index),
                Typ::Rook => self.white_rooks.set_bit(index),
                Typ::Pawn => self.white_pawns.set_bit(index),
                Typ::Knight => self.white_knights.set_bit(index),
                Typ::Bishop => self.white_bishops.set_bit(index),
            },
        }
        self
    }

    pub fn remove_piece(mut self, index: u32) -> Position {
        self.black_king.remove_bit(index);
        self.black_queen.remove_bit(index);
        self.black_rooks.remove_bit(index);
        self.black_pawns.remove_bit(index);
        self.black_knights.remove_bit(index);
        self.black_bishops.remove_bit(index);
        self.white_king.remove_bit(index);
        self.white_queen.remove_bit(index);
        self.white_rooks.remove_bit(index);
        self.white_pawns.remove_bit(index);
        self.white_knights.remove_bit(index);
        self.white_bishops.remove_bit(index);
        self
    }

    pub fn get_piece_at(self, square: u32) -> Option<Piece> {
        if self.black_king.contains(square) {
            return Some(BLACK_KING);
        };
        if self.black_queen.contains(square) {
            return Some(BLACK_QUEEN);
        }
        if self.black_rooks.contains(square) {
            return Some(BLACK_ROOK);
        }
        if self.black_pawns.contains(square) {
            return Some(BLACK_PAWN);
        }
        if self.black_knights.contains(square) {
            return Some(BLACK_KNIGHT);
        }
        if self.black_bishops.contains(square) {
            return Some(BLACK_BISHOP);
        }

        if self.white_king.contains(square) {
            return Some(WHITE_KING);
        };
        if self.white_queen.contains(square) {
            return Some(WHITE_QUEEN);
        }
        if self.white_rooks.contains(square) {
            return Some(WHITE_ROOK);
        }
        if self.white_pawns.contains(square) {
            return Some(WHITE_PAWN);
        }
        if self.white_knights.contains(square) {
            return Some(WHITE_KNIGHT);
        }
        if self.white_bishops.contains(square) {
            return Some(WHITE_BISHOP);
        }
        None
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            white_king: Default::default(),
            white_queen: Default::default(),
            white_rooks: Default::default(),
            white_bishops: Default::default(),
            white_knights: Default::default(),
            white_pawns: Default::default(),
            black_king: Default::default(),
            black_queen: Default::default(),
            black_rooks: Default::default(),
            black_bishops: Default::default(),
            black_knights: Default::default(),
            black_pawns: Default::default(),
            castling_rights: [true, true, true, true],
            en_passant: None,
            player: Color::White,
            chess_move: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
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
pub mod bitboard;
pub mod print;

#[cfg(test)]
mod tests;
