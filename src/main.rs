use std::convert::From;
use std::fmt;
use std::io;
// use std::io::{self, BufRead};

#[derive(Debug)]
enum Player {
    x,
    o,
}

#[derive(Debug)]
enum BoardValue {
    player(Player),
    empty(),
}

impl fmt::Display for BoardValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            BoardValue::player(v) => write!(f, "{:?}", v),
            BoardValue::empty() => write!(f, " "),
        }
    }
}
#[derive(Debug)]
struct Board {
    board: Vec<Vec<BoardValue>>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Vec::new();
        for _ in 0..3 {
            let mut row: Vec<BoardValue> = Vec::new();
            for _ in 0..3 {
                row.push(BoardValue::empty());
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
            BoardValue::empty() => {}
            _ => panic!("Invalid Move: space occupied by {:?}", self.board[row][col]),
        }
    }

    fn board_full(&self) -> bool {
        for row in &self.board {
            for cell in row {
                match cell {
                    BoardValue::empty() => return false,
                    _ => continue,
                }
            }
        }
        true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "┌───┬───┬───┐\n").unwrap();
        for (index, row) in self.board.iter().enumerate() {
            write!(f, "│").unwrap();
            for cell in row {
                write!(f, "{}  │", cell).unwrap()
            }
            match index {
                2 => write!(f, "\n└───┴───┴───┘\n").unwrap(),
                _ => write!(f, "\n├───┼───┼───┤\n").unwrap(),
            }
        }
        write!(f, "")
    }
}

#[derive(Debug)]
enum InputError {
    IoError(io::Error),
    IndexError(&'static str),
}

impl From<io::Error> for InputError {
    fn from(error: io::Error) -> Self {
        InputError::IoError(error)
    }
}

fn get_input() -> Result<(u8, u8), InputError> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let values: Vec<&str> = line.split_whitespace().collect();
    let values: Vec<u8> = values.iter().filter_map(|x| parse_input(x).ok()).collect();
    match values.len() {
        2 => {}
        _ => return Err(InputError::IndexError("Invalid number of valid inputs")),
    }
    let (row, col): (u8, u8) = (*values.get(0).unwrap(), *values.get(1).unwrap());

    Ok((row, col))
}

fn parse_input(input: &str) -> Result<u8, core::num::ParseIntError> {
    Ok(String::from(input).parse::<u8>())?
}

fn main() -> () {
    let board: Board = Board::new();
    let player = Player::x;

    loop {
        println!("{}", board);
        println!("Turn {:?}: Enter [row, col]: ", player);
        // let input = get_input();
        let (row, col) = match get_input() {
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

// impl fmt::Display for Board {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. RePlayers `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         let mut out = String::new();
//         for row in &self.board {
//             out.push_str(&format!("{:?}\n", row).to_owned())
//         }

//         write!(f, "{}", out)
//     }
// }
