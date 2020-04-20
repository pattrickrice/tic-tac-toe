use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Player {
    x,
    o,
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<&'static str>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Vec::new();
        for _ in 0..3 {
            let mut row: Vec<&str> = Vec::new();
            for _ in 0..3 {
                row.push("");
            }
            board.push(row);
        }

        Self { board }
    }

    pub fn game_over(&self) -> bool {
        self.board_full()
    }

    pub fn make_move(&self, row: usize, col: usize, player: Player) -> () {
        match self.board[row][col] {
            "" => {}
            _ => panic!("Invalid Move: space occupied by {}", self.board[row][col]),
        }
    }

    fn board_full(&self) -> bool {
        for row in &self.board {
            for c in row {
                match &c[..] {
                    "" => return false,
                    _ => continue,
                }
            }
        }
        true
    }
}

fn get_input() -> Result<(String, String), &'static str> {
    let stdin = io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    let values: Vec<&str> = line1.split(" ").collect();
    let (row, col) = (
        match values.get(0) {
            Some(x) => x,
            None => return Err("Invalid input!"),
        },
        match values.get(1) {
            Some(x) => x,
            None => return Err("Invalid input!"),
        },
    );
    println!("Entered [{:?}, {:?}]: ", row, col);
    let n = String::from(*row);
    Ok((String::from(*row), String::from(*col)))
}

fn main() -> () {
    let mut board: Board = Board::new();
    let player = Player::x;

    loop {
        println!("Turn {:?}: Enter [row, col]: ", player);
        let input = get_input();
        let (row, col) = match input {
            Err(x) => {
                println!("{:?}", x);
                continue;
            }
            Ok((x, y)) => (x, y),
        };

        println!("row {}; col {}", row, col);

        match board.game_over() {
            true => {
                println!("Game Over!");
                break;
            }
            false => println!("Next Player"),
        }
    }

    println!("{}", board)
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. RePlayers `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut out = String::new();
        for row in &self.board {
            out.push_str(&format!("{:?}\n", row).to_owned())
        }

        write!(f, "{}", out)
    }
}
