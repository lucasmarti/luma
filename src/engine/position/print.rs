use crate::engine::{heuristic::heuristics, position::bitboard::Bitboard};

use super::Position;

pub trait Print {
    fn print_board(&self);
    fn print(&self, name: String, board: Bitboard);
}

impl Print for Position {
    fn print_board(&self) {
        let mut chars: [char; 64] = ['_'; 64];
        for index in self.black_queen.iter() {
            chars[index as usize] = 'q';
        }
        for index in self.black_king.iter() {
            chars[index as usize] = 'k';
        }
        for index in self.black_rooks.iter() {
            chars[index as usize] = 'r';
        }
        for index in self.black_knights.iter() {
            chars[index as usize] = 'n';
        }
        for index in self.black_pawns.iter() {
            chars[index as usize] = 'p';
        }
        for index in self.black_bishops.iter() {
            chars[index as usize] = 'b';
        }

        for index in self.white_queen.iter() {
            chars[index as usize] = 'Q';
        }
        for index in self.white_king.iter() {
            chars[index as usize] = 'K';
        }
        for index in self.white_rooks.iter() {
            chars[index as usize] = 'R';
        }
        for index in self.white_knights.iter() {
            chars[index as usize] = 'N';
        }
        for index in self.white_pawns.iter() {
            chars[index as usize] = 'P';
        }
        for index in self.white_bishops.iter() {
            chars[index as usize] = 'B';
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
        println!("Heuristic Score: {:?}", heuristics(self));
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
