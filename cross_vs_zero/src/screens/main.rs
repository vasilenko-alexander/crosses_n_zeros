use game_logic::{GameBoard, Player};

use super::{GameSessionScreen, Screen};
use crate::actions::{errors::ActionParseError, GameStateAction, MainAction};

pub struct MainScreen {}

impl Screen for MainScreen {
    fn show_display(&self) {
        println!("Welcome to 'Cross vs Zero'!");
    }

    fn show_actions(&self) {
        println!("1. Start new game");
        println!("2. Quit");
        println!("Please select an action by its code: ");
    }

    fn handle_action(&mut self, action: String) -> Result<GameStateAction, ActionParseError> {
        let action = action.parse::<MainAction>()?;
        match action {
            MainAction::Start => Ok(GameStateAction::ChangeScreen(Box::new(
                GameSessionScreen::new(GameBoard::new(), Player::create_player_x()),
            ))),
            MainAction::Quit => Ok(GameStateAction::Quit),
        }
    }
}
