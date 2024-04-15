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

        // TODO: Change action mapping to use register of actions for a screen with Fn objects
        match action {
            GameSessionAction::MarkBoardField => {
                println!(
                    "Player{} select cell position in format: row col",
                    self.current_player
                );

                // TODO: Properly handle user input for cell position
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Could not read input");

                let position: BoardCellPosition = input.parse().expect("failed to parse position");

                self.board
                    .occupy_cell(position, self.current_player)
                    .expect("failed to occupy");

                if let Some(result) = self.board.check_for_result(self.current_player) {
                    self.show_display();
                    match result {
                        GameResult::Win(winner) => println!("{} is a winner", winner),
                        GameResult::Draw => println!("It's a draw"),
                    }

                    // TODO: Add Win screen with ability to restart
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
