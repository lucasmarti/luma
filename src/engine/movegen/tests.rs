use std::assert_eq;

use crate::engine::{
    movegen::{get_current_player_moves, *},
    MoveType, Position, Square,
};

#[test]
fn test_castling_rights() {
    // Beispiel: Teste, dass bestimmte Züge in kritischen Positionen erlaubt oder verboten sind
    MoveTest::new("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1")
        .include(&[(E1, G1)]) // Kurze Rochade sollte erlaubt sein (Beispiel je nach deinen Square-Namen)
        .run();
}

static KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
static STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
static SJE_SYMMETRIC_ALTERNATIVE: &str =
    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

#[test]
fn kiwipete() {
    // &[48, 2039, 97862, 4085603, 193690690]
    PerftTest::new(KIWIPETE).depth(4).expected(4085603).run();
}

#[test]
fn startpos() {
    // &[20, 400, 8902, 197281, 4865609]
    PerftTest::new(STARTPOS).depth(4).expected(197281).run();
}
#[test]
fn sje_symmetric_alternative() {
    // &[46, 2079, 89890, 3894594]
    PerftTest::new(SJE_SYMMETRIC_ALTERNATIVE)
        .depth(4)
        .expected(3894594)
        .run();
}

fn assert_move_exists(moves: &[Mve], from: Square, to: Square, fen: &str) {
    assert!(
        moves.iter().any(|m| m.from == from && m.to == to),
        "Expected move {from} -> {to} for FEN {fen}"
    );
}

fn assert_move_missing(moves: &[Mve], from: Square, to: Square, fen: &str) {
    assert!(
        !moves.iter().any(|m| m.from == from && m.to == to),
        "Unexpected move {from} -> {to} for FEN {fen}"
    );
}

const fn mv(piece: Piece, from: Square, to: Square, move_type: MoveType) -> Mve {
    Mve {
        piece,
        from,
        to,
        move_type,
    }
}

#[derive(Debug, Default)]
struct MoveTest {
    fen: &'static str,
    include: &'static [(Square, Square)],
    exclude: &'static [(Square, Square)],
    expected: &'static [(Square, Square)],
}

impl MoveTest {
    fn new(fen: &'static str) -> Self {
        Self {
            fen,
            ..Default::default()
        }
    }

    fn include(mut self, moves: &'static [(Square, Square)]) -> Self {
        self.include = moves;
        self
    }

    fn exclude(mut self, moves: &'static [(Square, Square)]) -> Self {
        self.exclude = moves;
        self
    }

    fn expected(mut self, moves: &'static [(Square, Square)]) -> Self {
        self.expected = moves;
        self
    }

    fn run(self) {
        let position = Position::from_fen(self.fen).unwrap();
        let moves = get_current_player_moves(&position);

        // Moves that must exist
        for &(from, to) in self.include {
            assert_move_exists(&moves, from, to, self.fen);
        }

        // Moves that must not exist
        for &(from, to) in self.exclude {
            assert_move_missing(&moves, from, to, self.fen);
        }

        // Exact move list
        if !self.expected.is_empty() {
            assert_eq!(
                moves.len(),
                self.expected.len(),
                "Wrong number of moves for FEN {}",
                self.fen
            );

            for (expected_from, expected_to) in self.expected {
                assert!(
                    moves
                        .iter()
                        .any(|mve| mve.from == *expected_from && mve.to == *expected_to),
                    "Missing move (from: {:?}, to: {:?}) for FEN {}",
                    expected_from,
                    expected_to,
                    self.fen
                );
            }
        }
    }
}

#[derive(Debug, Default)]
struct PerftTest {
    fen: &'static str,
    expected: u64,
    depth: usize,
}

impl PerftTest {
    fn new(fen: &'static str) -> Self {
        Self {
            fen,
            ..Default::default()
        }
    }

    fn expected(mut self, expected: u64) -> Self {
        self.expected = expected;
        self
    }

    fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    fn run(self) {
        let mut position = Position::from_fen(self.fen).unwrap();
        let calculated = perft(&mut position, self.depth);

        println!(
            "Depth {}: calculated = {}, expected = {}",
            self.depth, calculated, self.expected
        );
        assert_eq!(
            calculated as u64, self.expected,
            "Perft mismatch at depth {} for FEN: {}",
            self.depth, self.fen
        );
    }
}

fn perft(position: &mut Position, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = get_current_player_moves(position);

    if depth == 1 {
        return moves.len();
    }

    let mut total_nodes = 0;
    for mve in moves {
        let undo = position.make_move(mve);
        total_nodes += perft(position, depth - 1);
        position.unmake(mve, undo);
    }
    total_nodes
}
