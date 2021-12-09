
use std::{
    fs,
    env,
    error::Error,
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
        .map(|line| line.parse::<u64>().expect("could not parse integer"));

    let first = readings.next().expect("at least 1 reading");

    let mut state = Readings {
        increasing: 0,
        previous: first,
    };

    state = readings.fold(state, |mut state, reading| {
        if reading > state.previous {
            state.increasing += 1;
        }

        state.previous = reading;
        state
    });

    println!("{:?}", state);
    Ok(())
}