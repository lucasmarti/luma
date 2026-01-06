use crate::engine::search_algorithms::node::Node;

#[cfg(test)]
use crate::engine::{
    directions::squares::*,
    piece::*,
    position::{print::Print, Position},
    search_algorithms::{alpha_beta, get_best_move, minimax, Player, MAX_VALUE, MIN_VALUE},
};
use rand::random_range;

#[test]
fn test_get_best_move() {
    let position = Position::default()
        .put_piece(Piece::BlackKing, D7)
        .put_piece(Piece::WhiteKing, D2)
        .put_piece(Piece::WhitePawn, B6)
        .put_piece(Piece::BlackKnight, A7);
    position.print_board();
    if let Some(best_position) = get_best_move(position) {
        best_position.print_board();
        assert!(best_position.is_occupied_by_piece(A7, Piece::WhitePawn));
    }
}

#[test]
fn test_get_best_move2() {
    let position = Position::default()
        .put_piece(Piece::BlackKing, D7)
        .put_piece(Piece::WhiteKing, D2)
        .put_piece(Piece::BlackPawn, B7)
        .put_piece(Piece::WhiteKnight, A6)
        .toggle_player();
    position.print_board();
    if let Some(best_position) = get_best_move(position) {
        assert!(best_position.is_occupied_by_piece(A6, Piece::BlackPawn));
    }
}

#[test]
fn test_full_tree_max_player_1() {
    let tree = build_tree();
    let result_minimax = minimax::minimax(tree.clone(), Player::Max);
    let result_alpha_beta = alpha_beta::alpha_beta(tree, Player::Max, MIN_VALUE, MAX_VALUE);

    assert_eq!(result_alpha_beta.best_node, result_minimax.0);
    assert_eq!(result_minimax.0.unwrap().id, 3);
}
#[test]
fn test_full_tree_max_player_2() {
    let tree = build_tree();
    let result_minimax = minimax::minimax(tree.clone(), Player::Max);
    let result_alpha_beta = alpha_beta::alpha_beta(tree, Player::Max, MIN_VALUE, MAX_VALUE);

    assert_eq!(result_alpha_beta.best_node, result_minimax.0);

    assert_eq!(result_minimax.1, 5.0);
}

#[test]
fn test_full_tree_min_player_1() {
    let tree = build_tree();
    let result_minimax = minimax::minimax(tree.clone(), Player::Min);
    let result_alpha_beta = alpha_beta::alpha_beta(tree, Player::Min, MIN_VALUE, MAX_VALUE);

    assert_eq!(result_alpha_beta.best_node, result_minimax.0);
    assert_eq!(result_minimax.0.unwrap().id, 1);
}
#[test]
fn test_full_tree_min_player_2() {
    let tree = build_tree();
    let result_minimax = minimax::minimax(tree.clone(), Player::Min);
    let result_alpha_beta = alpha_beta::alpha_beta(tree, Player::Min, MIN_VALUE, MAX_VALUE);

    assert_eq!(result_alpha_beta.best_node, result_minimax.0);
    assert_eq!(result_minimax.1, 3.0);
}

#[test]
fn test_game_over() {
    let empty_tree = TestPosition {
        id: 0,
        value: 11.0,
        children: Vec::new(),
    };
    let result_minimax = minimax::minimax(empty_tree.clone(), Player::Max);
    let result_alpha_beta = alpha_beta::alpha_beta(empty_tree, Player::Max, MIN_VALUE, MAX_VALUE);

    assert_eq!(result_alpha_beta.best_node, result_minimax.0);
    assert!(result_minimax.0.is_none());
    assert_eq!(result_minimax.1, 11.0);
}
/*
#[test]
fn test_bug_xyz() {
    let position = init_position_for_bug_testing();
    let depth = 2;
    let tree = search_algorithms::build_tree(position, depth);
    let minimx_player = match tree.position.get_player() {
        crate::engine::piece::Color::Black => Player::Min,
        crate::engine::piece::Color::White => Player::Max,
    };
    let result_alpha_beta = alpha_beta::evaluate(&tree, minimx_player, depth);
    let result_minimax = minimax::evaluate(&tree, minimx_player, depth);
    if let (Some(alpha), Some(minimax)) = (result_alpha_beta.0, result_minimax.0) {
        println!("Alpha Beta {:?}", alpha.id);
        println!("Minimax {:?}", minimax.id);
    }
    println!("{:?}", result_alpha_beta.1);
    assert_eq!(result_alpha_beta.1, result_minimax.1);
}
     */

