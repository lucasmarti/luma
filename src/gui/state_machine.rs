use crate::{
    engine::{
        chess_moves::ChessMove,
        directions::squares::Square,
        piece::{Color, Piece},
    },
    gui::ui_element::UIEvent,
};
pub enum GameState {
    NoGame,
    Computer,
    Player(SquareSelected),
    GameOver,
}
#[derive(Debug)]
pub enum SquareSelected {
    No(NoSquareSelectedData),
    From(FromSquareSelectedData),
    Promotion(PromotionSquareSelectedData),
}
#[derive(Debug)]
pub struct NoSquareSelectedData {
    pub possible_moves: Vec<ChessMove>,
}
#[derive(Debug)]
pub struct FromSquareSelectedData {
    pub from: Square,
    pub possible_moves: Vec<ChessMove>,
    pub possible_moves_from: Vec<ChessMove>,
}

#[derive(Debug)]
pub struct PromotionSquareSelectedData {
    pub possible_promotion_moves: Vec<ChessMove>,
}

impl PromotionSquareSelectedData {
    pub fn from(possible_promotion_moves: Vec<ChessMove>) -> Self {
        PromotionSquareSelectedData {
            possible_promotion_moves,
        }
    }
}
#[derive(Debug)]
pub struct SelectToSquareFunctionData {
    pub possible_moves: Vec<ChessMove>,
    pub possible_moves_from: Vec<ChessMove>,
    pub to: Square,
}
impl SelectToSquareFunctionData {
    pub fn from(data: &FromSquareSelectedData, to: Square) -> Self {
        SelectToSquareFunctionData {
            possible_moves: data.possible_moves.clone(),
            possible_moves_from: data.possible_moves_from.clone(),
            to,
        }
    }
}
#[derive(Debug)]
pub struct PromoteFunctionData {
    pub possible_promotion_moves: Vec<ChessMove>,
    pub piece: Piece,
}

impl PromoteFunctionData {
    pub fn from(data: &PromotionSquareSelectedData, piece: Piece) -> Self {
        PromoteFunctionData {
            possible_promotion_moves: data.possible_promotion_moves.clone(),
            piece,
        }
    }
}
pub enum StateFunction {
    NewGameAs(Color),
    SelectFromSquare(SelectFromSquareFunctionData),
    SelectToSquare(SelectToSquareFunctionData),
    Promote(PromoteFunctionData),
    TurnBoard,
}

pub struct SelectFromSquareFunctionData {
    pub possible_moves: Vec<ChessMove>,
    pub from: Square,
}
impl SelectFromSquareFunctionData {
    pub fn from(no_square_selected_data: &NoSquareSelectedData, from: Square) -> Self {
        SelectFromSquareFunctionData {
            possible_moves: no_square_selected_data.possible_moves.clone(),
            from,
        }
    }
}

pub fn get_function(event: UIEvent, state: &GameState) -> Option<StateFunction> {
    match event {
        UIEvent::TurnBoard => Some(StateFunction::TurnBoard),
        UIEvent::NewGameAs(color) => Some(StateFunction::NewGameAs(color)),
        UIEvent::PromoteTo(piece) => match state {
            GameState::Player(SquareSelected::Promotion(data)) => Some(StateFunction::Promote(
                PromoteFunctionData::from(data, piece),
            )),
            _ => None,
        },
        UIEvent::Square(square) => match state {
            GameState::Player(player_uistate) => match player_uistate {
                SquareSelected::No(data) => Some(StateFunction::SelectFromSquare(
                    SelectFromSquareFunctionData::from(data, square),
                )),
                SquareSelected::From(data) => Some(StateFunction::SelectToSquare(
                    SelectToSquareFunctionData::from(data, square),
                )),
                _ => None,
            },
            _ => None,
        },
    }
}
