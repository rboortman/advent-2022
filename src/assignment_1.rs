use crate::{Assignment, Output};

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<i32>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        let mut result = Vec::new();
        let mut current = 0;
        for line in input.lines() {
            match line {
                "" => {
                    result.push(current);
                    current = 0;
                }
                _ => current += line.parse::<i32>().unwrap(),
            }
        }
        result.push(current);
        Some(result)
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(input.iter().max().unwrap().to_owned().into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut clone_input = input.clone();
        clone_input.sort();
        Some(clone_input.iter().rev().take(3).sum::<i32>().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str =
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 24000)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 45000)
    }
}
