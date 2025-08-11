use crate::engine::minimax::*;

#[test]
fn test_full_tree_max_player_1() {
    let tree = build_tree();
    let result = evaluate(&tree, Player::MAX);
    assert_eq!(result.0.unwrap().id, 3);
}
#[test]
fn test_full_tree_max_player_2() {
    let tree = build_tree();
    let result = evaluate(&tree, Player::MAX);
    assert_eq!(result.1, 5);
}

#[test]
fn test_full_tree_min_player_1() {
    let tree = build_tree();
    let result = evaluate(&tree, Player::MIN);
    assert_eq!(result.0.unwrap().id, 1);
}
#[test]
fn test_full_tree_min_player_2() {
    let tree = build_tree();
    let result = evaluate(&tree, Player::MIN);
    assert_eq!(result.1, 3);
}

#[test]
fn test_game_over() {
    let empty_tree = Position {
        id: 0,
        value: 11,
        children: Vec::new(),
    };
    let result = evaluate(&empty_tree, Player::MAX);
    assert!(result.0.is_none());
    assert_eq!(result.1, 11);
}

fn build_tree() -> Position {
    Position::branch(
        0,
        vec![
            Position::branch(
                1,
                vec![
                    Position::branch(11, vec![Position::leafe(111, 4), Position::leafe(112, 3)]),
                    Position::branch(12, vec![Position::leafe(121, 6), Position::leafe(122, 2)]),
                ],
            ),
            Position::branch(
                2,
                vec![
                    Position::branch(21, vec![Position::leafe(211, 2), Position::leafe(212, 1)]),
                    Position::branch(22, vec![Position::leafe(221, 9), Position::leafe(222, 5)]),
                    Position::branch(23, vec![Position::leafe(231, 3), Position::leafe(232, 1)]),
                ],
            ),
            Position::branch(
                3,
                vec![
                    Position::branch(31, vec![Position::leafe(311, 5), Position::leafe(312, 4)]),
                    Position::branch(32, vec![Position::leafe(321, 7), Position::leafe(322, 5)]),
                ],
            ),
        ],
    )
}

#[derive(Debug)]
pub struct Position {
    pub id: u32,
    pub value: i32,
    pub children: Vec<Position>,
}

impl Minimax for Position {
    fn evaluate(&self) -> i32 {
        self.value
    }

    fn is_game_over(&self) -> bool {
        self.children.is_empty()
    }

    fn get_children(&self) -> &Vec<Self>
    where
        Self: Sized,
    {
        &self.children
    }
}

impl Position {
    fn branch(id: u32, children: Vec<Position>) -> Position {
        Position {
            id,
            value: 0,
            children,
        }
    }
    fn leafe(id: u32, value: i32) -> Position {
        Position {
            id,
            value,
            children: Vec::new(),
        }
    }
}
