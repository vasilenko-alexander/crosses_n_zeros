mod game_board;

pub use crate::game_board::{BoardCellPosition, Cell, GameBoard};
use std::fmt::Display;

pub enum GameResult {
    Win(Player),
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    PlayerX(char),
    PlayerO(char),
}

impl Player {
    pub fn create_player_x() -> Self {
        Self::PlayerX('X')
    }

    pub fn create_player_o() -> Self {
        Self::PlayerO('O')
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player_mark = match self {
            Player::PlayerX(x) => x,
            Player::PlayerO(o) => o,
        };
        write!(f, "{}", player_mark)
    }
}
