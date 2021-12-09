use std::{collections::VecDeque, env, error::Error, fs};

#[derive(Debug, Clone)]
struct BingoGame {
    called: Vec<u64>,
    boards: Vec<Board>,
}

impl BingoGame {
    fn run(&mut self) {
        for call in &self.called {
            let mut won = Vec::new();

            for board in &mut self.boards {
                board.mark(*call);
                if board.won() {
                    won.push(board.clone());
                }
            }

            if won.len() > 0 {
                println!("won: {:?}", won);
                println!("called: {:?}", call);
                for board in won {
                    println!("board score: {:?}", board.score());
                    println!("board score * called: {:?}", board.score() * call);
                }
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<(u64, bool)>>,
}

impl Board {
    fn new() -> Self {
        Board { rows: Vec::new() }
    }

    fn valid(&self) -> bool {
        if self.rows.len() != 5 {
            return false;
        }

        for row in &self.rows {
            if row.len() != 5 {
                return false;
            }
        }

        return true;
    }

    fn mark(&mut self, number: u64) {
        for row in &mut self.rows {
            for (board_number, marked) in row.iter_mut() {
                if *board_number == number {
                    *marked = true;
                }
            }
        }
    }

    fn won(&self) -> bool {
        // naive algorithm but whatever

        // check if any of the rows won
        for row in &self.rows {
            let mut won = true;
            for (number, marked) in row {
                if !marked {
                    won = false;
                    break;
                }
            }

            if won {
                return true;
            }
        }

        // check if any of the columns won
        let transposed = transpose(self.rows.clone());
        for column in &transposed {
            let mut won = true;
            for (number, marked) in column {
                if !marked {
                    won = false;
                    break;
                }
            }

            if won {
                return true;
            }
        }

        return false;
    }

    fn score(&self) -> u64 {
        self.rows
            .iter()
            .flatten()
            .filter(|(_, marked)| !marked)
            .map(|(n, _)| n)
            .sum()
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut lines = contents.lines();

    // Parse called numbers
    let called_list = lines.next().expect("expected called numbers");
    let mut called = Vec::new();
    for called_string in called_list.split(",") {
        let called_number: u64 = called_string.parse()?;
        called.push(called_number);
    }

    // Parse boards
    let mut boards = Vec::new();
    let mut board = Board::new();
    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.len() > 0 {
            let mut row = Vec::new();
            for string in trimmed_line.split_whitespace() {
                let number: u64 = string.parse()?;
                row.push((number, false));
            }

            board.rows.push(row);
        } else {
            if board.valid() {
                boards.push(board);
                board = Board::new();
            }
        }
    }

    if board.valid() {
        boards.push(board);
        board = Board::new();
    }

    let mut game = BingoGame { boards, called };
    game.run();
    //println!("{:?}", game);

    Ok(())
}
