
use std::{
    fs,
    env,
    error::Error,
    collections::VecDeque,
};

struct BingoGame {
    called: Vec<u64>,
    boards: Vec<Board>,
}

struct Board {
    numbers: Vec<Vec<u64>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut readings = contents
        .lines()
        .map(|line| line.parse::<u64>().expect("could not parse integer"));

    Ok(())
}