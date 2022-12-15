use std::hash::Hash;
use std::ops::Add;
use std::{collections::HashSet, str::FromStr};

use crate::{Assignment, Output};

pub enum Direction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, magnitude) = s.split_once(' ').unwrap();
        let magnitude = magnitude.parse::<i32>().unwrap();

        match direction {
            "L" => Ok(Direction::Left(magnitude)),
            "R" => Ok(Direction::Right(magnitude)),
            "U" => Ok(Direction::Up(magnitude)),
            "D" => Ok(Direction::Down(magnitude)),
            _ => Err(format!("unknown direction {}", direction)),
        }
    }
}

impl Into<(Coord, i32)> for &Direction {
    fn into(self) -> (Coord, i32) {
        match self {
            Direction::Left(times) => (Coord::new(-1, 0), *times),
            Direction::Right(times) => (Coord::new(1, 0), *times),
            Direction::Up(times) => (Coord::new(0, 1), *times),
            Direction::Down(times) => (Coord::new(0, -1), *times),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    fn move_towards(&self, other: &Coord) -> Coord {
        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;

        let (new_x, new_y) = match (diff_x.abs(), diff_y.abs()) {
            (2, 0) => (self.x + (diff_x / 2), self.y),
            (2, 1) => (self.x + (diff_x / 2), self.y + diff_y),
            (2, 2) => (self.x + (diff_x / 2), self.y + (diff_y / 2)),
            (1, 2) => (self.x + diff_x, self.y + (diff_y / 2)),
            (0, 2) => (self.x, self.y + (diff_y / 2)),
            _ => (self.x, self.y),
        };

        Coord { x: new_x, y: new_y }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

fn calculate_knot_movement(input: &Vec<Direction>, knot_size: usize) -> usize {
    let mut set = HashSet::new();
    let mut knots = vec![Coord::new(0, 0); knot_size];

    for dir in input {
        let (change_vector, times) = dir.into();

        for _ in 0..times {
            let mut new_knots = vec![*knots.first().unwrap() + change_vector];
            for (i, knot) in knots.into_iter().enumerate().skip(1) {
                new_knots.push(knot.move_towards(&new_knots[i - 1]))
            }
            knots = new_knots;
            set.insert(*knots.last().unwrap());
        }
    }

    set.len()
}

impl Assignment for Solution {
    type Input = Vec<Direction>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(input.lines().map(|l| l.parse().unwrap()).collect())
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        Some((calculate_knot_movement(input, 2) as i32).into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        Some((calculate_knot_movement(input, 10) as i32).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    static LARGE_TEST_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 13)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 1)
    }

    #[test]
    fn test_large_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&LARGE_TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 36)
    }
}
