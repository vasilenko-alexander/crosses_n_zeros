use crate::actions::{errors::ActionParseError, GameSessionAction, GameStateAction};

use super::{MainScreen, Screen};

#[derive(Copy, Clone)]
pub struct GameSessionScreen {
    
}

impl Screen for GameSessionScreen {
    fn show_display(&self) {
        let first_player = 'X';
        let second_player = 'O';
        println!("{first_player}|{first_player}|{second_player}");
        println!("{second_player}|{second_player}|{first_player}");
        println!("{first_player}|{second_player}|{first_player}");
    }

    fn show_actions(&self) {
        println!("1. Occupy board field");
        println!("2. Admit Defeat");
        println!("3. Quit");
    }

    fn handle_action(&self, action: String) -> Result<GameStateAction, ActionParseError> {
        let action = action.parse::<GameSessionAction>()?;

        match action {
            GameSessionAction::MarkBoardField => {
                println!("Request coordinates:");
                Ok(GameStateAction::ChangeScreen(Box::new(Self {})))
            }
            GameSessionAction::AdmitDefeat => {
                Ok(GameStateAction::ChangeScreen(Box::new(MainScreen {})))
            }
            GameSessionAction::Quit => Ok(GameStateAction::Quit),
        }
    }
}
