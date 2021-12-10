use std::{collections::VecDeque, env, error::Error, fs};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("example_input")?;

    let re = Regex::new(r"[abcdefg]+").unwrap();

    for line in contents.lines() {
        let mut unique = Vec::new();
        let mut output = Vec::new();
        for cap in re.captures_iter(line) {
            let signal = cap.get(0).unwrap().as_str();
            if unique.len() < 10 {
                unique.push(signal);
            } else if output.len() < 4 {
                output.push(signal);
            }
        }
    }

    Ok(())
}
