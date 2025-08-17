use crate::engine::minimax::*;

#[test]
fn test_full_tree_max_player_1() {
    let tree = build_tree();
    let result = evaluate(&tree, Player::MAX, 2);
    assert_eq!(result.0.unwrap().id, 3);
}
#[test]
fn test_full_tree_max_player_2() {
    let tree = build_tree();
    let result = evaluate(&tree, Player::MAX, 2);
    assert_eq!(result.1, 5);
}

#[test]
fn test_full_tree_min_player_1() {
    let tree = build_tree();
    let result = evaluate(&tree, Player::MIN, 2);
    assert_eq!(result.0.unwrap().id, 1);
}
#[test]
fn test_full_tree_min_player_2() {
    let tree = build_tree();
    let result = evaluate(&tree, Player::MIN, 2);
    assert_eq!(result.1, 3);
}

#[test]
fn test_game_over() {
    let empty_tree = TestPosition {
        id: 0,
        value: 11,
        children: Vec::new(),
    };
    let result = evaluate(&empty_tree, Player::MAX, 2);
    assert!(result.0.is_none());
    assert_eq!(result.1, 11);
}

fn build_tree() -> TestPosition {
    TestPosition::branch(
        0,
        vec![
            TestPosition::branch(
                1,
                vec![
                    TestPosition::branch(
                        11,
                        vec![TestPosition::leafe(111, 4), TestPosition::leafe(112, 3)],
                    ),
                    TestPosition::branch(
                        12,
                        vec![TestPosition::leafe(121, 6), TestPosition::leafe(122, 2)],
                    ),
                ],
            ),
            TestPosition::branch(
                2,
                vec![
                    TestPosition::branch(
                        21,
                        vec![TestPosition::leafe(211, 2), TestPosition::leafe(212, 1)],
                    ),
                    TestPosition::branch(
                        22,
                        vec![TestPosition::leafe(221, 9), TestPosition::leafe(222, 5)],
                    ),
                    TestPosition::branch(
                        23,
                        vec![TestPosition::leafe(231, 3), TestPosition::leafe(232, 1)],
                    ),
                ],
            ),
            TestPosition::branch(
                3,
                vec![
                    TestPosition::branch(
                        31,
                        vec![TestPosition::leafe(311, 5), TestPosition::leafe(312, 4)],
                    ),
                    TestPosition::branch(
                        32,
                        vec![TestPosition::leafe(321, 7), TestPosition::leafe(322, 5)],
                    ),
                ],
            ),
        ],
    )
}

#[derive(Debug)]
pub struct TestPosition {
    pub id: u32,
    pub value: i32,
    pub children: Vec<TestPosition>,
}

impl Minimax for TestPosition {
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

impl TestPosition {
    fn branch(id: u32, children: Vec<TestPosition>) -> TestPosition {
        TestPosition {
            id,
            value: 0,
            children,
        }
    }
    fn leafe(id: u32, value: i32) -> TestPosition {
        TestPosition {
            id,
            value,
            children: Vec::new(),
        }
    }
}
