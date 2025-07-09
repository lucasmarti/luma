use crate::{
    chess_moves::ChessMove,
    directions::*,
    piece::{
        BLACK_BISHOP, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
        WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    position::Position,
};

use super::*;

#[test]
fn test_get_move_white_forward() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D2);

    let result = get_move_white_forward(&position, D2);

    assert!(result.is_some());
    if let Some(ChessMove::Progress(progress)) = result {
        assert!(progress.from == D2);
        assert!(progress.to == D3);
        assert!(progress.piece == WHITE_PAWN);
    } else {
        panic!("Expected Progress move");
    }
}

#[test]
fn test_get_move_white_forward_blocked() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D2);
    position = position.put_piece(BLACK_QUEEN, D3);

    let result = get_move_white_forward(&position, D2);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_forward() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D7);

    let result = get_move_black_forward(&position, D7);

    assert!(result.is_some());
    if let Some(ChessMove::Progress(progress)) = result {
        assert!(progress.from == D7);
        assert!(progress.to == D6);
        assert!(progress.piece == BLACK_PAWN);
    } else {
        panic!("Expected Progress move");
    }
}

#[test]
fn test_get_move_black_forward_blocked() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D7);
    position = position.put_piece(WHITE_QUEEN, D6);

    let result = get_move_black_forward(&position, D7);

    assert!(result.is_none());
}

#[test]
fn test_get_move_white_left_capture() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D4);
    position = position.put_piece(BLACK_QUEEN, C5);

    let result = get_move_white_left_capture(&position, D4);

    assert!(result.is_some());
    if let Some(ChessMove::Progress(progress)) = result {
        assert!(progress.from == D4);
        assert!(progress.to == C5);
        assert!(progress.piece == WHITE_PAWN);
    } else {
        panic!("Expected Progress move");
    }
}

#[test]
fn test_get_move_white_right_capture() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D4);
    position = position.put_piece(BLACK_QUEEN, E5);

    let result = get_move_white_right_capture(&position, D4);

    assert!(result.is_some());
    if let Some(ChessMove::Progress(progress)) = result {
        assert!(progress.from == D4);
        assert!(progress.to == E5);
        assert!(progress.piece == WHITE_PAWN);
    } else {
        panic!("Expected Progress move");
    }
}

#[test]
fn test_get_move_white_left_capture_no_piece() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D4);

    let result = get_move_white_left_capture(&position, D4);

    assert!(result.is_none());
}

#[test]
fn test_get_move_white_right_capture_no_piece() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D4);

    let result = get_move_white_right_capture(&position, D4);

    assert!(result.is_none());
}

#[test]
fn test_get_move_white_left_capture_own_piece() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D4);
    position = position.put_piece(WHITE_QUEEN, C5);

    let result = get_move_white_left_capture(&position, D4);

    assert!(result.is_none());
}

#[test]
fn test_get_move_white_right_capture_own_piece() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D4);
    position = position.put_piece(WHITE_QUEEN, E5);

    let result = get_move_white_right_capture(&position, D4);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_left_capture() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D5);
    position = position.put_piece(WHITE_QUEEN, C4);

    let result = get_move_black_left_capture(&position, D5);

    assert!(result.is_some());
    if let Some(ChessMove::Progress(progress)) = result {
        assert!(progress.from == D5);
        assert!(progress.to == C4);
        assert!(progress.piece == BLACK_PAWN);
    } else {
        panic!("Expected Progress move");
    }
}

#[test]
fn test_get_move_black_right_capture() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D5);
    position = position.put_piece(WHITE_QUEEN, E4);

    let result = get_move_black_right_capture(&position, D5);

    assert!(result.is_some());
    if let Some(ChessMove::Progress(progress)) = result {
        assert!(progress.from == D5);
        assert!(progress.to == E4);
        assert!(progress.piece == BLACK_PAWN);
    } else {
        panic!("Expected Progress move");
    }
}

