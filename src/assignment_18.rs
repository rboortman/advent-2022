use std::collections::HashSet;

use crate::{Assignment, Output};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord { x, y, z }
    }
    fn is_adjacent(&self, other: &Coord) -> bool {
        self.is_x_away(other, 1)
    }

    fn is_x_away(&self, other: &Coord, distance: i32) -> bool {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() == distance
    }

    fn is_boundary(&self, max: (i32, i32, i32)) -> bool {
        self.x == 0
            || self.x == max.0
            || self.y == 0
            || self.y == max.1
            || self.z == 0
            || self.z == max.2
    }
}

impl std::str::FromStr for Coord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',').map(|d| d.parse::<i32>().unwrap());

        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();

        Ok(Coord { x, y, z })
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<Coord>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(input.lines().map(|line| line.parse().unwrap()).collect())
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .iter()
                .map(|coord| {
                    6 - input
                        .iter()
                        .filter(|other| coord.is_adjacent(other))
                        .count() as i32
                })
                .sum::<i32>()
                .into(),
        )
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        fn inner(
            to_check: Coord,
            checking_now: &mut Vec<Coord>,
            visited: &Vec<Coord>,
            input: &Vec<Coord>,
            max: (i32, i32, i32),
        ) -> (bool, Vec<Coord>) {
            let mut result = Vec::new();

            if input.contains(&to_check) || checking_now.contains(&to_check) {
                return (true, result);
            }

            if visited.contains(&to_check) || to_check.is_boundary(max) {
                result.push(to_check);
                return (false, result);
            }

            checking_now.push(to_check.clone());

            let (bool_1, mut result_1) = inner(
                Coord::new(to_check.x - 1, to_check.y, to_check.z),
                checking_now,
                visited,
                input,
                max,
            );
            let (bool_2, mut result_2) = inner(
                Coord::new(to_check.x + 1, to_check.y, to_check.z),
                checking_now,
                visited,
                input,
                max,
            );
            let (bool_3, mut result_3) = inner(
                Coord::new(to_check.x, to_check.y - 1, to_check.z),
                checking_now,
                visited,
                input,
                max,
            );
            let (bool_4, mut result_4) = inner(
                Coord::new(to_check.x, to_check.y + 1, to_check.z),
                checking_now,
                visited,
                input,
                max,
            );
            let (bool_5, mut result_5) = inner(
                Coord::new(to_check.x, to_check.y, to_check.z - 1),
                checking_now,
                visited,
                input,
                max,
            );
            let (bool_6, mut result_6) = inner(
                Coord::new(to_check.x, to_check.y, to_check.z + 1),
                checking_now,
                visited,
                input,
                max,
            );

            result.push(to_check);
            result.append(&mut result_1);
            result.append(&mut result_2);
            result.append(&mut result_3);
            result.append(&mut result_4);
            result.append(&mut result_5);
            result.append(&mut result_6);

            (
                bool_1 && bool_2 && bool_3 && bool_4 && bool_5 && bool_6,
                result,
            )
        }

        let max_x = input.iter().map(|coord| coord.x).max().unwrap();
        let max_y = input.iter().map(|coord| coord.y).max().unwrap();
        let max_z = input.iter().map(|coord| coord.z).max().unwrap();

        let mut visited = Vec::new();
        let mut encapsulated = HashSet::new();

        for i in 1..max_x {
            for j in 1..max_y {
                for k in 1..max_z {
                    let current_coord = Coord::new(i, j, k);
                    if visited.contains(&current_coord) || input.contains(&current_coord) {
                        continue;
                    }

                    let (is_encapsulated, mut coords) = inner(
                        current_coord,
                        &mut Vec::new(),
                        &visited,
                        input,
                        (max_x, max_y, max_z),
                    );
                    if is_encapsulated {
                        encapsulated.extend(coords.clone());
                    }
                    visited.append(&mut coords);
                }
            }
        }

        let lava_edges = input
            .iter()
            .map(|coord| {
                6 - input
                    .iter()
                    .filter(|other| coord.is_adjacent(other))
                    .count() as i32
            })
            .sum::<i32>();

        let bubble_edges = encapsulated
            .iter()
            .map(|coord| {
                6 - encapsulated
                    .iter()
                    .filter(|other| coord.is_adjacent(other))
                    .count() as i32
            })
            .sum::<i32>();

        Some((lava_edges - bubble_edges).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: [(&str, i32, i32); 2] = [("2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5", 64, 58), ("1,1,1\n2,1,1\n3,1,1\n4,1,1\n5,1,1\n6,1,1\n1,2,1\n2,2,1\n3,2,1\n4,2,1\n5,2,1\n6,2,1\n1,3,1\n2,3,1\n3,3,1\n4,3,1\n5,3,1\n6,3,1\n1,1,2\n2,1,2\n3,1,2\n4,1,2\n5,1,2\n6,1,2\n1,2,2\n6,2,2\n1,3,2\n2,3,2\n3,3,2\n4,3,2\n5,3,2\n6,3,2\n1,1,3\n2,1,3\n3,1,3\n4,1,3\n5,1,3\n6,1,3\n1,2,3\n2,2,3\n3,2,3\n4,2,3\n5,2,3\n6,2,3\n1,3,3\n2,3,3\n3,3,3\n4,3,3\n5,3,3\n6,3,3", 108, 90)];

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        for (raw, expected, _) in TEST_INPUT {
            let input = sol.parse_input(raw);
            let result = sol.silver(&input.unwrap()).unwrap();
            assert_eq!(result, expected)
        }
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        for (raw, _, expected) in TEST_INPUT {
            let input = sol.parse_input(raw);
            let result = sol.gold(&input.unwrap()).unwrap();
            assert_eq!(result, expected)
        }
    }
}
