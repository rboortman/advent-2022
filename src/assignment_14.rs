use std::fmt::Display;

use crate::{Assignment, Output};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Empty,
    Path,
    Rock,
}

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Vec<Item>>,
    x_offset: usize,
    y_offset: usize,
}

impl Grid {
    fn new(
        paths: Vec<(usize, usize)>,
        x_offset: usize,
        y_offset: usize,
        max_x: usize,
        max_y: usize,
    ) -> Grid {
        let mut grid = vec![vec![Item::Empty; max_x - x_offset + 1]; max_y - y_offset + 1];
        for (x, y) in paths {
            grid[y - y_offset][x - x_offset] = Item::Path
        }

        Grid {
            grid,
            x_offset,
            y_offset,
        }
    }

    fn drop_rock(&mut self, mut x: usize, mut y: usize) -> bool {
        if *self.get(&x, &y).unwrap() == Item::Rock {
            return false;
        }

        let mut is_dropped = false;
        while !is_dropped {
            match self.get(&x, &(y + 1)) {
                None => return false,
                Some(Item::Empty) => y += 1,
                Some(_) => match self.get(&(x - 1), &(y + 1)) {
                    None => return false,
                    Some(Item::Empty) => {
                        x -= 1;
                        y += 1;
                    }
                    Some(_) => match self.get(&(x + 1), &(y + 1)) {
                        None => return false,
                        Some(Item::Empty) => {
                            x += 1;
                            y += 1;
                        }
                        Some(_) => is_dropped = true,
                    },
                },
            }
        }
        self.grid[y - self.y_offset][x - self.x_offset] = Item::Rock;
        is_dropped
    }

    fn get_total_rocks(&self) -> i32 {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|item| **item == Item::Rock).count() as i32)
            .sum::<i32>()
    }

    fn get<'a>(&'a self, x: &usize, y: &usize) -> Option<&'a Item> {
        if self.y_offset > *y
            || *y > self.y_offset + (self.grid.len() - 1)
            || self.x_offset > *x
            || *x > self.x_offset + (self.grid[*y].len() - 1)
        {
            return None;
        }

        self.grid
            .get(*y - self.y_offset)
            .and_then(|row| row.get(*x - self.x_offset))
    }

    fn enlarge(old: Grid) -> Grid {
        let mut new_grid = vec![vec![Item::Empty; 1000]; old.grid.len() + 1];
        new_grid.push(vec![Item::Path; new_grid[0].len()]);

        for (i, row) in old.grid.into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                if cell == Item::Empty {
                    continue;
                }
                new_grid[i + old.y_offset][j + old.x_offset] = cell;
            }
        }

        Grid {
            grid: new_grid,
            x_offset: 0,
            y_offset: 0,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        Item::Empty => ".",
                        Item::Path => "#",
                        Item::Rock => "o",
                    })
                    .collect::<String>()
                    + "\n"
            })
            .collect::<String>();

        write!(f, "{}", to_write)
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Grid;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        let coords = input
            .lines()
            .flat_map(|line| {
                let coords_row = line
                    .split(" -> ")
                    .map(|coord_str| {
                        let (x, y) = coord_str.split_once(',').unwrap();
                        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                    })
                    .collect::<Vec<(usize, usize)>>();

                let mut resulting_coords = Vec::new();
                resulting_coords.push(coords_row[0]);
                for i in 1..coords_row.len() {
                    let a = coords_row[i - 1];
                    let b = coords_row[i];
                    let x_a = a.0 as i32;
                    let y_a = a.1 as i32;
                    let x_b = b.0 as i32;
                    let y_b = b.1 as i32;

                    let d_x = (x_b - x_a).signum();
                    let d_y = (y_b - y_a).signum();

                    let x_range = std::cmp::max((x_b - x_a).abs(), 1);
                    let y_range = std::cmp::max((y_b - y_a).abs(), 1);

                    for j in 1..=x_range {
                        for k in 1..=y_range {
                            resulting_coords
                                .push(((x_a + (j * d_x)) as usize, (y_a + (k * d_y)) as usize));
                        }
                    }
                }
                resulting_coords
            })
            .collect::<Vec<(usize, usize)>>();

        let mut min_x = usize::MAX;
        let mut max_x = usize::MIN;
        let min_y = 0;
        let mut max_y = usize::MIN;

        for (x, y) in coords.iter() {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        Some(Grid::new(coords, min_x, min_y, max_x, max_y))
    }

    fn silver(&self, grid: &Self::Input) -> Option<Self::Output> {
        let mut cloned_grid = grid.clone();
        // println!("{}", cloned_grid);

        while cloned_grid.drop_rock(500, 0) {}

        // println!("{}", cloned_grid);

        Some(cloned_grid.get_total_rocks().into())
    }

    fn gold(&self, grid: &Self::Input) -> Option<Self::Output> {
        let cloned_grid = grid.clone();
        let mut bigger_grid = Grid::enlarge(cloned_grid);
        // println!("{}", bigger_grid);

        while bigger_grid.drop_rock(500, 0) {}

        // println!("{}", bigger_grid);

        Some(bigger_grid.get_total_rocks().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 24)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 93)
    }
}
