use crate::{
    engine::{
        chess_moves::{ChessMove, MoveType},
        directions::squares::Square,
        piece::{Color, Piece},
    },
    gui::ui_element::UIEvent,
};
pub enum GameState {
    Computer,
    Player(PlayerUIState),
    GameOver,
}
#[derive(Debug)]
pub enum PlayerUIState {
    NoSquareSelected,
    FromSquareSelected(FromSquareSelectedData),
    PromotionSquareSelected(PromotionSquareSelectedData),
}
#[derive(Debug)]
pub struct FromSquareSelectedData {
    pub square: Square,
    pub possible_moves: Vec<ChessMove>,
}
#[derive(Debug)]
pub struct SelectToSquareData {
    pub from_square: Square,
    pub possible_moves: Vec<ChessMove>,
    pub to_square: Square,
}
impl SelectToSquareData {
    pub fn from(data: &FromSquareSelectedData, square: Square) -> Self {
        SelectToSquareData {
            from_square: data.square,
            possible_moves: data.possible_moves.clone(),
            to_square: square,
        }
    }
    pub fn get_chess_move_for_to_square(&self) -> Option<ChessMove> {
        self.possible_moves
            .iter()
            .find(|m| m.to == self.to_square)
            .copied()
    }
}
#[derive(Debug)]

pub struct PromotionSquareSelectedData {
    pub promotion_moves: Vec<ChessMove>,
}
impl PromotionSquareSelectedData {
    pub fn from(data: SelectToSquareData) -> Self {
        PromotionSquareSelectedData {
            promotion_moves: data
                .possible_moves
                .into_iter()
                .filter(|m| {
                    m.move_type == MoveType::Promotion || m.move_type == MoveType::PromotionCapture
                })
                .collect(),
        }
    }
}
pub struct PromoteData {
    pub promotion_moves: Vec<ChessMove>,
    pub promotion_piece: Piece,
}
impl PromoteData {
    pub fn from(data: &PromotionSquareSelectedData, piece: Piece) -> Self {
        PromoteData {
            promotion_moves: data.promotion_moves.clone(),
            promotion_piece: piece,
        }
    }
    pub fn get_chess_move_for_selected_promotion_piece(&self) -> Option<ChessMove> {
        self.promotion_moves
            .iter()
            .find(|m| m.pormotion == Some(self.promotion_piece))
            .copied()
    }
}
pub enum StateFunction {
    NewGameAs(Color),
    SelectFromSquare(Square),
    SelectToSquare(SelectToSquareData),
    Promote(PromoteData),
}

pub fn get_function(event: UIEvent, state: &GameState) -> Option<StateFunction> {
    match event {
        UIEvent::TurnBoardClicked => None,
        UIEvent::NewGameAsButtonClicked(color) => Some(StateFunction::NewGameAs(color)),
        UIEvent::PromoteToButtonClicked(piece) => match state {
            GameState::Player(player_uistate) => match player_uistate {
                PlayerUIState::PromotionSquareSelected(data) => {
                    Some(StateFunction::Promote(PromoteData::from(data, piece)))
                }
                _ => None,
            },
            _ => None,
        },
        UIEvent::SquareClicked(square) => match state {
            GameState::Player(player_uistate) => match player_uistate {
                PlayerUIState::NoSquareSelected => Some(StateFunction::SelectFromSquare(square)),
                PlayerUIState::FromSquareSelected(data) => Some(StateFunction::SelectToSquare(
                    SelectToSquareData::from(data, square),
                )),
                _ => None,
            },
            _ => None,
        },
    }
}
