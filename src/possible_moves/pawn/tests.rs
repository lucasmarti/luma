use crate::{
    directions::*,
    piece::{
        BLACK_BISHOP, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
        WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    position::Position,
    possible_moves::pawn::*,
};

#[cfg(test)]
mod test_white_forward {
    use super::*;

    #[test]
    fn test_valid_forward_move() {
        let position = Position::default();
        let result = get_move_white_forward(&position, E2);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(E3, WHITE_PAWN));
        assert!(!new_position.is_occupied(E2));
    }

    #[test]
    fn test_blocked_by_own_piece() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E2)
            .put_piece(WHITE_PAWN, E3);

        let result = get_move_white_forward(&position, E2);
        assert!(result.is_none());
    }

    #[test]
    fn test_blocked_by_opponent_piece() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E2)
            .put_piece(BLACK_PAWN, E3);

        let result = get_move_white_forward(&position, E2);
        assert!(result.is_none());
    }

    #[test]
    fn test_from_row_7_returns_none() {
        let position = Position::default().put_piece(WHITE_PAWN, E7);

        let result = get_move_white_forward(&position, E7);
        assert!(result.is_none());
    }

    #[test]
    fn test_from_row_8_returns_none() {
        let position = Position::default().put_piece(WHITE_PAWN, E8);

        let result = get_move_white_forward(&position, E8);
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod test_black_forward {
    use super::*;

    #[test]
    fn test_valid_forward_move() {
        let position = Position::default().put_piece(BLACK_PAWN, E7);

        let result = get_move_black_forward(&position, E7);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(E6, BLACK_PAWN));
        assert!(!new_position.is_occupied(E7));
    }

    #[test]
    fn test_blocked_by_own_piece() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E7)
            .put_piece(BLACK_PAWN, E6);

        let result = get_move_black_forward(&position, E7);
        assert!(result.is_none());
    }

    #[test]
    fn test_blocked_by_opponent_piece() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E7)
            .put_piece(WHITE_PAWN, E6);

        let result = get_move_black_forward(&position, E7);
        assert!(result.is_none());
    }

    #[test]
    fn test_from_row_2_returns_none() {
        let position = Position::default().put_piece(BLACK_PAWN, E2);

        let result = get_move_black_forward(&position, E2);
        assert!(result.is_none());
    }

    #[test]
    fn test_from_row_1_returns_none() {
        let position = Position::default().put_piece(BLACK_PAWN, E1);

        let result = get_move_black_forward(&position, E1);
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod test_white_two_forward {
    use super::*;

    #[test]
    fn test_valid_from_row_2() {
        let position = Position::default().put_piece(WHITE_PAWN, E2);

        let result = get_move_white_two_forward(&position, E2);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(E4, WHITE_PAWN));
        assert!(!new_position.is_occupied(E2));
        assert!(!new_position.is_occupied(E3));
    }

    #[test]
    fn test_not_from_other_rows() {
        let position = Position::default();

        // Test from rows 3-8
        for row in [E3, E4, E5, E6, E7, E8] {
            let pos = position.put_piece(WHITE_PAWN, row);
            let result = get_move_white_two_forward(&pos, row);
            assert!(result.is_none());
        }
    }

    #[test]
    fn test_blocked_one_square_ahead() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E2)
            .put_piece(BLACK_PAWN, E3);

        let result = get_move_white_two_forward(&position, E2);
        assert!(result.is_none());
    }

    #[test]
    fn test_blocked_two_squares_ahead() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E2)
            .put_piece(BLACK_PAWN, E4);

        let result = get_move_white_two_forward(&position, E2);
        assert!(result.is_none());
    }

    #[test]
    fn test_blocked_by_own_piece() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E2)
            .put_piece(WHITE_KNIGHT, E3);

        let result = get_move_white_two_forward(&position, E2);
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod test_black_two_forward {
    use super::*;

    #[test]
    fn test_valid_from_row_7() {
        let position = Position::default().put_piece(BLACK_PAWN, E7);

        let result = get_move_black_two_forward(&position, E7);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(E5, BLACK_PAWN));
        assert!(!new_position.is_occupied(E7));
        assert!(!new_position.is_occupied(E6));
    }

    #[test]
    fn test_not_from_other_rows() {
        let position = Position::default();

        // Test from rows 1-6
        for row in [E1, E2, E3, E4, E5, E6] {
            let pos = position.put_piece(BLACK_PAWN, row);
            let result = get_move_black_two_forward(&pos, row);
            assert!(result.is_none());
        }
    }

    #[test]
    fn test_blocked_one_square_ahead() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E7)
            .put_piece(WHITE_PAWN, E6);

        let result = get_move_black_two_forward(&position, E7);
        assert!(result.is_none());
    }

    #[test]
    fn test_blocked_two_squares_ahead() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E7)
            .put_piece(WHITE_PAWN, E5);

        let result = get_move_black_two_forward(&position, E7);
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod test_white_captures {
    use super::*;

    #[test]
    fn test_left_capture_opponent() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E4)
            .put_piece(BLACK_PAWN, D5);

        let result = get_move_white_left_capture(&position, E4);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(D5, WHITE_PAWN));
        assert!(!new_position.is_occupied(E4));
        assert!(!new_position.is_occupied_by_piece(D5, BLACK_PAWN));
    }

    #[test]
    fn test_right_capture_opponent() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E4)
            .put_piece(BLACK_PAWN, F5);

        let result = get_move_white_right_capture(&position, E4);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(F5, WHITE_PAWN));
        assert!(!new_position.is_occupied(E4));
    }

    #[test]
    fn test_no_capture_empty_square() {
        let position = Position::default().put_piece(WHITE_PAWN, E4);

        let left_result = get_move_white_left_capture(&position, E4);
        let right_result = get_move_white_right_capture(&position, E4);

        assert!(left_result.is_none());
        assert!(right_result.is_none());
    }

    #[test]
    fn test_no_capture_own_piece() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E4)
            .put_piece(WHITE_KNIGHT, D5)
            .put_piece(WHITE_BISHOP, F5);

        let left_result = get_move_white_left_capture(&position, E4);
        let right_result = get_move_white_right_capture(&position, E4);

        assert!(left_result.is_none());
        assert!(right_result.is_none());
    }

    #[test]
    fn test_from_row_7_returns_none() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E7)
            .put_piece(BLACK_PAWN, D8)
            .put_piece(BLACK_PAWN, F8);

        let left_result = get_move_white_left_capture(&position, E7);
        let right_result = get_move_white_right_capture(&position, E7);

        assert!(left_result.is_none());
        assert!(right_result.is_none());
    }

    #[test]
    fn test_edge_cases() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, A4)
            .put_piece(WHITE_PAWN, H4)
            .put_piece(BLACK_PAWN, B5)
            .put_piece(BLACK_PAWN, G5);

        // A-file can't capture left
        let a_left = get_move_white_left_capture(&position, A4);
        assert!(a_left.is_none());

        // But can capture right
        let a_right = get_move_white_right_capture(&position, A4);
        assert!(a_right.is_some());

        // H-file can't capture right
        let h_right = get_move_white_right_capture(&position, H4);
        assert!(h_right.is_none());

        // But can capture left
        let h_left = get_move_white_left_capture(&position, H4);
        assert!(h_left.is_some());
    }
}

