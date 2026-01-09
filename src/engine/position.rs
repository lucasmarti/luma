use crate::engine::{
    directions::squares::*,
    piece::{Color, Piece, Typ},
    position::bitboard::Bitboard,
};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Eq)]
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
    en_passant: Option<Square>,
    player: Color,
}
impl Position {
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
        let king = match color {
            Color::Black => Piece::BlackKing,
            Color::White => Piece::WhiteKing,
        };
        if let Some(square) = self.get_squares(king).iter().next() {
            return square;
        }
        panic!("No King found {:?}", color);
    }

    pub fn set_en_passant(mut self, square: Square) -> Position {
        if A4 <= square && square <= H5 {
            self.en_passant = Some(square);
            self
        } else {
            panic!("Invalid en passant square {:?}", square);
        }
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
        match piece.get_color() {
            Color::Black => match piece.get_type() {
                Typ::King => self.black_king,
                Typ::Queen => self.black_queen,
                Typ::Rook => self.black_rooks,
                Typ::Pawn => self.black_pawns,
                Typ::Knight => self.black_knights,
                Typ::Bishop => self.black_bishops,
            },
            Color::White => match piece.get_type() {
                Typ::King => self.white_king,
                Typ::Queen => self.white_queen,
                Typ::Rook => self.white_rooks,
                Typ::Pawn => self.white_pawns,
                Typ::Knight => self.white_knights,
                Typ::Bishop => self.white_bishops,
            },
        }
    }

    pub fn put_piece(mut self, piece: Piece, square: Square) -> Position {
        match piece.get_color() {
            Color::Black => match piece.get_type() {
                Typ::King => self.black_king.set_bit(square),
                Typ::Queen => self.black_queen.set_bit(square),
                Typ::Rook => self.black_rooks.set_bit(square),
                Typ::Pawn => self.black_pawns.set_bit(square),
                Typ::Knight => self.black_knights.set_bit(square),
                Typ::Bishop => self.black_bishops.set_bit(square),
            },
            Color::White => match piece.get_type() {
                Typ::King => self.white_king.set_bit(square),
                Typ::Queen => self.white_queen.set_bit(square),
                Typ::Rook => self.white_rooks.set_bit(square),
                Typ::Pawn => self.white_pawns.set_bit(square),
                Typ::Knight => self.white_knights.set_bit(square),
                Typ::Bishop => self.white_bishops.set_bit(square),
            },
        }
        self
    }

    pub fn remove_piece(mut self, square: Square) -> Position {
        self.black_king.remove_bit(square);
        self.black_queen.remove_bit(square);
        self.black_rooks.remove_bit(square);
        self.black_pawns.remove_bit(square);
        self.black_knights.remove_bit(square);
        self.black_bishops.remove_bit(square);
        self.white_king.remove_bit(square);
        self.white_queen.remove_bit(square);
        self.white_rooks.remove_bit(square);
        self.white_pawns.remove_bit(square);
        self.white_knights.remove_bit(square);
        self.white_bishops.remove_bit(square);
        self
    }

    pub fn get_piece_at(self, square: Square) -> Option<Piece> {
        if self.black_king.contains(square) {
            return Some(Piece::BlackKing);
        };
        if self.black_queen.contains(square) {
            return Some(Piece::BlackQueen);
        }
        if self.black_rooks.contains(square) {
            return Some(Piece::BlackRook);
        }
        if self.black_pawns.contains(square) {
            return Some(Piece::BlackPawn);
        }
        if self.black_knights.contains(square) {
            return Some(Piece::BlackKnight);
        }
        if self.black_bishops.contains(square) {
            return Some(Piece::BlackBishop);
        }

        if self.white_king.contains(square) {
            return Some(Piece::WhiteKing);
        };
        if self.white_queen.contains(square) {
            return Some(Piece::WhiteQueen);
        }
        if self.white_rooks.contains(square) {
            return Some(Piece::WhiteRook);
        }
        if self.white_pawns.contains(square) {
            return Some(Piece::WhitePawn);
        }
        if self.white_knights.contains(square) {
            return Some(Piece::WhiteKnight);
        }
        if self.white_bishops.contains(square) {
            return Some(Piece::WhiteBishop);
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
        }
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
        self.white_king.hash(state);
        self.white_queen.hash(state);
        self.white_rooks.hash(state);
        self.white_bishops.hash(state);
        self.white_knights.hash(state);
        self.white_pawns.hash(state);
        self.black_king.hash(state);
        self.black_queen.hash(state);
        self.black_rooks.hash(state);
        self.black_bishops.hash(state);
        self.black_knights.hash(state);
        self.black_pawns.hash(state);
        self.castling_rights.hash(state);
        self.en_passant.hash(state);
        self.player.hash(state);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.white_king == other.white_king
            && self.white_queen == other.white_queen
            && self.white_rooks == other.white_rooks
            && self.white_bishops == other.white_bishops
            && self.white_knights == other.white_knights
            && self.white_pawns == other.white_pawns
            && self.black_king == other.black_king
            && self.black_queen == other.black_queen
            && self.black_rooks == other.black_rooks
            && self.black_bishops == other.black_bishops
            && self.black_knights == other.black_knights
            && self.black_pawns == other.black_pawns
            && self.castling_rights == other.castling_rights
            && self.en_passant == other.en_passant
            && self.player == other.player
    }
}
pub mod bitboard;
pub mod print;

#[cfg(test)]
mod tests;
