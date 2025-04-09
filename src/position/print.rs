use crate::bitboard::Bitboard;

use super::Position;

pub trait Print {
    fn print_board(&self);
    fn print(&self, name: String, board: Bitboard);
}

impl Print for Position {
    fn print_board(&self) {
        self.print("White King".to_string(), self.white_king);
        self.print("White Queen".to_string(), self.white_queen);
        self.print("White Rook".to_string(), self.white_rooks);
        self.print("White Knight".to_string(), self.white_knights);
        self.print("White Bishop".to_string(), self.white_bishops);
        self.print("White Pawn".to_string(), self.white_pawns);
        self.print("Black King".to_string(), self.black_king);
        self.print("Black Queen".to_string(), self.black_queen);
        self.print("Black Rook".to_string(), self.black_rooks);
        self.print("Black Knight".to_string(), self.black_knights);
        self.print("Black Bishop".to_string(), self.black_bishops);
        self.print("Black Pawn".to_string(), self.black_pawns);
    }

    fn print(&self, name: String, board: Bitboard) {
        println!("{}", name);
        for row in (0..8).rev() {
            for column in 0..8 {
                let index = row * 8 + column;
                let cell: u64 = 1 << index;
                if cell & board.get_inner() > 0 {
                    print!("X ");
                } else {
                    print!("_ ");
                }
            }
            println!();
        }
        println!();
    }
}
