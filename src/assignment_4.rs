use crate::{Assignment, Output};

#[derive(Debug)]
pub struct Elf {
    lower: i32,
    upper: i32,
}

impl Elf {
    fn new(lower: i32, upper: i32) -> Elf {
        Elf { lower, upper }
    }

    fn is_engulfed(&self, other: &Elf) -> bool {
        other.lower <= self.lower && self.upper <= other.upper
    }

    fn overlaps(&self, other: &Elf) -> bool {
        (other.lower <= self.lower && self.lower <= other.upper)
            || (other.lower <= self.upper && self.upper <= other.upper)
            || other.is_engulfed(self)
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<(Elf, Elf)>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        let mut result = Vec::new();
        for line in input.lines() {
            let (first, second) = line.split_once(',').unwrap();

            let (first_lower, first_upper) = first.split_once('-').unwrap();
            let first_elf = Elf::new(first_lower.parse().unwrap(), first_upper.parse().unwrap());

            let (second_lower, second_upper) = second.split_once('-').unwrap();
            let second_elf = Elf::new(second_lower.parse().unwrap(), second_upper.parse().unwrap());

            result.push((first_elf, second_elf));
        }
        Some(result)
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        let result = input
            .iter()
            .map(|(first, second)| first.is_engulfed(second) || second.is_engulfed(first))
            .filter(|b| *b)
            .count() as i32;
        Some(result.into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let result = input
            .iter()
            .map(|(first, second)| first.overlaps(second))
            .filter(|b| *b)
            .count() as i32;
        Some(result.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 2)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 4)
    }
}
