use crate::engine::movegen::*;
use crate::engine::{position::CastlingRights, Position, Square, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenError {
    InvalidFieldCount,
    InvalidBoard,
    InvalidSideToMove,
    InvalidCastlingRights,
    InvalidEnPassantSquare,
    InvalidHalfmoveClock,
    InvalidFullmoveNumber,
}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Position, FenError> {
        let fields: Vec<&str> = fen.split_whitespace().collect();

        if fields.len() != 6 {
            return Err(FenError::InvalidFieldCount);
        }

        let mut position = Position {
            castling_rights: CastlingRights::NONE,
            ..Position::default()
        };

        // Piece placement
        let ranks = fields[0].split('/').collect::<Vec<_>>();

        if ranks.len() != 8 {
            return Err(FenError::InvalidBoard);
        }

        for (fen_rank, rank) in ranks.iter().enumerate() {
            let mut file = 0u32;

            for c in rank.chars() {
                match c {
                    '1'..='8' => {
                        file += c.to_digit(10).unwrap();
                    }

                    'P' | 'N' | 'B' | 'R' | 'Q' | 'K' | 'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                        if file >= 8 {
                            return Err(FenError::InvalidBoard);
                        }

                        // FEN starts at rank 8, our board starts at rank 1.
                        let rank_from_white = 7 - fen_rank as u32;
                        let square_index = rank_from_white * 8 + file;

                        let square = Square::new(square_index).ok_or(FenError::InvalidBoard)?;

                        let piece = match c {
                            'P' => WHITE_PAWN,
                            'N' => WHITE_KNIGHT,
                            'B' => WHITE_BISHOP,
                            'R' => WHITE_ROOK,
                            'Q' => WHITE_QUEEN,
                            'K' => WHITE_KING,

                            'p' => BLACK_PAWN,
                            'n' => BLACK_KNIGHT,
                            'b' => BLACK_BISHOP,
                            'r' => BLACK_ROOK,
                            'q' => BLACK_QUEEN,
                            'k' => BLACK_KING,

                            _ => unreachable!(),
                        };

                        position.put_piece(piece, square);
                        file += 1;
                    }

                    _ => return Err(FenError::InvalidBoard),
                }
            }

            if file != 8 {
                return Err(FenError::InvalidBoard);
            }
        }

        // Side to move
        position.player = match fields[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenError::InvalidSideToMove),
        };

        // Castling rights
        for c in fields[2].chars() {
            let rights = match c {
                'K' => CastlingRights::WHITE_KINGSIDE,
                'Q' => CastlingRights::WHITE_QUEENSIDE,
                'k' => CastlingRights::BLACK_KINGSIDE,
                'q' => CastlingRights::BLACK_QUEENSIDE,
                '-' if fields[2] == "-" => continue,
                _ => return Err(FenError::InvalidCastlingRights),
            };

            position.castling_rights.add(rights);
        }

        // En passant
        position.en_passant = if fields[3] == "-" {
            None
        } else {
            Some(parse_fen_square(fields[3])?)
        };

        // Halfmove clock.
        // Position currently does not store it, so only validate it.
        fields[4]
            .parse::<u32>()
            .map_err(|_| FenError::InvalidHalfmoveClock)?;

        // Fullmove number.
        // Position currently does not store it, so only validate it.
        fields[5]
            .parse::<u32>()
            .map_err(|_| FenError::InvalidFullmoveNumber)?;

        Ok(position)
    }
}

fn parse_fen_square(s: &str) -> Result<Square, FenError> {
    let bytes = s.as_bytes();

    if bytes.len() != 2 {
        return Err(FenError::InvalidEnPassantSquare);
    }

    let file = match bytes[0] {
        b'a'..=b'h' => (bytes[0] - b'a') as u32,
        _ => return Err(FenError::InvalidEnPassantSquare),
    };

    let rank = match bytes[1] {
        b'1'..=b'8' => (bytes[1] - b'1') as u32,
        _ => return Err(FenError::InvalidEnPassantSquare),
    };

    Square::new(rank * 8 + file).ok_or(FenError::InvalidEnPassantSquare)
}

#[test]
fn from_fen_starting_position() {
    let position =
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    assert_eq!(position.get_player(), Color::White);

    assert_eq!(position.get_piece_at(A1), Some(WHITE_ROOK));

    assert_eq!(position.get_piece_at(E1), Some(WHITE_KING));

    assert_eq!(position.get_piece_at(A8), Some(BLACK_ROOK));

    assert_eq!(position.get_piece_at(E8), Some(BLACK_KING));
}