#[cfg(test)]
mod test_black_captures {
    use super::*;

    #[test]
    fn test_left_capture_opponent() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E5)
            .put_piece(WHITE_PAWN, D4);

        let result = get_move_black_left_capture(&position, E5);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(D4, BLACK_PAWN));
        assert!(!new_position.is_occupied(E5));
    }

    #[test]
    fn test_right_capture_opponent() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E5)
            .put_piece(WHITE_PAWN, F4);

        let result = get_move_black_right_capture(&position, E5);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(F4, BLACK_PAWN));
        assert!(!new_position.is_occupied(E5));
    }

    #[test]
    fn test_from_row_2_returns_none() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E2)
            .put_piece(WHITE_PAWN, D1)
            .put_piece(WHITE_PAWN, F1);

        let left_result = get_move_black_left_capture(&position, E2);
        let right_result = get_move_black_right_capture(&position, E2);

        assert!(left_result.is_none());
        assert!(right_result.is_none());
    }
}

#[cfg(test)]
mod test_white_en_passant {
    use super::*;

    #[test]
    fn test_left_en_passant_valid() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E5)
            .put_piece(BLACK_PAWN, D5)
            .set_en_passant(D5);

        let result = get_move_white_left_en_passant(&position, E5);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(D6, WHITE_PAWN));
        assert!(!new_position.is_occupied(E5));
        assert!(!new_position.is_occupied(D5)); // Captured pawn removed
    }

    #[test]
    fn test_right_en_passant_valid() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E5)
            .put_piece(BLACK_PAWN, F5)
            .set_en_passant(F5);

        let result = get_move_white_right_en_passant(&position, E5);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(F6, WHITE_PAWN));
        assert!(!new_position.is_occupied(E5));
        assert!(!new_position.is_occupied(F5)); // Captured pawn removed
    }

    #[test]
    fn test_en_passant_none() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E5)
            .put_piece(BLACK_PAWN, D5)
            .put_piece(BLACK_PAWN, F5);
        // en_passant is None by default

        let left_result = get_move_white_left_en_passant(&position, E5);
        let right_result = get_move_white_right_en_passant(&position, E5);

        assert!(left_result.is_none());
        assert!(right_result.is_none());
    }

    #[test]
    fn test_en_passant_not_adjacent() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E5)
            .set_en_passant(C5); // Not adjacent

        let left_result = get_move_white_left_en_passant(&position, E5);
        let right_result = get_move_white_right_en_passant(&position, E5);

        assert!(left_result.is_none());
        assert!(right_result.is_none());
    }

    #[test]
    fn test_en_passant_edge_cases() {
        // A-file can't en passant left
        let position = Position::default()
            .put_piece(WHITE_PAWN, A5)
            .set_en_passant(B5);

        let a_left = get_move_white_left_en_passant(&position, A5);
        assert!(a_left.is_none());

        // H-file can't en passant right
        let position2 = Position::default()
            .put_piece(WHITE_PAWN, H5)
            .set_en_passant(G5);

        let h_right = get_move_white_right_en_passant(&position2, H5);
        assert!(h_right.is_none());
    }
}

