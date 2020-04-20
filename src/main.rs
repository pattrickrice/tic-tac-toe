use std::fmt;

fn main() {
    let mut board: Board = Board::new();

    println!("{}", board)
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
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut out = String::new();
        for row in &self.board {
            out.push_str(&format!("{:?}\n", row).to_owned())
        }

        write!(f, "{}", out)
    }
}
