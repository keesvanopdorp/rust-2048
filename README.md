# Rust-2048

This is my  version of the game 2048 written in Rust as a side project, because I wanted to learn more about Rust.

## Decencies used in the project

- Std (Standard rust library) - Is included standard when compiling Rust.

- Rand - Use to randomly generate numbers used to generate the coordinates for new tiles.

- Crossterm - Used to get the raw input of the arrows keys to play the game.

## project structure

in the src folder there are 2 files used to create the game.

- `main.rs` - is used to create a instance of the Game struct and call the start method on that instance.

- `game/mod.rs` - THe struct used for all the game logic in the game.

## Open TODO's 

- [ ] Add a algorithm to check the board for valid.

- [ ] Check merge code for disappearing fields.

- [ ] add a main menu before the main game loop.

- [ ] Add a how to play section.

- [ ] Add unit testing to the project.

## How to run the project

- Be sure that you have Rust and Cargo installed you can it install it [here](https://www.rust-lang.org/tools/install).
    
    - To check if you have Rust and Cargo installed you can run `rustc --version` or `cargo --version`.

- open a Terminal and run `cargo run` to build and run a debug build and `cargo run --release` to build and run a release build.

    - **`cargo run` wil automatically install and build all the project dependencies.**