#[test]
fn test_get_move_black_left_capture_no_piece() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D5);

    let result = get_move_black_left_capture(&position, D5);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_right_capture_no_piece() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D5);

    let result = get_move_black_right_capture(&position, D5);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_left_capture_own_piece() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D5);
    position = position.put_piece(BLACK_QUEEN, C4);

    let result = get_move_black_left_capture(&position, D5);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_right_capture_own_piece() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D5);
    position = position.put_piece(BLACK_QUEEN, E4);

    let result = get_move_black_right_capture(&position, D5);

    assert!(result.is_none());
}

#[test]
fn test_get_move_white_left_en_passant() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, E5);
    position = position.put_piece(BLACK_PAWN, D5);
    position.en_passant = Some(D5);

    let result = get_move_white_left_en_passant(&position, E5);

    assert!(result.is_some());
    if let Some(ChessMove::EnPassant(en_passant)) = result {
        assert!(en_passant.from == E5);
        assert!(en_passant.to == D6);
        assert!(en_passant.capture == D5);
        assert!(en_passant.piece == WHITE_PAWN);
    } else {
        panic!("Expected EnPassant move");
    }
}

#[test]
fn test_get_move_white_right_en_passant() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D5);
    position = position.put_piece(BLACK_PAWN, E5);
    position.en_passant = Some(E5);

    let result = get_move_white_right_en_passant(&position, D5);

    assert!(result.is_some());
    if let Some(ChessMove::EnPassant(en_passant)) = result {
        assert!(en_passant.from == D5);
        assert!(en_passant.to == E6);
        assert!(en_passant.capture == E5);
        assert!(en_passant.piece == WHITE_PAWN);
    } else {
        panic!("Expected EnPassant move");
    }
}

#[test]
fn test_get_move_white_left_en_passant_no_en_passant() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, E5);
    position = position.put_piece(BLACK_PAWN, D5);
    position.en_passant = None;

    let result = get_move_white_left_en_passant(&position, E5);

    assert!(result.is_none());
}

#[test]
fn test_get_move_white_right_en_passant_wrong_square() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D5);
    position = position.put_piece(BLACK_PAWN, E5);
    position.en_passant = Some(C5); // Wrong square

    let result = get_move_white_right_en_passant(&position, D5);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_left_en_passant() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, E4);
    position = position.put_piece(WHITE_PAWN, D4);
    position.en_passant = Some(D4);

    let result = get_move_black_left_en_passant(&position, E4);

    assert!(result.is_some());
    if let Some(ChessMove::EnPassant(en_passant)) = result {
        assert!(en_passant.from == E4);
        assert!(en_passant.to == D3);
        assert!(en_passant.capture == D4);
        assert!(en_passant.piece == BLACK_PAWN);
    } else {
        panic!("Expected EnPassant move");
    }
}

#[test]
fn test_get_move_black_right_en_passant() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D4);
    position = position.put_piece(WHITE_PAWN, E4);
    position.en_passant = Some(E4);

    let result = get_move_black_right_en_passant(&position, D4);

    assert!(result.is_some());
    if let Some(ChessMove::EnPassant(en_passant)) = result {
        assert!(en_passant.from == D4);
        assert!(en_passant.to == E3);
        assert!(en_passant.capture == E4);
        assert!(en_passant.piece == BLACK_PAWN);
    } else {
        panic!("Expected EnPassant move");
    }
}

#[test]
fn test_get_move_black_left_en_passant_no_en_passant() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, E4);
    position = position.put_piece(WHITE_PAWN, D4);
    position.en_passant = None;

    let result = get_move_black_left_en_passant(&position, E4);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_right_en_passant_wrong_square() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D4);
    position = position.put_piece(WHITE_PAWN, E4);
    position.en_passant = Some(C4); // Wrong square

    let result = get_move_black_right_en_passant(&position, D4);

    assert!(result.is_none());
}

