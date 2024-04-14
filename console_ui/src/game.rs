use std::{borrow::Borrow, io};

use crate::{
    actions::GameStateAction,
    screens::{MainScreen, Screen},
};

pub struct Game {
    welcome_banner: String,
    current_screen: Box<dyn Screen>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            welcome_banner: "Welcome to Cross and Zeros!".to_owned(),
            current_screen: Box::new(MainScreen {}),
        }
    }

    pub fn run(&mut self) {
        self.show_welcome_banner();

        loop {
            self.current_screen.show();

            let player_action = self.read_player_action();

            let game_action = match self.current_screen.handle_action(player_action) {
                Ok(action) => action,
                Err(err) => {
                    println!("{err} Please try again...");
                    continue;
                }
            };

            match game_action {
                GameStateAction::ChangeScreen(screen) => self.current_screen = screen,
                GameStateAction::Quit => break,
            }
        }
    }

    fn show_welcome_banner(&self) {
        println!("{}", self.welcome_banner)
    }

    fn read_player_action(&self) -> String {
        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Something went wrong. Failed to read action code");

        action
    }
}
