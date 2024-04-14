use crate::actions::{errors::ActionParseError, GameStateAction};
mod game_session;
mod main;

pub use game_session::GameSessionScreen;
pub use main::MainScreen;

pub trait Screen {
    fn show(&self) {
        self.show_display();
        self.show_actions();
    }
    fn show_display(&self);
    fn show_actions(&self);
    fn handle_action(&self, action: String) -> Result<GameStateAction, ActionParseError>;
}
