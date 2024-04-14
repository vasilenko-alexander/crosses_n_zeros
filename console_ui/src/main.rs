use std::{fmt::Display, io, str::FromStr};

fn main() {
    println!("Hello, Players! Welcome to Cross and Zeros!");
    let mut user_choice = String::new();
    'game_loop: loop {
        show_main_menu();

        user_choice.clear();
        // If we cannot read from the console, we crash
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Could not read user action!");

        match user_choice.parse::<MainMenuActions>() {
            Ok(action) => match action {
                MainMenuActions::Start => println!("Starting the game..."),
                MainMenuActions::Quit => break 'game_loop,
            },
            Err(err) => {
                println!("{err}Please try again...");
            }
        }
    }
}

fn show_main_menu() {
    println!("Cross and Zeros:");
    println!("1. Start new game");
    println!("2. Quit");
    println!("Please select an action by its code: ");
}

enum MainMenuActions {
    Start,
    Quit,
}

struct ActionParseError {
    msg: String,
}

impl FromStr for MainMenuActions {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(MainMenuActions::Start),
            "2" => Ok(MainMenuActions::Quit),
            _ => Err(ActionParseError {
                msg: format!("Unknown action with code: {s}"),
            }),
        }
    }
}

impl Display for ActionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
