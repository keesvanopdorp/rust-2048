use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rand::Rng;
use std::{io::stdout, process::exit};

pub struct Game {
    score: i32,
    game_started: bool,
    board: [[i32; 4]; 4],
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            game_started: false,
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
        }
        self.board[x][y] = 2;
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
        //let mut stdout = stdout();

        self.init();

        // ===== Begin of gameloop =====
        loop {
            disable_raw_mode().unwrap();
            //execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
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
            self.generate_tile();
        }
        // ===== End of gameloop =====
    }

    fn move_board(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                let mut x = 0;
                while x != 3 {
                    let mut y = 3;
                    while y != 0 {
                        let previous_index: usize = y - 1;
                        print!("y,x: {},{}\r\n", previous_index, x as usize);
                        if self.board[previous_index][x] != 0 {
                            // move title up
                            self.board[previous_index][x] = self.board[y][x];
                            self.board[y][x] = 0;
                        }

                        if self.board[previous_index][x] == self.board[y][x] {
                            self.board[previous_index][x] = self.board[y][x];
                            self.board[y][x] = 0;
                            self.score += self.board[previous_index][x];
                        }
                        y -= 1;
                    }
                    x += 1;
                }
            }
            Direction::Down => {
                let mut y = 0;
                while y != 3 {
                    let mut x = 0;
                    while x != 3 {
                        if self.board[y + 1][x] == 0 && y != 3 {
                            self.board[y + 1][x] = self.board[y][x];
                            self.board[y][x] = 0;
                        }

                        if self.board[y + 1][x] == self.board[y][x] {
                            self.board[y + 1][x] = self.board[y + 1][x] + self.board[y][x];
                            self.board[y][x] = 0;
                            self.score += self.board[y + 1][x];
                        }

                        x += 1;
                    }
                    y += 1;
                }
            }
            Direction::Right => {
                let mut y = 0;
                while y != 3 {
                    let mut x = 0;
                    while x != 3 {
                        if self.board[y][x + 1] == 0 && x != 3 {
                            self.board[y][x + 1] = self.board[y][x];
                            self.board[y][x] = 0;
                        }

                        if self.board[y][x + 1] == self.board[y][x] {
                            self.board[y][x + 1] = self.board[y][x + 1] + self.board[y][x];
                            self.board[y][x] = 0;
                            self.score += self.board[y][x + 1];
                        }

                        x += 1;
                    }
                    y += 1;
                }
            }
            Direction::Left => {
                let mut y = 0;
                while y != 3 {
                    let mut x = 3;
                    while x >= 0 {
                        if self.board[y][x - 1] == 0 && x != 0 {
                            self.board[y][x - 1] = self.board[y][x];
                            self.board[y][x] = 0;
                        }

                        if self.board[y][x - 1] == self.board[y][x] {
                            self.board[y][x - 1] = self.board[y][x - 1] + self.board[y][x];
                            self.board[y][x] = 0;
                            self.score += self.board[y][x - 1];
                        }
                        println!("x {}", x);
                        x -= 1;
                    }
                    y += 1;
                }
            }
        }
    }
}
