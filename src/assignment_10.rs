use std::str::FromStr;

use crate::{Assignment, Output};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Add(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, value) = s.split_once(' ').unwrap_or((s, ""));
        match key {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Add(value.parse().unwrap())),
            _ => Err(format!("Could not parse {}", s)),
        }
    }
}

impl Into<(i32, i32)> for &Instruction {
    fn into(self) -> (i32, i32) {
        match self {
            Instruction::Noop => (0, 1),
            Instruction::Add(value) => (*value, 2),
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<Instruction>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(input.lines().map(|l| l.parse().unwrap()).collect())
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut result = 0;
        let mut x = 1;
        let mut cycle = 0;
        let mut window = 0;

        let window_start = 20;
        let window_size = 40;

        for instruction in input {
            let (d_x, d_cycle) = instruction.into();
            cycle += d_cycle;

            let new_window = (cycle + window_size - window_start) / window_size;
            if window != new_window {
                result += x * (window_start + window_size * window);
                window = new_window;
            }
            x += d_x;
        }

        Some((result).into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut screen = [' '; 240];
        let mut x: i32 = 1;
        let mut cycle: i32 = 0;

        for instruction in input {
            let (d_x, d_cycle) = instruction.into();

            for _ in 0..d_cycle {
                if ((cycle % 40) - x).abs() <= 1 {
                    screen[cycle as usize] = '█';
                }
                cycle += 1;
            }

            x += d_x;
        }

        let a = screen
            .chunks(40)
            .map(|arr| arr.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", a);

        Some((a).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 13140)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, "██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n███   ███   ███   ███   ███   ███   ███ \n████    ████    ████    ████    ████    \n█████     █████     █████     █████     \n██████      ██████      ██████      ████\n███████       ███████       ███████     ")
    }
}
