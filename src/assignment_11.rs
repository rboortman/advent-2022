use std::str::FromStr;

use regex::Regex;

use crate::{Assignment, Output};

#[derive(Debug, Clone, Copy)]
enum MonkeyOperators {
    Add(u128),
    Multiplication(u128),
    MultiplicationSelf,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u128>,
    operation: MonkeyOperators,
    TestDivision: u128,
    FalseMonkey: usize,
    TrueMonkey: usize,
}

impl Monkey {
    fn exec_cycle(&self) -> (Monkey, Vec<(usize, u128)>) {
        let result = self
            .items
            .clone()
            .into_iter()
            .map(|item| {
                let new_item_value = match self.operation {
                    MonkeyOperators::Add(digit) => (item + digit) / 3,
                    MonkeyOperators::Multiplication(digit) => (item * digit) / 3,
                    MonkeyOperators::MultiplicationSelf => (item * item) / 3,
                };
                let tested_value = new_item_value / self.TestDivision;
                let new_monkey = if tested_value * self.TestDivision == new_item_value {
                    self.TrueMonkey
                } else {
                    self.FalseMonkey
                };
                (new_monkey, new_item_value)
            })
            .collect();
        (
            Monkey {
                items: Vec::new(),
                operation: self.operation,
                TestDivision: self.TestDivision,
                FalseMonkey: self.FalseMonkey,
                TrueMonkey: self.TrueMonkey,
            },
            result,
        )
    }

    fn add_item(&mut self, item: u128) {
        self.items.push(item);
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().skip(1).collect::<Vec<&str>>();

        let digit_regex = Regex::new(r"\d+").unwrap();

        let starting_items = digit_regex
            .find_iter(lines.get(0).unwrap())
            .map(|found| found.as_str().parse::<u128>().unwrap())
            .collect::<Vec<u128>>();

        let operator = &lines.get(1).unwrap().split(' ').collect::<Vec<&str>>()[4..=5];
        let bla = match operator[0] {
            "+" => MonkeyOperators::Add(operator[1].parse::<u128>().unwrap()),
            "*" => match operator[1].parse::<u128>() {
                Ok(digit) => MonkeyOperators::Multiplication(digit),
                Err(_) => MonkeyOperators::MultiplicationSelf,
            },
            _ => panic!("cannot parse {:?}", operator),
        };

        let test_division = *digit_regex
            .find_iter(lines.get(2).unwrap())
            .map(|found| found.as_str().parse::<u128>().unwrap())
            .collect::<Vec<u128>>()
            .get(0)
            .unwrap();
        let true_monkey = *digit_regex
            .find_iter(lines.get(3).unwrap())
            .map(|found| found.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .get(0)
            .unwrap();
        let false_monkey = *digit_regex
            .find_iter(lines.get(4).unwrap())
            .map(|found| found.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .get(0)
            .unwrap();

        Ok(Monkey {
            items: starting_items,
            operation: bla,
            TestDivision: test_division,
            TrueMonkey: true_monkey,
            FalseMonkey: false_monkey,
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
    type Input = Vec<Monkey>;
    type Output = Output;

    fn parse_input(&self, input: &String) -> Option<Self::Input> {
        Some(
            input
                .split("\n\n")
                .map(|monkey_string| monkey_string.parse().unwrap())
                .collect(),
        )
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut cloned_input = input.clone();
        for cycle in 0..20 {
            let mut monkeys = Vec::new();
            for monkey in &cloned_input {
                let cycle_result = monkey.exec_cycle();
                for (index, item) in cycle_result.1 {
                    &cloned_input[index].add_item(item);
                }

                monkeys.push(cycle_result.0);
            }

            cloned_input = monkeys;

            println!("Cyle: {}, {:?}", cycle, cloned_input);
        }
        println!("{:?}", input);
        Some((-1).into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        Some((-1).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 10605)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, -1)
    }
}
