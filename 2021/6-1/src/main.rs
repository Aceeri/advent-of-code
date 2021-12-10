use std::{collections::{HashSet, VecDeque}, env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut fishes = contents.split(",").map(|num| num.parse().unwrap()).collect::<Vec<u8>>();
    //dbg!(fishes);

    for i in 1..=80 {
        let mut add = 0;
        for fish in &mut fishes {
            if *fish == 0 {
                *fish = 6;
                add += 1;
            } else {
                *fish -= 1;
            }
        }

        for i in 0..add {
            fishes.push(8);
        }
        println!("{} day: {:?}", i, fishes);
    }

    println!("{:?}", fishes.len());

    Ok(())
}
