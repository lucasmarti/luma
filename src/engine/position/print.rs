use crate::engine::position::bitboard::Bitboard;

use super::Position;

pub trait Print {
    fn print_board(&self);
    fn print(&self, name: String, board: Bitboard);
}

impl Print for Position {
    fn print_board(&self) {
        let mut chars: [char; 64] = ['_'; 64];
        for square in self.black_queen.iter() {
            chars[square.as_index() as usize] = 'q';
        }
        for square in self.black_king.iter() {
            chars[square.as_index() as usize] = 'k';
        }
        for square in self.black_rooks.iter() {
            chars[square.as_index() as usize] = 'r';
        }
        for square in self.black_knights.iter() {
            chars[square.as_index() as usize] = 'n';
        }
        for square in self.black_pawns.iter() {
            chars[square.as_index() as usize] = 'p';
        }
        for square in self.black_bishops.iter() {
            chars[square.as_index() as usize] = 'b';
        }

        for square in self.white_queen.iter() {
            chars[square.as_index() as usize] = 'Q';
        }
        for square in self.white_king.iter() {
            chars[square.as_index() as usize] = 'K';
        }
        for square in self.white_rooks.iter() {
            chars[square.as_index() as usize] = 'R';
        }
        for square in self.white_knights.iter() {
            chars[square.as_index() as usize] = 'N';
        }
        for square in self.white_pawns.iter() {
            chars[square.as_index() as usize] = 'P';
        }
        for square in self.white_bishops.iter() {
            chars[square.as_index() as usize] = 'B';
        }
        for row in (0..8).rev() {
            for column in 0..8 {
                let index = row * 8 + column;
                print!("{}", chars[index]);
            }
            println!();
        }
        println!();
        println!("Current Player: {:?}", self.get_player());
        //println!("Heuristic Score: {:?}", heuristics(self));
        println!();
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
