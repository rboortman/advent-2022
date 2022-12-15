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
    test_division: u128,
    false_monkey: usize,
    true_monkey: usize,
}

impl Monkey {
    fn exec_cycle(
        &self,
        other_items: &mut Vec<u128>,
        product: u128,
        is_worried: &bool,
    ) -> (Monkey, usize, Vec<(usize, u128)>) {
        let mut cloned_items = self.items.clone();
        cloned_items.append(other_items);
        let len = cloned_items.len();

        let result = cloned_items
            .into_iter()
            .map(|item| {
                let mut new_item_value = match self.operation {
                    MonkeyOperators::Add(digit) => item + digit,
                    MonkeyOperators::Multiplication(digit) => item * digit,
                    MonkeyOperators::MultiplicationSelf => item * item,
                };

                if !is_worried {
                    new_item_value /= 3;
                }

                new_item_value %= product;

                let tested_value = new_item_value / self.test_division;
                let new_monkey = if tested_value * self.test_division == new_item_value {
                    self.true_monkey
                } else {
                    self.false_monkey
                };
                (new_monkey, new_item_value)
            })
            .collect();
        (
            Monkey {
                items: Vec::new(),
                operation: self.operation,
                test_division: self.test_division,
                false_monkey: self.false_monkey,
                true_monkey: self.true_monkey,
            },
            len,
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
            .find_iter(lines.first().unwrap())
            .map(|found| found.as_str().parse::<u128>().unwrap())
            .collect::<Vec<u128>>();

        let operator = &lines.get(1).unwrap().split(' ').collect::<Vec<&str>>()[6..=7];
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
            .first()
            .unwrap();
        let true_monkey = *digit_regex
            .find_iter(lines.get(3).unwrap())
            .map(|found| found.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .first()
            .unwrap();
        let false_monkey = *digit_regex
            .find_iter(lines.get(4).unwrap())
            .map(|found| found.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .first()
            .unwrap();

        Ok(Monkey {
            items: starting_items,
            operation: bla,
            test_division,
            true_monkey,
            false_monkey,
        })
    }
}

fn throw_items(monkeys: Vec<Monkey>, is_worried: &bool) -> (Vec<Monkey>, Vec<usize>) {
    let mut new_monkeys = Vec::new();
    let mut new_items: Vec<(usize, u128)> = Vec::new();

    let product: u128 = monkeys.iter().map(|m| m.test_division).product();
    let mut total_inspected = vec![0; monkeys.len()];

    for (i, monkey) in monkeys.into_iter().enumerate() {
        let mut items_thrown_now = new_items
            .clone()
            .into_iter()
            .filter(|(j, _)| *j == i)
            .map(|(_, item)| item)
            .collect::<Vec<u128>>();

        let (new_monkey, inspected, items) =
            monkey.exec_cycle(&mut items_thrown_now, product, is_worried);
        new_monkeys.push(new_monkey);
        total_inspected[i] += inspected;

        for (j, item) in items {
            if j < i {
                new_monkeys[j].add_item(item);
            } else {
                new_items.push((j, item));
            }
        }
    }
    (new_monkeys, total_inspected)
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

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(
            input
                .split("\n\n")
                .map(|monkey_string| monkey_string.parse().unwrap())
                .collect(),
        )
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut cloned_input = input.clone();
        let mut inspected = vec![0; cloned_input.len()];

        for _ in 0..20 {
            let (new_monkeys, new_inspected) = throw_items(cloned_input, &false);
            cloned_input = new_monkeys;

            for (j, insp) in new_inspected.into_iter().enumerate() {
                inspected[j] += insp;
            }
        }

        inspected.sort();
        inspected.reverse();

        Some(
            (inspected
                .into_iter()
                .take(2)
                .map(|i| i as u32)
                .product::<u32>())
            .into(),
        )
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut cloned_input = input.clone();
        let mut inspected = vec![0; cloned_input.len()];

        for _ in 0..10_000 {
            let (new_monkeys, new_inspected) = throw_items(cloned_input, &true);
            cloned_input = new_monkeys;

            for (j, insp) in new_inspected.into_iter().enumerate() {
                inspected[j] += insp;
            }
        }

        inspected.sort();
        inspected.reverse();

        Some(
            (inspected
                .into_iter()
                .take(2)
                .map(|i| i as u128)
                .product::<u128>())
            .into(),
        )
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
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 10605)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        let tested_result: u32 = 2713310158;
        assert_eq!(result, tested_result)
    }
}
