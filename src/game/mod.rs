use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rand::Rng;
use std::{borrow::Borrow, io::stdout, process::exit};

#[derive(PartialEq)]
enum GameState {
    Started,
    Win,
    Lose,
}
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct Game {
    score: i32,
    game_state: GameState,
    board: [[i32; 4]; 4],
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            game_state: GameState::Started,
            board: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        }
    }

    fn init(&mut self) {
        let mut i = 0;
        while i <= 1 {
            self.generate_tile();
            i += 1;
        }
    }

    fn generate_tile(&mut self) {
        let x = rand::thread_rng().gen_range(0..4);
        let y = rand::thread_rng().gen_range(0..4);
        if self.board[y][x] != 0 {
            self.generate_tile();
            return;
        } else {
            self.board[x][y] = 2;
        }
    }

    fn print_board(&self) {
        // ===== Begin printing board =====
        let mut i = 0;
        while i <= 4 {
            if i == 0 {
                print!("---------------------\r\n");
            } else {
                print!("|");
                let row: &[i32; 4] = self.board.get(i - 1).unwrap();
                for column in row {
                    if column != &0 {
                        let string = column.to_string();
                        print!("{}{}|", string, " ".repeat(4 - string.len()));
                    } else {
                        print!("    |");
                    }
                }
                print!("\r\n");
                print!("---------------------\r\n");
            }
            i += 1;
        }
        // ===== End printing board =====
    }

    pub fn run(&mut self) {
        let mut stdout = stdout();

        self.init();

        // ===== Begin of gameloop =====
        loop {
            disable_raw_mode().unwrap();
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            print!("Score: {}\r\n", self.score);

            self.print_board();

            enable_raw_mode().unwrap();
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    // code for up
                    self.move_board(Direction::Up);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    // code for up
                    self.move_board(Direction::Down);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    // code for up
                    self.move_board(Direction::Left);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    // code for up
                    self.move_board(Direction::Right);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    exit(1);
                }
                _ => (),
            }
            self.game_state = self.check_board();

            // End gameloop if the game state is not started
            if self.game_state != GameState::Started {
                break;
            }
            // Generate a game   
            self.generate_tile();
        }
        // ===== End of gameloop =====

        // Gets triggered if the game state is win
        if self.game_state == GameState::Win {
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            print!("You won the game\r\n");
            print!("You final score was: {}\r\n", self.score);
        }

        // Gets triggered if the game state is lose
        if self.game_state == GameState::Lose {
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            print!("You lose the game\r\n");
            print!("You final score was: {}\r\n", self.score);
        }
    }

    fn move_board(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                let y_range = (0..self.board.len()).rev();
                for y in y_range.borrow().to_owned() {
                    let x_range = 0..self.board[y].len();
                    for x in x_range {
                        if y != 0 {
                            if self.board[y - 1][x] == 0 {
                                self.board[y - 1][x] = self.board[y][x];
                                self.board[y][x] = 0;
                            }

                            if self.board[y - 1][x] == self.board[y][x] {
                                self.board[y - 1][x] += self.board[y][x];
                                self.board[y][x] = 0;
                                self.score += self.board[y - 1][x];
                            }
                        }
                    }
                }
            }
            Direction::Down => {
                for y in 0..self.board.len() {
                    for x in 0..self.board[y].len() {
                        if y + 1 < 4 {
                            if self.board[y + 1][x] == 0 {
                                self.board[y + 1][x] = self.board[y][x];
                                self.board[y][x] = 0;
                            }

                            if self.board[y + 1][x] == self.board[y][x] {
                                self.board[y + 1][x] += self.board[y][x];
                                self.board[y][x] = 0;
                                self.score += self.board[y + 1][x];
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                let y_range = 0..self.board.len();
                for y in y_range.borrow().to_owned() {
                    let x_range = 0..self.board[y].len();
                    for x in x_range {
                        if x + 1 < 4 {
                            if self.board[y][x + 1] == 0 {
                                self.board[y][x + 1] = self.board[y][x];
                                self.board[y][x] = 0;
                            }

                            if self.board[y][x + 1] == self.board[y][x] {
                                self.board[y][x + 1] += self.board[y][x];
                                self.board[y][x] = 0;
                                self.score += self.board[y][x + 1];
                            }
                        }
                    }
                }
            }
            Direction::Left => {
                let y_range = 0..self.board.len();
                for y in y_range.borrow().to_owned() {
                    let x_range = (0..self.board[y].len()).rev();
                    for x in x_range {
                        if x != 0 {
                            if self.board[y][x - 1] == 0 {
                                self.board[y][x - 1] = self.board[y][x];
                                self.board[y][x] = 0;
                            }

                            if self.board[y][x - 1] == self.board[y][x] {
                                self.board[y][x - 1] += self.board[y][x];
                                self.board[y][x] = 0;
                                self.score += self.board[y][x - 1];
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_board(&self) -> GameState {
        let game_state: GameState = GameState::Started;

        for row in self.board {
            for column in row {
                if column == 2048 {return GameState::Win;}
            }
        }
        // TODO: Implement search board algorithm
        return game_state;
    }
}
