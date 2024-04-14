mod actions;
mod game;
mod screens;

use actions::{GameActions, MainAction};
use std::io;

fn main() {
    show_welcome_banner();

    loop {
        show_main_menu();

        // If we cannot read user input from console we would not be able to proceed.
        // So we crash and will log such things in future
        let user_choice = read_user_action_choice()
            .expect("Failed to read player's choice from console. Shutting down...");

        match user_choice.parse::<MainAction>() {
            Ok(action) => match action {
                MainAction::Start => {
                    show_game_board();
                    show_game_menu();

                    let user_action = read_user_action_choice()
                        .expect("Failed to read player's choice from console. Shutting down...");

                    match user_action.parse::<GameActions>() {
                        Ok(action) => println!("Do something!"),
                        Err(err) => println!("{err}Please try again..."),
                    }
                }
                MainAction::Quit => break,
            },
            Err(err) => {
                println!("{err}Please try again...");
            }
        }
    }
}

fn show_game_menu() {
    println!("1. Mark board field in format(code x,y)");
    println!("2. Admit Defeat");
}

fn show_welcome_banner() {
    println!("Hello, Players! Welcome to Cross and Zeros!");
}

fn show_main_menu() {
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
    let first_player = 'X';
    let second_player = 'O';
    println!("{first_player}|{first_player}|{second_player}");
    println!("{second_player}|{second_player}|{first_player}");
    println!("{first_player}|{second_player}|{first_player}");
}
