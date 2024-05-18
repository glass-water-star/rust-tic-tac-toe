use rand::Rng;
use std::{io, os};
use strum_macros::Display;
#[derive(Copy, Clone)]
enum Player {
    Human,
    Computer,
}

enum GameStatus {
    InProgress,
    Over,
}
#[derive(Display)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

pub fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

struct Game {
    board: Vec<Vec<String>>,
    current_player: Player,
    status: GameStatus,
    result: GameResult,
    player_mark: String,
    computer_mark: String,
    winner: Player,
}

impl Game {
    pub fn new(player_mark: String) -> Self {
        Self {
            board: vec![vec![String::from(" "); 3]; 3],
            current_player: Player::Human,
            status: GameStatus::InProgress,
            result: GameResult::Draw,
            player_mark: player_mark.to_string(),
            computer_mark: if player_mark == "X" {
                "O".to_string()
            } else {
                "X".to_string()
            },
            winner: Player::Human,
        }
    }

    pub fn display_board(&self) {
        println!("Current Board:");
        println!(" - - -");
        for row in self.board.iter() {
            print!("|");
            for cell in row.iter() {
                print!("{}|", cell);
            }
            println!();
            println!(" - - -");
        }
    }

    fn turn(&mut self) {
        match self.current_player {
            Player::Human => {
                println!("Please input your move in the format: row, column");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input: Vec<&str> = input.trim().split(',').collect();
                if input[0] == "e" {
                    self.status = GameStatus::Over;
                    return;
                }
                println!("checking...");
                let row: usize = input[0].parse().unwrap();
                let col: usize = input[1].parse().unwrap();
                if self.board[row][col] == " " {
                    self.board[row][col] = self.player_mark.to_string();
                    self.check_winner();
                    self.current_player = Player::Computer;
                } else {
                    println!("Invalid move, please try again");
                }
            }
            Player::Computer => {
                println!("Computer's turn");
                let row = pick_random_position();
                let col = pick_random_position();
                if self.board[row as usize][col as usize] == " " {
                    self.board[row as usize][col as usize] = self.computer_mark.to_string();
                    self.check_winner();
                    self.current_player = Player::Human;
                } else {
                    self.turn();
                }
            }
        }
    }
    fn check_winner(&mut self) -> bool {
        let directions: [i32; 5] = [-2, -1, 0, 1, 2];
        for i in 0..self.board.len() {
            for j in 0..self.board.len() {
                let mut x_s: usize = 0;
                let mut o_s: usize = 0;
                for m in directions {
                    for n in directions {
                        if (m == 0 && n == 0) || add(i, m) < Some(0) || add(j, n) < Some(0) {
                            continue;
                        }
                        let im: usize = add(i, m).unwrap();
                        let jn: usize = add(j, n).unwrap();
                        if im >= self.board.len() || jn >= self.board.len() {
                            continue;
                        }
                        if add(i, m) == None || add(j, n) == None {
                            continue;
                        } else {
                            if self.board[i][j] == self.board[im][jn] {
                                if self.board[i][j] == "X" {
                                    x_s += 1;
                                } else if self.board[i][j] == "O" {
                                    o_s += 1;
                                }
                            }
                        }
                        if x_s == 3 {
                            self.winner = self.current_player;
                            self.status = GameStatus::Over;
                            self.result = GameResult::Win;
                            return true;
                        }
                        if o_s == 3 {
                            self.winner = self.current_player;
                            self.status = GameStatus::Over;
                            self.result = GameResult::Lose;
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
}

fn pick_random_position() -> u32 {
    rand::thread_rng().gen_range(0..=2)
}

fn main() {
    println!("Lets Play Tic Tac Toe!");
    println!("Do you want to be X or O?");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: String = input.trim().to_string();
    if input == "X" {
        println!("You are X and will go first");
    } else {
        println!("You are O and will go second");
    }
    let mut game: Game = Game::new(input);
    while matches!(game.status, GameStatus::InProgress) {
        game.turn();
        game.display_board();
    }
    println!("Game Over!");
    println!("Result: {}", game.result);
    println!("Would you like to play again? (y/n)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.trim() == "y" {
        main();
    }
}