#[test]
fn test_get_moves_white_promotion() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D7);

    let result = get_moves_white_promotion(&position, D7);

    assert_eq!(result.len(), 4);

    // Check that all promotion moves are present
    let mut found_queen = false;
    let mut found_rook = false;
    let mut found_bishop = false;
    let mut found_knight = false;

    for chess_move in result {
        if let ChessMove::Promotion(promotion) = chess_move {
            assert!(promotion.from == D7);
            assert!(promotion.to == D8);
            assert!(promotion.piece == WHITE_PAWN);

            match promotion.new_piece {
                WHITE_QUEEN => found_queen = true,
                WHITE_ROOK => found_rook = true,
                WHITE_BISHOP => found_bishop = true,
                WHITE_KNIGHT => found_knight = true,
                _ => panic!("Unexpected promotion piece"),
            }
        } else {
            panic!("Expected Promotion move");
        }
    }

    assert!(found_queen && found_rook && found_bishop && found_knight);
}

#[test]
fn test_get_moves_black_promotion() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D2);

    let result = get_moves_black_promotion(&position, D2);

    assert_eq!(result.len(), 4);

    // Check that all promotion moves are present
    let mut found_queen = false;
    let mut found_rook = false;
    let mut found_bishop = false;
    let mut found_knight = false;

    for chess_move in result {
        if let ChessMove::Promotion(promotion) = chess_move {
            assert!(promotion.from == D2);
            assert!(promotion.to == D1);
            assert!(promotion.piece == BLACK_PAWN);

            match promotion.new_piece {
                BLACK_QUEEN => found_queen = true,
                BLACK_ROOK => found_rook = true,
                BLACK_BISHOP => found_bishop = true,
                BLACK_KNIGHT => found_knight = true,
                _ => panic!("Unexpected promotion piece"),
            }
        } else {
            panic!("Expected Promotion move");
        }
    }

    assert!(found_queen && found_rook && found_bishop && found_knight);
}

#[test]
fn test_get_moves_white_promotion_blocked() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D7);
    position = position.put_piece(BLACK_QUEEN, D8); // Block the promotion square

    let result = get_moves_white_promotion(&position, D7);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_get_moves_black_promotion_blocked() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D2);
    position = position.put_piece(WHITE_QUEEN, D1); // Block the promotion square

    let result = get_moves_black_promotion(&position, D2);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_get_move_white_two_forward() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D2);

    let result = get_move_white_two_forward(&position, D2);

    assert!(result.is_some());
    if let Some(ChessMove::Progress(progress)) = result {
        assert!(progress.from == D2);
        assert!(progress.to == D4);
        assert!(progress.piece == WHITE_PAWN);
    } else {
        panic!("Expected Progress move");
    }
}

#[test]
fn test_get_move_white_two_forward_blocked_one_square() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D2);
    position = position.put_piece(BLACK_QUEEN, D3); // Block one square ahead

    let result = get_move_white_two_forward(&position, D2);

    assert!(result.is_none());
}

#[test]
fn test_get_move_white_two_forward_blocked_two_squares() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D2);
    position = position.put_piece(BLACK_QUEEN, D4); // Block two squares ahead

    let result = get_move_white_two_forward(&position, D2);

    assert!(result.is_none());
}

#[test]
fn test_get_move_white_two_forward_not_starting_position() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D3); // Not in starting position

    let result = get_move_white_two_forward(&position, D3);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_two_forward() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D7);

    let result = get_move_black_two_forward(&position, D7);

    assert!(result.is_some());
    if let Some(ChessMove::Progress(progress)) = result {
        assert!(progress.from == D7);
        assert!(progress.to == D5);
        assert!(progress.piece == BLACK_PAWN);
    } else {
        panic!("Expected Progress move");
    }
}

#[test]
fn test_get_move_black_two_forward_blocked_one_square() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D7);
    position = position.put_piece(WHITE_QUEEN, D6); // Block one square ahead

    let result = get_move_black_two_forward(&position, D7);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_two_forward_blocked_two_squares() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D7);
    position = position.put_piece(WHITE_QUEEN, D5); // Block two squares ahead

    let result = get_move_black_two_forward(&position, D7);

    assert!(result.is_none());
}

