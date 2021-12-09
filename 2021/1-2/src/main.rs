
use std::{
    fs,
    env,
    error::Error,
    collections::VecDeque,
};

#[derive(Debug, Clone)]
struct Readings {
    increasing: u64,
    previous: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut readings = contents
        .lines()
        .map(|line| line.parse::<u64>().expect("could not parse integer"))
        .collect::<Vec<u64>>();

    let mut state = Readings {
        increasing: 0,
        previous: u64::MAX,
    };

    state = readings.as_slice().windows(3).fold(state, |mut state, window| {
        let window_sum: u64 = window.iter().sum();

        if window_sum > state.previous {
            state.increasing += 1;
        }

        state.previous = window_sum;
        state
    });

    println!("{:?}", state);
    Ok(())
}