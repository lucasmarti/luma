use crate::engine::{
    piece::{Color, Typ},
    position::bitboard::Bitboard,
};

use super::Position;

#[allow(dead_code)]
pub trait Print {
    fn print_board(&self);
    fn print(&self, name: String, board: Bitboard);
    fn symbol(&self, color: Color, typ: Typ) -> char;
}

impl Print for Position {
    fn symbol(&self, color: Color, typ: Typ) -> char {
        match (color, typ) {
            (Color::White, Typ::King) => 'K',
            (Color::White, Typ::Queen) => 'Q',
            (Color::White, Typ::Rook) => 'R',
            (Color::White, Typ::Bishop) => 'B',
            (Color::White, Typ::Knight) => 'N',
            (Color::White, Typ::Pawn) => 'P',

            (Color::Black, Typ::King) => 'k',
            (Color::Black, Typ::Queen) => 'q',
            (Color::Black, Typ::Rook) => 'r',
            (Color::Black, Typ::Bishop) => 'b',
            (Color::Black, Typ::Knight) => 'n',
            (Color::Black, Typ::Pawn) => 'p',
        }
    }

    fn print_board(&self) {
        let mut chars = ['_'; 64];

        for color in Color::ALL {
            for typ in Typ::ALL {
                for square in self[(color, typ)].iter() {
                    chars[square.as_index() as usize] = self.symbol(color, typ);
                }
            }
        }

        for row in (0..8).rev() {
            for column in 0..8 {
                print!("{}", chars[row * 8 + column]);
            }
            println!();
        }

        println!();
        println!("Current Player: {:?}", self.get_player());
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
