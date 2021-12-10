use std::{collections::{HashSet, VecDeque}, env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut fishes = contents.split(",").map(|num| num.parse().unwrap()).collect::<Vec<usize>>();
    let mut per_day: [u128; 9] = [0; 9];
    for fish in fishes {
        per_day[fish] += 1;
    }

    for i in 1..=256 {
        let next_day: [u128; 9] = [per_day[1], per_day[2], per_day[3], per_day[4], per_day[5], per_day[6], per_day[7] + per_day[0], per_day[8], per_day[0]];
        per_day = next_day;
        let sum: u128 = per_day.iter().sum();
    }

    let sum: u128 = per_day.iter().sum();
    println!("{:?}", sum);

    Ok(())
}