#[test]
fn test_get_move_black_two_forward_not_starting_position() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D6); // Not in starting position

    let result = get_move_black_two_forward(&position, D6);

    assert!(result.is_none());
}

#[test]
fn test_get_moves_white_promotion_left_capture() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D7);
    position = position.put_piece(BLACK_QUEEN, C8); // Enemy piece to capture

    let result = get_moves_white_promotion_left_capture(&position, D7);

    assert_eq!(result.len(), 4);

    // Check that all promotion capture moves are present
    let mut found_queen = false;
    let mut found_rook = false;
    let mut found_bishop = false;
    let mut found_knight = false;

    for chess_move in result {
        if let ChessMove::Promotion(promotion) = chess_move {
            assert!(promotion.from == D7);
            assert!(promotion.to == C8);
            assert!(promotion.piece == WHITE_PAWN);

            match promotion.new_piece {
                WHITE_QUEEN => found_queen = true,
                WHITE_ROOK => found_rook = true,
                WHITE_BISHOP => found_bishop = true,
                WHITE_KNIGHT => found_knight = true,
                _ => panic!("Unexpected promotion piece"),
            }
        } else {
            panic!("Expected Promotion move");
        }
    }

    assert!(found_queen && found_rook && found_bishop && found_knight);
}

#[test]
fn test_get_moves_white_promotion_right_capture() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D7);
    position = position.put_piece(BLACK_QUEEN, E8); // Enemy piece to capture

    let result = get_moves_white_promotion_right_capture(&position, D7);

    assert_eq!(result.len(), 4);

    // Check that all promotion capture moves are present
    let mut found_queen = false;
    let mut found_rook = false;
    let mut found_bishop = false;
    let mut found_knight = false;

    for chess_move in result {
        if let ChessMove::Promotion(promotion) = chess_move {
            assert!(promotion.from == D7);
            assert!(promotion.to == E8);
            assert!(promotion.piece == WHITE_PAWN);

            match promotion.new_piece {
                WHITE_QUEEN => found_queen = true,
                WHITE_ROOK => found_rook = true,
                WHITE_BISHOP => found_bishop = true,
                WHITE_KNIGHT => found_knight = true,
                _ => panic!("Unexpected promotion piece"),
            }
        } else {
            panic!("Expected Promotion move");
        }
    }

    assert!(found_queen && found_rook && found_bishop && found_knight);
}

#[test]
fn test_get_moves_black_promotion_left_capture() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D2);
    position = position.put_piece(WHITE_QUEEN, C1); // Enemy piece to capture

    let result = get_moves_black_promotion_left_capture(&position, D2);

    assert_eq!(result.len(), 4);

    // Check that all promotion capture moves are present
    let mut found_queen = false;
    let mut found_rook = false;
    let mut found_bishop = false;
    let mut found_knight = false;

    for chess_move in result {
        if let ChessMove::Promotion(promotion) = chess_move {
            assert!(promotion.from == D2);
            assert!(promotion.to == C1);
            assert!(promotion.piece == BLACK_PAWN);

            match promotion.new_piece.typ {
                crate::piece::Typ::Queen => found_queen = true,
                crate::piece::Typ::Rook => found_rook = true,
                crate::piece::Typ::Bishop => found_bishop = true,
                crate::piece::Typ::Knight => found_knight = true,
                _ => panic!("Unexpected promotion piece type"),
            }
        } else {
            panic!("Expected Promotion move");
        }
    }

    assert!(found_queen && found_rook && found_bishop && found_knight);
}

