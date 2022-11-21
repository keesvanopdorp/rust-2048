mod game;

use crate::game::Game;
use crossterm::terminal;

fn main() {
    // Sets the terminal title for the current terminal window
    terminal::SetTitle("2048 game");
    // Starts the game and init the gameloop
    Game::new().run();
}