#[cfg(test)]
mod test_black_en_passant {
    use super::*;

    #[test]
    fn test_left_en_passant_valid() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E4)
            .put_piece(WHITE_PAWN, D4)
            .set_en_passant(D4);

        let result = get_move_black_left_en_passant(&position, E4);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(D3, BLACK_PAWN));
        assert!(!new_position.is_occupied(E4));
        assert!(!new_position.is_occupied(D4)); // Captured pawn removed
    }

    #[test]
    fn test_right_en_passant_valid() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E4)
            .put_piece(WHITE_PAWN, F4)
            .set_en_passant(F4);

        let result = get_move_black_right_en_passant(&position, E4);

        assert!(result.is_some());
        let new_position = result.unwrap();
        assert!(new_position.is_occupied_by_piece(F3, BLACK_PAWN));
        assert!(!new_position.is_occupied(E4));
        assert!(!new_position.is_occupied(F4)); // Captured pawn removed
    }
}

#[cfg(test)]
mod test_white_promotions {
    use super::*;

    #[test]
    fn test_forward_promotion() {
        let position = Position::default().put_piece(WHITE_PAWN, E7);

        let results = get_moves_white_promotion(&position, E7);

        assert_eq!(results.len(), 4);

        // Check each promotion piece
        let has_queen = results
            .iter()
            .any(|p| p.is_occupied_by_piece(E8, WHITE_QUEEN));
        let has_rook = results
            .iter()
            .any(|p| p.is_occupied_by_piece(E8, WHITE_ROOK));
        let has_bishop = results
            .iter()
            .any(|p| p.is_occupied_by_piece(E8, WHITE_BISHOP));
        let has_knight = results
            .iter()
            .any(|p| p.is_occupied_by_piece(E8, WHITE_KNIGHT));

        assert!(has_queen);
        assert!(has_rook);
        assert!(has_bishop);
        assert!(has_knight);

        // Original pawn should be gone in all cases
        for position in &results {
            assert!(!position.is_occupied(E7));
        }
    }

