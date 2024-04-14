use std::{fmt::Display, io, str::FromStr};

fn main() {
    show_welcome_banner();

    loop {
        show_main_menu();

        // If we cannot read user input from console we would not be able to proceed.
        // So we crash and will log such things in future
        let user_choice = read_user_action_choice()
            .expect("Failed to read player's choice from console. Shutting down...");

        match user_choice.parse::<MainMenuActions>() {
            Ok(action) => match action {
                MainMenuActions::Start => show_game_board(),
                MainMenuActions::Quit => break,
            },
            Err(err) => {
                println!("{err}Please try again...");
            }
        }
    }
}

fn show_welcome_banner() {
    println!("Hello, Players! Welcome to Cross and Zeros!");
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

fn show_game_board() {
    let underscore = '\u{ff3f}';
    let first_player = 'X';
    let second_player = 'O';
    println!("{first_player}|{first_player}|{second_player}");
    println!("{second_player}|{second_player}|{first_player}");
    println!("{first_player}|{second_player}|{first_player}");
}

enum GameActions {
    MarkBoardField(i32, i32),
    AdmitDefeat,
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
