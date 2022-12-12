use crate::{Assignment, Output};

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(y: usize, x: usize) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug)]
pub struct Grid {
    elevations: Vec<Vec<u8>>,
    start: Coord,
}

impl Grid {
    fn is_at_goal(&self, current_position: &Coord) -> bool {
        self.elevations[current_position.y][current_position.x] == 27
    }

    fn possible_next(&self, current_position: &Coord) -> Vec<Coord> {
        let mut possibilities = Vec::new();
        let current_elevation = self.elevations[current_position.y][current_position.x];
        let size = self.elevations.len();

        if current_position.y > 0
            && self.elevations[current_position.y - 1][current_position.x] <= current_elevation + 1
        {
            possibilities.push(Coord::new(current_position.y - 1, current_position.x));
        }
        if current_position.y < size - 1
            && self.elevations[current_position.y + 1][current_position.x] <= current_elevation + 1
        {
            possibilities.push(Coord::new(current_position.y + 1, current_position.x));
        }
        if current_position.x > 0
            && self.elevations[current_position.y][current_position.x - 1] <= current_elevation + 1
        {
            possibilities.push(Coord::new(current_position.y, current_position.x - 1));
        }
        if current_position.x < size - 1
            && self.elevations[current_position.y][current_position.x + 1] <= current_elevation + 1
        {
            possibilities.push(Coord::new(current_position.y, current_position.x + 1));
        }

        possibilities
    }
}

impl std::str::FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a_index = ('a' as u8) - 1;
        let elevations: Vec<Vec<u8>> = s
            .lines()
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'S' => 0,
                        'E' => 27,
                        c => (c as u8) - a_index,
                    })
                    .collect()
            })
            .collect();

        let mut x = 0;
        let mut y = 0;

        for (i, v) in elevations.iter().enumerate() {
            for (j, elevation) in v.into_iter().enumerate() {
                if *elevation == 0 {
                    y = i;
                    x = j;
                }
            }
        }

        Ok(Grid {
            elevations,
            start: Coord::new(y, x),
        })
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

    fn parse_input(&self, input: &String) -> Option<Self::Input> {
        Some(input.parse().unwrap())
    }

    fn silver(&self, grid: &Self::Input) -> Option<Self::Output> {
        println!("{:?}", grid);
        let mut possible_paths = vec![vec![grid.start.clone()]];
        let mut winner_path = Vec::new();

        loop {
            let mut new_possible_paths = Vec::new();
            for path in possible_paths {
                for new_coord in grid.possible_next(path.last().unwrap()) {
                    if grid.is_at_goal(&new_coord) {
                        winner_path = path.clone();
                        winner_path.push(new_coord);
                        break;
                    } else if !path.contains(&new_coord) {
                        let mut new_path = path.clone();
                        new_path.push(new_coord);
                        new_possible_paths.push(new_path);
                    }
                }
            }

            possible_paths = new_possible_paths;
        }

        println!("{:?}", winner_path);

        Some((-1).into())
    }

    fn gold(&self, grid: &Self::Input) -> Option<Self::Output> {
        Some((-1).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 31)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, -1)
    }
}
