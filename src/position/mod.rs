use crate::{
    bitboard::Bitboard,
    chess_moves::{ChessMove, EnPassant, Progress, Promotion},
    directions::*,
    piece::{Color, Piece, Typ, BLACK_KING, BLACK_ROOK, WHITE_KING, WHITE_ROOK},
};
#[derive(Clone, Copy)]
pub struct Position {
    pub white_king: Bitboard,
    pub white_queen: Bitboard,
    pub white_rooks: Bitboard,
    pub white_bishops: Bitboard,
    pub white_knights: Bitboard,
    pub white_pawns: Bitboard,
    pub white_kingside_castle: bool,
    pub white_queenside_castle: bool,
    pub black_king: Bitboard,
    pub black_queen: Bitboard,
    pub black_rooks: Bitboard,
    pub black_bishops: Bitboard,
    pub black_knights: Bitboard,
    pub black_pawns: Bitboard,
    pub black_kingside_castle: bool,
    pub black_queenside_castle: bool,
    pub en_passant: Option<u32>,
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

    pub fn get_black(&self) -> Bitboard {
        self.black_king
            | self.black_queen
            | self.black_rooks
            | self.black_knights
            | self.black_pawns
            | self.black_bishops
    }

    pub fn get_white(&self) -> Bitboard {
        self.white_king
            | self.white_queen
            | self.white_rooks
            | self.white_knights
            | self.white_pawns
            | self.white_bishops
    }

    pub fn get_all(&self) -> Bitboard {
        self.get_black() | self.get_white()
    }

    pub fn move_piece(self, possible_moves: ChessMove) -> Position {
        match possible_moves {
            ChessMove::Progress(progress) => self.progress(progress),
            ChessMove::Promotion(promotion) => self.promote(promotion),
            ChessMove::EnPassant(en_passant) => self.enPassant(en_passant),
            ChessMove::BlackKingsideCastle => self.black_kingside_castle(),
            ChessMove::BlackQueensideCastle => self.black_queenside_castle(),
            ChessMove::WhiteKingsideCastle => self.white_kingside_castle(),
            ChessMove::WhiteQueensideCastle => self.white_queenside_castle(),
        }
    }
    fn progress(self, progress: Progress) -> Position {
        self.remove_piece(progress.from)
            .remove_piece(progress.to)
            .put_piece(progress.piece, progress.to)
    }

    fn promote(self, promotion: Promotion) -> Position {
        self.remove_piece(promotion.from)
            .remove_piece(promotion.to)
            .put_piece(promotion.new_piece, promotion.to)
    }
    fn enPassant(self, enPassant: EnPassant) -> Position {
        self.remove_piece(enPassant.from)
            .remove_piece(enPassant.capture)
            .remove_piece(enPassant.to)
            .put_piece(enPassant.piece, enPassant.to)
    }

    fn white_queenside_castle(self) -> Position {
        self.remove_piece(E1)
            .put_piece(WHITE_KING, C1)
            .remove_piece(A1)
            .put_piece(WHITE_ROOK, D1)
    }

    fn white_kingside_castle(self) -> Position {
        self.remove_piece(E1)
            .put_piece(WHITE_KING, G1)
            .remove_piece(H1)
            .put_piece(WHITE_ROOK, F1)
    }

    fn black_queenside_castle(self) -> Position {
        self.remove_piece(E8)
            .put_piece(BLACK_KING, C8)
            .remove_piece(A8)
            .put_piece(BLACK_ROOK, D8)
    }

    fn black_kingside_castle(self) -> Position {
        self.remove_piece(E8)
            .put_piece(BLACK_KING, G8)
            .remove_piece(H8)
            .put_piece(BLACK_ROOK, F8)
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

    fn remove_piece(mut self, index: u32) -> Position {
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
            white_kingside_castle: false,
            white_queenside_castle: false,
            black_kingside_castle: false,
            black_queenside_castle: false,
            en_passant: None,
        }
    }
}
mod print;

#[cfg(test)]
mod tests;
