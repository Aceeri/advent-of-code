use std::{collections::{HashSet, VecDeque}, env, error::Error, fs};

struct Grid {
    rows: Vec<Vec<u32>>,
}

impl Grid {
    fn new(rows: Vec<Vec<u32>>) -> Self {
        Grid { rows, }
    }

    fn surrounding(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut compare = Vec::new();
        if x > 0 {
            compare.push((x-1, y));
        }

        if y > 0 {
            compare.push((x, y - 1));
        }

        if x < self.rows.len() - 1 {
            compare.push((x + 1, y));
        }

        if y < self.rows[x].len() - 1 {
            compare.push((x, y + 1));
        }

        compare
    }

    fn check_surrounding(&self, x: usize, y: usize) -> bool {
        let number = self.rows[x][y];
        let compare = self.surrounding(x, y);

        for surrounding in &compare {
            let surrounding_height = self.get(surrounding.0, surrounding.1);
            if number >= surrounding_height {
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

    fn basins(&self) -> Vec<Vec<(usize, usize)>> {
        let mut basins = Vec::new();
        let low_points = self.low_points();

        for low_point in low_points {
            let mut check = VecDeque::new();
            let mut check_hash = HashSet::new();

            check.push_back(low_point);
            check_hash.insert(low_point);

            let mut part = Vec::new();

            // fill the surrounding areas until you hit a 9 height.
            while check.len() > 0 {
                println!("current: {:?}", check);
                let current = check[0];
                let height = self.get(current.0, current.1);
                if height < 9 {
                    part.push(current);
                    for surrounding_point in self.surrounding(current.0, current.1) {
                        if !check_hash.contains(&surrounding_point) {
                            check.push_back(surrounding_point);
                            check_hash.insert(surrounding_point);
                        }
                    }
                }

                check.pop_front();
            }

            basins.push(part);
        }
        
        basins
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
    //println!("{:?}", low_points);
    //println!("{:?}", sum);

    let mut basins = grid.basins();
    basins.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
    basins.truncate(3);

    let result: usize = basins.iter().map(|basin| basin.len()).product();
    println!("{:?}", result);
    for basin in basins {
        println!("{:?}", basin.len());
    }

    Ok(())
}
