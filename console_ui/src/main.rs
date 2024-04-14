use std::{fmt::Display, io, str::FromStr};

fn main() {
    println!("Hello, Players! Welcome to Cross and Zeros!");

    'game_loop: loop {
        show_main_menu();

        let user_choice =
            read_user_action_choice().expect("Could not read player's choice. Shutting down...");

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

fn read_user_action_choice() -> Result<String, io::Error> {
    let mut user_choice = String::new();

    io::stdin().read_line(&mut user_choice)?;

    Ok(user_choice)
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
