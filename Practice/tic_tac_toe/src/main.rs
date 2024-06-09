use std::fmt;
use std::io;

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Filled(Player),
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Player::X => "X",
            Player::O => "O",
        })
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Empty => write!(f, " "),
            Cell::Filled(player) => write!(f, "{}", player),
        }
    }
}

struct Game {
    board: [[Cell; 3]; 3],
    current_player: Player,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[Cell::Empty; 3]; 3],
            current_player: Player::X,
        }
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    fn make_move(&mut self, row: usize, col: usize) -> Result<(), String> {
        if row >= 3 || col >= 3 {
            return Err("Invalid move".to_string());
        }
        if self.board[row][col] != Cell::Empty {
            return Err("Cell already filled".to_string());
        }

        self.board[row][col] = Cell::Filled(self.current_player);
        self.switch_player();
        Ok(())
    }

    fn check_winner(&self) -> Option<Player> {
        for i in 0..3 {
            if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] {
                if let Cell::Filled(player) = self.board[i][0] {
                    return Some(player);
                }
            }
            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
                if let Cell::Filled(player) = self.board[0][i] {
                    return Some(player);
                }
            }
        }
        
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            if let Cell::Filled(player) = self.board[0][0] {
                return Some(player);
            }
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            if let Cell::Filled(player) = self.board[0][2] {
                return Some(player);
            }
        }

        None
    }

    fn is_draw(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            for cell in row {
                write!(f, "[{}]", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut game = Game::new();
    let mut input = String::new();

    loop {
        println!("{}", game);

        if let Some(winner) = game.check_winner() {
            println!("Player {} wins!", winner);
            break;
        }

        if game.is_draw() {
            println!("It's a draw!");
            break;
        }

        println!("Player {}'s turn. Enter row and column (e.g., 1 1):", game.current_player);
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let mut parts = input.trim().split_whitespace();
        if let (Some(row), Some(col)) = (parts.next(), parts.next()) {
            let row: usize = row.parse().unwrap();
            let col: usize = col.parse().unwrap();

            if let Err(err) = game.make_move(row - 1, col - 1) {
                println!("Error: {}", err);
            }
        }
    }
}