#[test]
fn test_get_moves_black_promotion_right_capture() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D2);
    position = position.put_piece(WHITE_QUEEN, E1); // Enemy piece to capture

    let result = get_moves_black_promotion_right_capture(&position, D2);

    assert_eq!(result.len(), 4);

    // Check that all promotion capture moves are present
    let mut found_queen = false;
    let mut found_rook = false;
    let mut found_bishop = false;
    let mut found_knight = false;

    for chess_move in result {
        if let ChessMove::Promotion(promotion) = chess_move {
            assert!(promotion.from == D2);
            assert!(promotion.to == E1);
            assert!(promotion.piece == BLACK_PAWN);

            match promotion.new_piece.typ {
                crate::piece::Typ::Queen => found_queen = true,
                crate::piece::Typ::Rook => found_rook = true,
                crate::piece::Typ::Bishop => found_bishop = true,
                crate::piece::Typ::Knight => found_knight = true,
                _ => panic!("Unexpected promotion piece type"),
            }
        } else {
            panic!("Expected Promotion move");
        }
    }

    assert!(found_queen && found_rook && found_bishop && found_knight);
}

#[test]
fn test_get_moves_white_promotion_left_capture_no_enemy() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D7);
    // No enemy piece on C8

    let result = get_moves_white_promotion_left_capture(&position, D7);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_get_moves_white_promotion_right_capture_own_piece() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D7);
    position = position.put_piece(WHITE_QUEEN, E8); // Own piece on E8

    let result = get_moves_white_promotion_right_capture(&position, D7);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_get_moves_black_promotion_left_capture_no_enemy() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D2);
    // No enemy piece on C1

    let result = get_moves_black_promotion_left_capture(&position, D2);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_get_moves_black_promotion_right_capture_own_piece() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D2);
    position = position.put_piece(BLACK_QUEEN, E1); // Own piece on E1

    let result = get_moves_black_promotion_right_capture(&position, D2);

    assert_eq!(result.len(), 0);
}

// Integration tests for get_possible_white_moves
#[test]
fn test_get_possible_white_moves_starting_position() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D2);

    let moves = get_possible_white_moves(&position, D2);

    // Should contain: forward (D3) and two-forward (D4)
    assert_eq!(moves.len(), 2);

    // Verify specific moves are present
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == D3)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == D4)));
}

#[test]
fn test_get_possible_white_moves_with_captures() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D4);
    position = position.put_piece(BLACK_QUEEN, C5); // Left capture
    position = position.put_piece(BLACK_ROOK, E5); // Right capture

    let moves = get_possible_white_moves(&position, D4);

    // Should contain: forward (D5), left capture (C5), right capture (E5)
    assert_eq!(moves.len(), 3);

    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == D5)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == C5)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == E5)));
}

#[test]
fn test_get_possible_white_moves_with_en_passant() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D5);
    position = position.put_piece(BLACK_PAWN, C5); // Left en passant target
    position = position.put_piece(BLACK_PAWN, E5); // Right en passant target
    position.en_passant = Some(C5);

    let moves = get_possible_white_moves(&position, D5);

    // Should contain: forward (D6) and left en passant
    assert_eq!(moves.len(), 2);

    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == D6)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::EnPassant(ep) if ep.to == C6 && ep.capture == C5)));
}

#[test]
fn test_get_possible_white_moves_promotion_scenario() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D7);
    position = position.put_piece(BLACK_QUEEN, C8); // Left capture promotion
    position = position.put_piece(BLACK_ROOK, E8); // Right capture promotion

    let moves = get_possible_white_moves(&position, D7);

    // Should contain: 4 forward promotions + 4 left capture promotions + 4 right capture promotions = 12 moves
    assert_eq!(moves.len(), 12);

    // All should be promotion moves
    assert!(moves.iter().all(|m| matches!(m, ChessMove::Promotion(_))));

    // Check we have moves to all three squares (D8, C8, E8)
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Promotion(p) if p.to == D8)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Promotion(p) if p.to == C8)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Promotion(p) if p.to == E8)));
}

#[test]
fn test_get_possible_white_moves_blocked() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, D4);
    position = position.put_piece(BLACK_QUEEN, D5); // Block forward movement
    position = position.put_piece(WHITE_ROOK, C5); // Own piece on left diagonal
    position = position.put_piece(WHITE_BISHOP, E5); // Own piece on right diagonal

    let moves = get_possible_white_moves(&position, D4);

    // Should have no moves available
    assert_eq!(moves.len(), 0);
}

