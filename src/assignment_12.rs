use std::collections::{HashMap, HashSet};

use crate::{Assignment, Output};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(y: usize, x: usize) -> Coord {
        Coord { x, y }
    }
}

// impl Ord for Coord {}

#[derive(Debug)]
pub struct Grid {
    elevations: Vec<Vec<u8>>,
    start: Coord,
    end: Coord,
}

impl Grid {
    fn possible_next(&self, current_position: &Coord) -> Vec<Coord> {
        let mut possibilities = Vec::new();
        let current_elevation = self.elevations[current_position.y][current_position.x];
        let y_size = self.elevations.len();
        let x_size = self.elevations[0].len();

        if current_position.y > 0
            && self.elevations[current_position.y - 1][current_position.x] <= current_elevation + 1
        {
            possibilities.push(Coord::new(current_position.y - 1, current_position.x));
        }
        if current_position.y < y_size - 1
            && self.elevations[current_position.y + 1][current_position.x] <= current_elevation + 1
        {
            possibilities.push(Coord::new(current_position.y + 1, current_position.x));
        }
        if current_position.x > 0
            && self.elevations[current_position.y][current_position.x - 1] <= current_elevation + 1
        {
            possibilities.push(Coord::new(current_position.y, current_position.x - 1));
        }
        if current_position.x < x_size - 1
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
                        // 'S' => 0,
                        'S' => 27,
                        // 'E' => 27,
                        'E' => 0,
                        c => 27 - ((c as u8) - a_index),
                    })
                    .collect()
            })
            .collect();

        let mut start_x = 0;
        let mut start_y = 0;
        let mut end_x = 0;
        let mut end_y = 0;

        for (i, v) in elevations.iter().enumerate() {
            for (j, elevation) in v.into_iter().enumerate() {
                if *elevation == 0 {
                    start_y = i;
                    start_x = j;
                }
                if *elevation == 27 {
                    end_y = i;
                    end_x = j;
                }
            }
        }

        Ok(Grid {
            elevations,
            start: Coord::new(start_y, start_x),
            end: Coord::new(end_y, end_x),
        })
    }
}

fn debug_print(distances: &HashMap<Coord, u32>, grid: &Grid) {
    let total_x = grid.elevations[0].len();
    let total_y = grid.elevations.len();
    let mut arr = vec![vec![0; total_x]; total_y];

    for (coord, distance) in distances {
        arr[coord.y][coord.x] = distance.to_owned();
    }

    println!("{:?}", arr);
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
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        distances.insert(grid.start.clone(), 0);

        let mut debug_index = 0;

        // while !visited.contains(&grid.end) {
        while distances.len() > visited.len() {
            // println!("{} ({}, {})", debug_index, distances.len(), visited.len());
            debug_index += 1;

            let mut sorted_distances = distances.clone().into_iter().collect::<Vec<(Coord, u32)>>();
            sorted_distances.sort_by_key(|(_, distance)| *distance);
            let (to_check, distance) = *sorted_distances
                .iter()
                .filter(|(coord, _)| !visited.contains(coord))
                .collect::<Vec<&(Coord, u32)>>()
                .get(0)
                .unwrap();

            for coord in grid.possible_next(to_check) {
                if !distances.contains_key(&coord) {
                    distances.insert(coord, distance + 1);
                } else if distances.get(&coord).unwrap() > &(distance + 1) {
                    println!("got here2");
                    distances.insert(coord, distance + 1);
                }
            }

            visited.insert(to_check.to_owned());

            if debug_index > 3000 {
                break;
            }
        }

        // println!("{:?}", distances);
        debug_print(&distances, &grid);

        Some((*distances.get(&grid.end).unwrap()).into())
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
