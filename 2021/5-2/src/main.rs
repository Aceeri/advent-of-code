use std::{collections::{HashSet, VecDeque}, env, error::Error, fs};

use regex::Regex;

fn line_range(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
    if end < start {
        return Box::new((end..=start).rev());
    }

    Box::new(start..=end)
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;

    let re = Regex::new(r"(\d+),(\d+)\s*->\s*(\d+),(\d+)").unwrap();

    let mut grid = vec![vec![0u8; 1000]; 1000];
    for captures in re.captures_iter(&contents) {
        let start_x: usize = captures.get(1).unwrap().as_str().parse()?;
        let start_y: usize = captures.get(2).unwrap().as_str().parse()?;
        let end_x: usize = captures.get(3).unwrap().as_str().parse()?;
        let end_y: usize = captures.get(4).unwrap().as_str().parse()?;

        if start_x == end_x { // horizontal
            let x = start_x;
            let min = start_y.min(end_y);
            let max = start_y.max(end_y);
            for y in min..=max {
                grid[y][x] += 1;
            }

        } else if start_y == end_y { // vertical
            let y = start_y;
            let min = start_x.min(end_x);
            let max = start_x.max(end_x);
            for x in min..=max {
                grid[y][x] += 1;
            }
        } else { // diagonal
            for (y, x) in (line_range(start_y, end_y)).zip(line_range(start_x, end_x)) {
                grid[y][x] += 1;
            }
        }
    }

    let mut overlapped = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
            if grid[y][x] > 1 {
                overlapped += 1;
            }
        }
        println!();
    }

    println!("{:?}", overlapped);


    Ok(())
}
