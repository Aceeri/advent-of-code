use std::{collections::{HashSet, VecDeque}, env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut lines = contents.lines();


    let mut scores = Vec::new();
    let mut score = 0;

    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let mut corrupt = false;
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

                        //println!("expected {:?}, found: {:?}", last, c);
                        corrupt = true;
                        break; 
                    }
                }
                _ => {},
            }
        }

        if !corrupt {
            let mut total_score: u128 = 0;
            println!("complete with {:?}", stack);
            for c in stack.iter().rev() {
                let s = match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                };
                total_score *= 5;
                total_score += s;
            }

            scores.push(total_score);
        }
    }

    scores.sort_by(|a, b| a.cmp(b));
    println!("{:?}", scores.len());
    println!("{:?}", scores.len() / 2);
    println!("{:?}", scores[scores.len()/2]);


    Ok(())
}