#[test]
fn test_random_tree() {
    let tree = generate_random_tree(10, 3);
    let result_minimax = minimax::minimax(tree.clone(), Player::Max);
    let result_alpha_beta = alpha_beta::alpha_beta(tree.clone(), Player::Max, MIN_VALUE, MAX_VALUE);
    assert_eq!(result_alpha_beta.best_node, result_minimax.0);
}
#[allow(dead_code)]
fn generate_random_tree(depth: u32, branching_factor: u32) -> TestPosition {
    let mut id_counter = 0;

    fn build_recursive(depth: u32, branching_factor: u32, id_counter: &mut u32) -> TestPosition {
        let current_id = *id_counter;
        *id_counter += 1;

        if depth == 0 {
            // Leaf: Random Wert zwischen 0 und 100
            let value = random_range(0.0..100.0);
            return TestPosition::leafe(current_id, value);
        }

        // Branch: Erstelle Kinder
        let mut children = Vec::new();
        for _ in 0..branching_factor {
            children.push(build_recursive(depth - 1, branching_factor, id_counter));
        }

        TestPosition::branch(current_id, children)
    }

    build_recursive(depth, branching_factor, &mut id_counter)
}

#[allow(dead_code)]
fn build_tree() -> TestPosition {
    TestPosition::branch(
        0,
        vec![
            TestPosition::branch(
                1,
                vec![
                    TestPosition::branch(
                        11,
                        vec![TestPosition::leafe(111, 4.0), TestPosition::leafe(112, 3.0)],
                    ),
                    TestPosition::branch(
                        12,
                        vec![TestPosition::leafe(121, 6.0), TestPosition::leafe(122, 2.0)],
                    ),
                ],
            ),
            TestPosition::branch(
                2,
                vec![
                    TestPosition::branch(
                        21,
                        vec![TestPosition::leafe(211, 2.0), TestPosition::leafe(212, 1.0)],
                    ),
                    TestPosition::branch(
                        22,
                        vec![TestPosition::leafe(221, 9.0), TestPosition::leafe(222, 5.0)],
                    ),
                    TestPosition::branch(
                        23,
                        vec![TestPosition::leafe(231, 3.0), TestPosition::leafe(232, 1.0)],
                    ),
                ],
            ),
            TestPosition::branch(
                3,
                vec![
                    TestPosition::branch(
                        31,
                        vec![TestPosition::leafe(311, 5.0), TestPosition::leafe(312, 4.0)],
                    ),
                    TestPosition::branch(
                        32,
                        vec![TestPosition::leafe(321, 7.0), TestPosition::leafe(322, 5.0)],
                    ),
                ],
            ),
        ],
    )
}

#[derive(Debug, PartialEq, Clone)]
pub struct TestPosition {
    pub id: u32,
    pub value: f32,
    pub children: Vec<TestPosition>,
}
impl Node for TestPosition {
    fn evaluate(&self) -> f32 {
        self.value
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn get_children(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        self.children.clone()
    }
}
#[allow(dead_code)]
impl TestPosition {
    fn branch(id: u32, children: Vec<TestPosition>) -> TestPosition {
        TestPosition {
            id,
            value: 0.0,
            children,
        }
    }
    fn leafe(id: u32, value: f32) -> TestPosition {
        TestPosition {
            id,
            value,
            children: Vec::new(),
        }
    }
}
