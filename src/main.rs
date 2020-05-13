use std::convert::From;
use std::fmt;
use std::io;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Debug, PartialEq)]
enum BoardValue {
    Player(Player),
    Empty(),
}

impl fmt::Display for BoardValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            BoardValue::Player(v) => write!(f, "{:?}", v),
            BoardValue::Empty() => write!(f, " "),
        }
    }
}

// impl std::cmp::PartialEq for BoardValue {
//     fn eq(&self, other: &Self) -> bool {
//         self.isbn == other.isbn
//     }
// }
#[derive(Debug)]
struct Board {
    board: Vec<Vec<BoardValue>>,
}
#[derive(Debug)]

enum BoardError {
    InvalidMove(std::string::String),
}

impl Board {
    pub fn new() -> Self {
        let mut board = Vec::new();
        for _ in 0..3 {
            let mut row: Vec<BoardValue> = Vec::new();
            for _ in 0..3 {
                row.push(BoardValue::Empty());
            }
            board.push(row);
        }

        Self { board }
    }

    pub fn size(&self) -> usize {
        self.board.len()
    }

    pub fn game_over(&self) -> bool {
        self.board_full() || self.check_for_winner()
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: Player) -> Result<(), BoardError> {
        match self.board[row][col] {
            BoardValue::Empty() => self.board[row][col] = BoardValue::Player(Player::from(player)),
            _ => {
                return Err(BoardError::InvalidMove(format!(
                    "Invalid Move: space occupied by {:?}",
                    self.board[row][col]
                )))
            }
        };
        Ok(())
    }

    fn check_for_winner(&self) -> bool {
        // TODO cleanup
        for current_player in [BoardValue::Player(Player::X), BoardValue::Player(Player::O)].iter()
        {
            for row in self.board.iter() {
                let mut won_by_row = true;
                for cell in row {
                    match cell {
                        cell if cell == current_player => continue,
                        _ => won_by_row = false,
                    }
                }
                if won_by_row {
                    return true;
                }
            }
            for col_index in 0..self.board.len() {
                let mut won_by_col = true;
                for row_index in 0..self.board.len() {
                    let cell = &self.board[row_index][col_index];
                    match cell {
                        cell if cell == current_player => continue,
                        _ => won_by_col = false,
                    }
                }
                if won_by_col {
                    return true;
                }
            }
            let mut won_by_diagonal_left = true;
            let mut won_by_diagonal_right = true;
            for index in 0..self.board.len() {
                let cell = &self.board[index][index];
                match cell {
                    cell if cell == current_player => continue,
                    _ => won_by_diagonal_left = false,
                }
                let cell = &self.board[index][self.board.len() - 1 - index];
                match cell {
                    cell if cell == current_player => continue,
                    _ => won_by_diagonal_right = false,
                }
            }
            if won_by_diagonal_left || won_by_diagonal_right {
                return true;
            }
        }
        return false;
    }

    fn board_full(&self) -> bool {
        for row in &self.board {
            for cell in row {
                match cell {
                    BoardValue::Empty() => return false,
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
    RangeError(&'static str),
}

impl From<io::Error> for InputError {
    fn from(error: io::Error) -> Self {
        InputError::IoError(error)
    }
}

fn get_input(max_size: &usize) -> Result<(usize, usize), InputError> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let values: Vec<&str> = line.split_whitespace().collect();
    let values: Vec<usize> = values.iter().filter_map(|x| parse_input(x).ok()).collect();
    match values.len() {
        2 => {}
        _ => {
            return Err(InputError::IndexError(
                "Invalid valid input; example input: 1 3",
            ))
        }
    }
    let (row, col): (usize, usize) = (*values.get(0).unwrap(), *values.get(1).unwrap());
    for item in [row, col].iter() {
        let min: usize = 0;
        if item < &min || item > max_size {
            return Err(InputError::RangeError("Input not in valid range of board!"));
        }
    }

    Ok((row, col))
}

fn parse_input(input: &str) -> Result<usize, core::num::ParseIntError> {
    Ok(String::from(input).parse::<usize>())?
}

fn main() -> () {
    let mut board: Board = Board::new();
    let mut player = Player::X;

    loop {
        println!("{}", board);
        println!("Turn {:?}: Enter [row, col]: ", &player);

        let (row, col) = match get_input(&board.size()) {
            Err(x) => {
                println!("{:?}", x);
                continue;
            }
            Ok((x, y)) => (x - 1, y - 1),
        };

        match board.make_move(row, col, player) {
            Err(x) => {
                println!("{:?}", x);
                continue;
            }
            _ => {}
        }

        match board.game_over() {
            true => {
                println!("Game Over!");
                break;
            }
            false => println!("Next Player"),
        }

        player = match player {
            Player::X => Player::O,
            _ => Player::X,
        };
    }

    println!("{}", board)
}