// Integration tests for get_possible_black_moves
#[test]
fn test_get_possible_black_moves_starting_position() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D7);

    let moves = get_possible_black_moves(&position, D7);

    // Should contain: forward (D6) and two-forward (D5)
    assert_eq!(moves.len(), 2);

    // Verify specific moves are present
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == D6)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == D5)));
}

#[test]
fn test_get_possible_black_moves_with_captures() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D5);
    position = position.put_piece(WHITE_QUEEN, C4); // Left capture
    position = position.put_piece(WHITE_ROOK, E4); // Right capture

    let moves = get_possible_black_moves(&position, D5);

    // Should contain: forward (D4), left capture (C4), right capture (E4)
    assert_eq!(moves.len(), 3);

    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == D4)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == C4)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == E4)));
}

#[test]
fn test_get_possible_black_moves_with_en_passant() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D4);
    position = position.put_piece(WHITE_PAWN, C4); // Left en passant target
    position = position.put_piece(WHITE_PAWN, E4); // Right en passant target
    position.en_passant = Some(E4);

    let moves = get_possible_black_moves(&position, D4);

    // Should contain: forward (D3) and right en passant
    assert_eq!(moves.len(), 2);

    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == D3)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::EnPassant(ep) if ep.to == E3 && ep.capture == E4)));
}

#[test]
fn test_get_possible_black_moves_promotion_scenario() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D2);
    position = position.put_piece(WHITE_QUEEN, C1); // Left capture promotion
    position = position.put_piece(WHITE_ROOK, E1); // Right capture promotion

    let moves = get_possible_black_moves(&position, D2);

    // Should contain: 4 forward promotions + 4 left capture promotions + 4 right capture promotions = 12 moves
    assert_eq!(moves.len(), 12);

    // All should be promotion moves
    assert!(moves.iter().all(|m| matches!(m, ChessMove::Promotion(_))));

    // Check we have moves to all three squares (D1, C1, E1)
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Promotion(p) if p.to == D1)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Promotion(p) if p.to == C1)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Promotion(p) if p.to == E1)));
}

#[test]
fn test_get_possible_black_moves_blocked() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, D5);
    position = position.put_piece(WHITE_QUEEN, D4); // Block forward movement
    position = position.put_piece(BLACK_ROOK, C4); // Own piece on left diagonal
    position = position.put_piece(BLACK_BISHOP, E4); // Own piece on right diagonal

    let moves = get_possible_black_moves(&position, D5);

    // Should have no moves available
    assert_eq!(moves.len(), 0);
}

#[test]
fn test_get_possible_white_moves_complex_scenario() {
    let mut position = Position::default();
    position = position.put_piece(WHITE_PAWN, E5);
    position = position.put_piece(BLACK_PAWN, D5); // Left en passant target
    position = position.put_piece(BLACK_QUEEN, F6); // Right capture
    position.en_passant = Some(D5);

    let moves = get_possible_white_moves(&position, E5);

    // Should contain: forward (E6), left en passant, right capture (F6)
    assert_eq!(moves.len(), 3);

    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == E6)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::EnPassant(ep) if ep.to == D6 && ep.capture == D5)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == F6)));
}

#[test]
fn test_get_possible_black_moves_complex_scenario() {
    let mut position = Position::default();
    position = position.put_piece(BLACK_PAWN, E4);
    position = position.put_piece(WHITE_PAWN, F4); // Right en passant target
    position = position.put_piece(WHITE_QUEEN, D3); // Left capture
    position.en_passant = Some(F4);

    let moves = get_possible_black_moves(&position, E4);

    // Should contain: forward (E3), right en passant, left capture (D3)
    assert_eq!(moves.len(), 3);

    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == E3)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::EnPassant(ep) if ep.to == F3 && ep.capture == F4)));
    assert!(moves
        .iter()
        .any(|m| matches!(m, ChessMove::Progress(p) if p.to == D3)));
}
