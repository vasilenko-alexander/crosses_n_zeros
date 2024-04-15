use std::io;

use super::{MainScreen, Screen};
use crate::actions::{errors::ActionParseError, GameSessionAction, GameStateAction};
use game_logic::{BoardCellPosition, GameBoard, GameResult, Player};

#[derive(Copy, Clone)]
pub struct GameSessionScreen {
    board: GameBoard,
    current_player: Player,
}

impl GameSessionScreen {
    pub fn new(board: GameBoard, player: Player) -> Self {
        Self {
            board,
            current_player: player,
        }
    }
}

impl Screen for GameSessionScreen {
    fn show_display(&self) {
        println!("{}", self.board);
    }

    fn show_actions(&self) {
        println!("1. Occupy board cell");
        println!("2. Admit Defeat");
        println!("3. Quit");
    }

    fn handle_action(&mut self, action: String) -> Result<GameStateAction, ActionParseError> {
        let action = action.parse::<GameSessionAction>()?;

        match action {
            GameSessionAction::MarkBoardField => {
                println!(
                    "Player{} select cell position in format: row col",
                    self.current_player
                );

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Could not read input");

                let position: BoardCellPosition = input.parse().expect("failed to parse position");

                self.board
                    .occupy_cell(position, self.current_player)
                    .expect("failed to occupy");

                if let Some(result) = self.board.check_for_result(self.current_player) {
                    match result {
                        GameResult::Win(winner) => println!("Player{} is a winner", winner),
                        GameResult::Draw => println!("It's a draw"),
                    }

                    return Ok(GameStateAction::ChangeScreen(Box::new(MainScreen {})));
                }

                let next_player = if self.current_player == Player::create_player_x() {
                    Player::create_player_o()
                } else {
                    Player::create_player_x()
                };

                Ok(GameStateAction::ChangeScreen(Box::new(Self::new(
                    self.board,
                    next_player,
                ))))
            }
            GameSessionAction::AdmitDefeat => {
                Ok(GameStateAction::ChangeScreen(Box::new(MainScreen {})))
            }
            GameSessionAction::Quit => Ok(GameStateAction::Quit),
        }
    }
}
