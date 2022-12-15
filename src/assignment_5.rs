use regex::Regex;
use std::collections::VecDeque;

use crate::{Assignment, Output};

#[derive(Debug, Clone)]
pub struct Crate {
    id: char,
}

impl Crate {
    fn new(c: char) -> Crate {
        Crate { id: c }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn new(amount: usize, from: usize, to: usize) -> Instruction {
        Instruction { amount, from, to }
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = (VecDeque<VecDeque<Crate>>, VecDeque<Instruction>);
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        let mut stacks = VecDeque::new();
        let mut instructions = VecDeque::new();
        let mut convert_stacks = true;
        let total_stacks = (input.find('\n').unwrap() + 1) / 4;

        let instruction_re =
            Regex::new(r"^move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();

        for _ in 0..total_stacks {
            stacks.push_back(VecDeque::new());
        }

        for line in input.lines() {
            if line.is_empty() {
                convert_stacks = false;
                continue;
            }

            if convert_stacks {
                for (index, chunks) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                    if chunks[0] == '[' {
                        stacks[index].push_front(Crate::new(chunks[1]));
                    }
                }
            } else {
                let cap = instruction_re.captures(line).unwrap();
                instructions.push_back(Instruction::new(
                    cap.name("amount").unwrap().as_str().parse().unwrap(),
                    cap.name("from").unwrap().as_str().parse::<usize>().unwrap() - 1,
                    cap.name("to").unwrap().as_str().parse::<usize>().unwrap() - 1,
                ))
            }
        }

        Some((stacks, instructions))
    }

    fn silver(&self, (s, instructions): &Self::Input) -> Option<Self::Output> {
        let mut stacks = s.clone();

        for instruction in instructions {
            for _ in 0..instruction.amount {
                let cr = stacks[instruction.from].pop_back().unwrap();
                stacks[instruction.to].push_back(cr);
            }
        }

        let result = stacks
            .iter()
            .map(|stack| stack.back().unwrap().id)
            .collect::<String>();
        Some(result.into())
    }

    fn gold(&self, (s, instructions): &Self::Input) -> Option<Self::Output> {
        let mut stacks = s.clone();

        for instruction in instructions {
            let stack_length = stacks[instruction.from].len() - instruction.amount;
            let mut crates = stacks[instruction.from].split_off(stack_length);
            stacks[instruction.to].append(&mut crates);
        }

        let result = stacks
            .iter()
            .map(|stack| stack.back().unwrap().id)
            .collect::<String>();
        Some(result.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, String::from("CMZ"))
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, String::from("MCD"))
    }
}
