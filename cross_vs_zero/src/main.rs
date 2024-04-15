mod actions;
mod game;
mod screens;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
