use std::{collections::{HashSet, VecDeque}, env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut lines = contents.lines();

    let mut score = 0;

    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {

            print!("{}", c);
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                '[' => stack.push(']'),
                ')' | '}' | '>' | ']' => {
                    let last = stack.pop().unwrap();

                    if last != c {
                        score += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        };

                        println!("expected {:?}, found: {:?}", last, c);
                        break; 
                    }
                }
                _ => {},
            }
        }
    }
    println!("{:?}", score);

    Ok(())
}
