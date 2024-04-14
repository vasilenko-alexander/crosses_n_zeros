mod actions;
mod game;
mod screens;

use game::Game;

fn main() {
    Game::new().run();
}