    #[test]
    fn test_forward_promotion_blocked() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E7)
            .put_piece(BLACK_KNIGHT, E8);

        let results = get_moves_white_promotion(&position, E7);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_not_from_row_7() {
        let position = Position::default();

        for row in [E1, E2, E3, E4, E5, E6, E8] {
            let pos = position.put_piece(WHITE_PAWN, row);
            let results = get_moves_white_promotion(&pos, row);
            assert_eq!(results.len(), 0);
        }
    }

    #[test]
    fn test_left_capture_promotion() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E7)
            .put_piece(BLACK_KNIGHT, D8);

        let results = get_moves_white_promotion_left_capture(&position, E7);

        assert_eq!(results.len(), 4);

        // All should capture on D8
        for position in &results {
            assert!(!position.is_occupied(E7));
            assert!(!position.is_occupied_by_piece(D8, BLACK_KNIGHT));
        }

        let has_queen = results
            .iter()
            .any(|p| p.is_occupied_by_piece(D8, WHITE_QUEEN));
        let has_rook = results
            .iter()
            .any(|p| p.is_occupied_by_piece(D8, WHITE_ROOK));
        let has_bishop = results
            .iter()
            .any(|p| p.is_occupied_by_piece(D8, WHITE_BISHOP));
        let has_knight = results
            .iter()
            .any(|p| p.is_occupied_by_piece(D8, WHITE_KNIGHT));

        assert!(has_queen);
        assert!(has_rook);
        assert!(has_bishop);
        assert!(has_knight);
    }

    #[test]
    fn test_right_capture_promotion() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E7)
            .put_piece(BLACK_KNIGHT, F8);

        let results = get_moves_white_promotion_right_capture(&position, E7);

        assert_eq!(results.len(), 4);

        for position in &results {
            assert!(!position.is_occupied(E7));
            assert!(!position.is_occupied_by_piece(F8, BLACK_KNIGHT));
        }
    }

    #[test]
    fn test_capture_promotion_empty_square() {
        let position = Position::default().put_piece(WHITE_PAWN, E7);

        let left_results = get_moves_white_promotion_left_capture(&position, E7);
        let right_results = get_moves_white_promotion_right_capture(&position, E7);

        assert_eq!(left_results.len(), 0);
        assert_eq!(right_results.len(), 0);
    }

    #[test]
    fn test_capture_promotion_own_piece() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E7)
            .put_piece(WHITE_KNIGHT, D8)
            .put_piece(WHITE_BISHOP, F8);

        let left_results = get_moves_white_promotion_left_capture(&position, E7);
        let right_results = get_moves_white_promotion_right_capture(&position, E7);

        assert_eq!(left_results.len(), 0);
        assert_eq!(right_results.len(), 0);
    }
}

#[cfg(test)]
mod test_black_promotions {
    use super::*;

    #[test]
    fn test_forward_promotion() {
        let position = Position::default().put_piece(BLACK_PAWN, E2);

        let results = get_moves_black_promotion(&position, E2);

        assert_eq!(results.len(), 4);

        let has_queen = results
            .iter()
            .any(|p| p.is_occupied_by_piece(E1, BLACK_QUEEN));
        let has_rook = results
            .iter()
            .any(|p| p.is_occupied_by_piece(E1, BLACK_ROOK));
        let has_bishop = results
            .iter()
            .any(|p| p.is_occupied_by_piece(E1, BLACK_BISHOP));
        let has_knight = results
            .iter()
            .any(|p| p.is_occupied_by_piece(E1, BLACK_KNIGHT));

        assert!(has_queen);
        assert!(has_rook);
        assert!(has_bishop);
        assert!(has_knight);

        for position in &results {
            assert!(!position.is_occupied(E2));
        }
    }

    #[test]
    fn test_left_capture_promotion() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E2)
            .put_piece(WHITE_KNIGHT, D1);

        let results = get_moves_black_promotion_left_capture(&position, E2);

        assert_eq!(results.len(), 4);

        for position in &results {
            assert!(!position.is_occupied(E2));
            assert!(!position.is_occupied_by_piece(D1, WHITE_KNIGHT));
        }
    }

    #[test]
    fn test_right_capture_promotion() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E2)
            .put_piece(WHITE_KNIGHT, F1);

        let results = get_moves_black_promotion_right_capture(&position, E2);

        assert_eq!(results.len(), 4);

        for position in &results {
            assert!(!position.is_occupied(E2));
            assert!(!position.is_occupied_by_piece(F1, WHITE_KNIGHT));
        }
    }
}

#[cfg(test)]
mod test_aggregate_functions {
    use super::*;

    #[test]
    fn test_get_possible_white_moves_basic() {
        let position = Position::default().put_piece(WHITE_PAWN, E2);

        let moves = get_possible_white_moves(&position, E2);

        // Should have forward and two-forward moves
        assert_eq!(moves.len(), 2);

        let has_e3 = moves.iter().any(|p| p.is_occupied_by_piece(E3, WHITE_PAWN));
        let has_e4 = moves.iter().any(|p| p.is_occupied_by_piece(E4, WHITE_PAWN));

        assert!(has_e3);
        assert!(has_e4);
    }

