
use std::{
    fs,
    env,
    error::Error,
    collections::VecDeque,
};

#[derive(Debug, Clone)]
struct Readings<const WINDOW: usize> {
    increasing: u64,
    window: VecDeque<u64>, // FIFO queue
}

impl<const WINDOW: usize> Readings<WINDOW> {
    pub fn read_initial_window(&mut self, mut iter: impl Iterator<Item = u64>) {
        for i in 0..WINDOW {
            self.window.push_back(iter.next().unwrap());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut readings = contents
        .lines()
        .map(|line| line.parse::<u64>().expect("could not parse integer"));

    let mut state = Readings::<3> {
        increasing: 0,
        window: VecDeque::new(),
    };

    state.read_initial_window(&mut readings);

    state = readings.fold(state, |mut state, reading| {
        let previous_sum: u64 = state.window.iter().sum();
        state.window.pop_front();
        state.window.push_back(reading);
        let current_sum: u64 = state.window.iter().sum();
        print!("{} > {}", previous_sum, current_sum);

        if current_sum > previous_sum {
            print!(" (increased)");
            state.increasing += 1;
        } else if current_sum == previous_sum {
            print!(" (no change)");
        } else if current_sum < previous_sum {
            print!(" (decreased)");
        }
        println!();

        state
    });

    println!("{:?}", state);
    Ok(())
}