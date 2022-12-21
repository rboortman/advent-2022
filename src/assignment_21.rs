use std::collections::HashMap;

use crate::{Assignment, Output};

#[derive(Debug, Clone)]
pub enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Monkey {
    Expression(String, String, String, Operation),
    Number(String, i64),
}

impl std::str::FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s[..4].to_string();

        let digit_finder = regex::Regex::new(r"-?\d+").unwrap();
        if digit_finder.is_match(s) {
            let digit = digit_finder
                .captures(s)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            Ok(Monkey::Number(name, digit))
        } else {
            let expression_finder = regex::Regex::new(
                r"^\w+: (?P<monkey_left>\w+) (?P<expression>.) (?P<monkey_right>\w+)",
            )
            .unwrap();
            let caps = expression_finder.captures(s).unwrap();
            let monkey_left = caps.name("monkey_left").unwrap().as_str().to_string();
            let monkey_right = caps.name("monkey_right").unwrap().as_str().to_string();
            let expression = match caps.name("expression").unwrap().as_str() {
                "+" => Operation::Plus,
                "-" => Operation::Minus,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => panic!("Could not parse expression in string {}", s),
            };

            Ok(Monkey::Expression(
                name,
                monkey_left,
                monkey_right,
                expression,
            ))
        }
    }
}

fn solve_monkey_equation(name: &String, monkeys: &HashMap<String, Monkey>) -> (i64, bool) {
    match monkeys.get(name).unwrap() {
        Monkey::Number(name, number) => (*number, name == &String::from("humn")),
        Monkey::Expression(_, left, right, expression) => {
            let (left_evaluated, left_is_human) = solve_monkey_equation(left, monkeys);
            let (right_evaluated, right_is_human) = solve_monkey_equation(right, monkeys);

            match expression {
                Operation::Plus => (
                    left_evaluated + right_evaluated,
                    left_is_human || right_is_human,
                ),
                Operation::Minus => (
                    left_evaluated - right_evaluated,
                    left_is_human || right_is_human,
                ),
                Operation::Multiply => (
                    left_evaluated * right_evaluated,
                    left_is_human || right_is_human,
                ),
                Operation::Divide => (
                    left_evaluated / right_evaluated,
                    left_is_human || right_is_human,
                ),
            }
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
    type Input = HashMap<String, Monkey>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(
            input
                .lines()
                .map(|monkey| monkey.parse().unwrap())
                .map(|monkey| match &monkey {
                    Monkey::Expression(name, _, _, _) => (name.clone(), monkey),
                    Monkey::Number(name, _) => (name.clone(), monkey),
                })
                .collect(),
        )
    }

    fn silver(&self, monkeys: &Self::Input) -> Option<Self::Output> {
        Some(
            solve_monkey_equation(&String::from("root"), monkeys)
                .0
                .into(),
        )
    }

    fn gold(&self, monkeys: &Self::Input) -> Option<Self::Output> {
        fn what_to_shout(name: &String, monkeys: &HashMap<String, Monkey>, should_be: i64) -> i64 {
            if name == &String::from("humn") {
                should_be
            } else if let Monkey::Expression(_, left, right, expression) =
                monkeys.get(name).unwrap()
            {
                let (left_evaluated, left_is_human) = solve_monkey_equation(left, monkeys);
                let (right_evaluated, _) = solve_monkey_equation(right, monkeys);

                if left_is_human {
                    match expression {
                        Operation::Plus => {
                            what_to_shout(left, monkeys, should_be - right_evaluated)
                        }
                        Operation::Minus => {
                            what_to_shout(left, monkeys, should_be + right_evaluated)
                        }
                        Operation::Multiply => {
                            what_to_shout(left, monkeys, should_be / right_evaluated)
                        }
                        Operation::Divide => {
                            what_to_shout(left, monkeys, should_be * right_evaluated)
                        }
                    }
                } else {
                    match expression {
                        Operation::Plus => {
                            what_to_shout(right, monkeys, should_be - left_evaluated)
                        }
                        Operation::Minus => {
                            what_to_shout(right, monkeys, left_evaluated - should_be)
                        }
                        Operation::Multiply => {
                            what_to_shout(right, monkeys, should_be / left_evaluated)
                        }
                        Operation::Divide => {
                            what_to_shout(right, monkeys, left_evaluated / should_be)
                        }
                    }
                }
            } else {
                -1
            }
        }

        if let Monkey::Expression(_, left, right, _) = monkeys.get(&String::from("root")).unwrap() {
            let left_solve = solve_monkey_equation(left, monkeys);
            let right_solve = solve_monkey_equation(right, monkeys);

            if left_solve.1 {
                Some(what_to_shout(left, monkeys, right_solve.0).into())
            } else {
                Some(what_to_shout(right, monkeys, left_solve.0).into())
            }
        } else {
            Some((-1).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 152)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 301)
    }
}
