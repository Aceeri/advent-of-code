use std::{collections::VecDeque, env, error::Error, fs};

struct Grid {
    rows: Vec<Vec<u32>>,
}

impl Grid {
    fn new(rows: Vec<Vec<u32>>) -> Self {
        Grid { rows, }
    }

    fn check_surrounding(&self, x: usize, y: usize) -> bool {
        let number = self.rows[x][y];

        let mut compare = Vec::new();
        if x > 0 {
            compare.push(self.rows[x-1][y]);
        }

        if y > 0 {
            compare.push(self.rows[x][y-1]);
        }

        if x < self.rows.len() - 1 {
            compare.push(self.rows[x + 1][y]);
        }

        if y < self.rows[x].len() - 1 {
            compare.push(self.rows[x][y + 1]);
        }

        for surrounding in &compare {
            if number >= *surrounding {
                return false;
            }
        }

        println!("number: {:?}, surrounding: {:?}", number, compare);

        return true;
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points = Vec::new();
        for x in 0..self.rows.len() {
            for y in 0..self.rows[x].len() {
                if self.check_surrounding(x, y) {
                    low_points.push((x, y));
                }
            }
        }
        low_points
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        self.rows[x][y]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut lines = contents.lines();

    let rows = lines.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let grid = Grid::new(rows);
    let low_points = grid.low_points();
    let sum: u32 = low_points.iter().map(|point| 1 + grid.get(point.0, point.1)).sum();
    println!("{:?}", low_points);
    println!("{:?}", sum);

    Ok(())
}