    #[test]
    fn test_get_possible_white_moves_with_captures() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E4)
            .put_piece(BLACK_PAWN, D5)
            .put_piece(BLACK_PAWN, F5);

        let moves = get_possible_white_moves(&position, E4);

        // Forward, left capture, right capture
        assert_eq!(moves.len(), 3);

        let has_forward = moves.iter().any(|p| p.is_occupied_by_piece(E5, WHITE_PAWN));
        let has_left_capture = moves.iter().any(|p| p.is_occupied_by_piece(D5, WHITE_PAWN));
        let has_right_capture = moves.iter().any(|p| p.is_occupied_by_piece(F5, WHITE_PAWN));

        assert!(has_forward);
        assert!(has_left_capture);
        assert!(has_right_capture);
    }

    #[test]
    fn test_get_possible_white_moves_promotion() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E7)
            .put_piece(BLACK_KNIGHT, D8);

        let moves = get_possible_white_moves(&position, E7);

        // 4 forward promotions + 4 left capture promotions
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_get_possible_white_moves_en_passant() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E5)
            .put_piece(BLACK_PAWN, D5)
            .set_en_passant(D5);

        let moves = get_possible_white_moves(&position, E5);

        // Forward + en passant
        assert_eq!(moves.len(), 2);

        let has_en_passant = moves
            .iter()
            .any(|p| p.is_occupied_by_piece(D6, WHITE_PAWN) && !p.is_occupied(D5));

        assert!(has_en_passant);
    }

    #[test]
    fn test_get_possible_black_moves_basic() {
        let position = Position::default().put_piece(BLACK_PAWN, E7);

        let moves = get_possible_black_moves(&position, E7);

        // Should have forward and two-forward moves
        assert_eq!(moves.len(), 2);

        let has_e6 = moves.iter().any(|p| p.is_occupied_by_piece(E6, BLACK_PAWN));
        let has_e5 = moves.iter().any(|p| p.is_occupied_by_piece(E5, BLACK_PAWN));

        assert!(has_e6);
        assert!(has_e5);
    }

    #[test]
    fn test_get_possible_black_moves_promotion() {
        let position = Position::default()
            .put_piece(BLACK_PAWN, E2)
            .put_piece(WHITE_KNIGHT, D1)
            .put_piece(WHITE_BISHOP, F1);

        let moves = get_possible_black_moves(&position, E2);

        // 4 forward + 4 left capture + 4 right capture promotions
        assert_eq!(moves.len(), 12);
    }
}

#[cfg(test)]
mod test_public_interface {
    use super::*;

    #[test]
    fn test_get_possible_moves_white_pawn() {
        let position = Position::default().put_piece(WHITE_PAWN, E2);

        let moves = get_possible_moves(&position, E2, WHITE_PAWN);

        // Should have forward and two-forward moves
        assert_eq!(moves.len(), 2);

        let has_e3 = moves.iter().any(|p| p.is_occupied_by_piece(E3, WHITE_PAWN));
        let has_e4 = moves.iter().any(|p| p.is_occupied_by_piece(E4, WHITE_PAWN));

        assert!(has_e3);
        assert!(has_e4);
    }

    #[test]
    fn test_get_possible_moves_black_pawn() {
        let position = Position::default().put_piece(BLACK_PAWN, E7);

        let moves = get_possible_moves(&position, E7, BLACK_PAWN);

        // Should have forward and two-forward moves
        assert_eq!(moves.len(), 2);

        let has_e6 = moves.iter().any(|p| p.is_occupied_by_piece(E6, BLACK_PAWN));
        let has_e5 = moves.iter().any(|p| p.is_occupied_by_piece(E5, BLACK_PAWN));

        assert!(has_e6);
        assert!(has_e5);
    }

    #[test]
    fn test_get_possible_moves_non_pawn() {
        let position = Position::default().put_piece(WHITE_KNIGHT, E4);

        let moves = get_possible_moves(&position, E4, WHITE_KNIGHT);

        // Should return empty vector for non-pawn pieces
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_get_possible_moves_complex_position() {
        let position = Position::default()
            .put_piece(WHITE_PAWN, E5)
            .put_piece(BLACK_PAWN, D5)
            .put_piece(BLACK_PAWN, F5)
            .put_piece(BLACK_KNIGHT, E6)
            .set_en_passant(D5);

        let moves = get_possible_moves(&position, E5, WHITE_PAWN);

        // White pawn on E5 with:
        // - Black pawns on D5 and F5 (same rank, so no normal captures)
        // - Black knight on E6 (blocks forward move)
        // - En passant available on D5
        // Result: Only en passant capture is possible
        assert_eq!(moves.len(), 1);

        let has_en_passant = moves
            .iter()
            .any(|p| p.is_occupied_by_piece(D6, WHITE_PAWN) && !p.is_occupied(D5));

        assert!(has_en_passant);
    }
}
