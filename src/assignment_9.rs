use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

use crate::{Assignment, Output};

pub enum Direction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl From<(&str, &str)> for Direction {
    fn from(tup: (&str, &str)) -> Self {
        let speed = tup.1.parse::<i32>().unwrap_or(0);
        match tup.0 {
            "U" => Direction::Up(speed),
            "R" => Direction::Right(speed),
            "D" => Direction::Down(speed),
            "L" => Direction::Left(speed),
            _ => Direction::Up(0),
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

impl Assignment for Solution {
    type Input = Vec<Direction>;
    type Output = Output;

    fn parse_input(&self, input: &String) -> Option<Self::Input> {
        Some(
            input
                .lines()
                .map(|l| Direction::from(l.split_once(' ').unwrap()))
                .collect(),
        )
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut set = HashSet::new();
        let mut knots = vec![Coord::new(0, 0); 2];

        for dir in input {
            let (change_vector, times) = match dir {
                Direction::Up(t) => (Coord::new(1, 0), t.to_owned()),
                Direction::Right(t) => (Coord::new(0, 1), t.to_owned()),
                Direction::Down(t) => (Coord::new(-1, 0), t.to_owned()),
                Direction::Left(t) => (Coord::new(0, -1), t.to_owned()),
            };

            for _ in 0..times {
                let mut new_knots = Vec::new();
                for (i, knot) in knots.into_iter().enumerate() {
                    if i == 0 {
                        new_knots.push(knot + change_vector);
                    } else {
                        new_knots.push(knot.move_towards(&new_knots[i - 1]))
                    }
                }
                knots = new_knots;
                set.insert(knots.get(knots.len() - 1).unwrap().clone());
            }
        }

        Some((set.len() as i32).into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut set = HashSet::new();
        let mut knots = vec![Coord::new(0, 0); 10];

        for dir in input {
            let (change_vector, times) = match dir {
                Direction::Up(t) => (Coord::new(1, 0), t.to_owned()),
                Direction::Right(t) => (Coord::new(0, 1), t.to_owned()),
                Direction::Down(t) => (Coord::new(-1, 0), t.to_owned()),
                Direction::Left(t) => (Coord::new(0, -1), t.to_owned()),
            };

            for _ in 0..times {
                let mut new_knots = Vec::new();
                for (i, knot) in knots.into_iter().enumerate() {
                    if i == 0 {
                        new_knots.push(knot + change_vector);
                    } else {
                        new_knots.push(knot.move_towards(&new_knots[i - 1]))
                    }
                }
                knots = new_knots;
                set.insert(knots.get(knots.len() - 1).unwrap().clone());
            }
        }

        Some((set.len() as i32).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    static LARGE_TEST_INPUT: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 13)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
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
