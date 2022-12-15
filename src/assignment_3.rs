use crate::{Assignment, Output};

#[derive(Debug)]
pub struct Bag {
    left: [i32; 52],
    right: [i32; 52],
}

fn get_char_index(c: char) -> usize {
    match c.is_uppercase() {
        true => (c as usize) - 39,
        false => (c as usize) - 97,
    }
}

impl Bag {
    pub fn new(input: &str) -> Bag {
        let half_len = input.chars().count() / 2;
        let left_string = &input[..half_len];
        let right_string = &input[half_len..];

        let mut left = [0; 52];
        let mut right = [0; 52];

        for c in left_string.chars() {
            left[get_char_index(c)] += 1;
        }
        for c in right_string.chars() {
            right[get_char_index(c)] += 1;
        }

        Bag { left, right }
    }

    fn both_sides(&self) -> i32 {
        let mut index = -1;
        for (i, _) in self.left.iter().enumerate() {
            if self.has_item_left(i) && self.has_item_right(i) {
                index = i as i32;
                break;
            }
        }
        index + 1
    }

    fn has_item_left(&self, index: usize) -> bool {
        self.left[index] > 0
    }

    fn has_item_right(&self, index: usize) -> bool {
        self.right[index] > 0
    }

    fn has_item(&self, index: usize) -> bool {
        self.has_item_left(index) || self.has_item_right(index)
    }

    fn get_group_badge(&self, others: &[&Bag]) -> i32 {
        let mut index = -1;
        for (i, _) in self.left.iter().enumerate() {
            if self.has_item(i) && others.iter().all(|b| b.has_item(i)) {
                index = i as i32;
                break;
            }
        }
        if index < 0 {
            for (i, _) in self.right.iter().enumerate() {
                if self.has_item(i) && others.iter().all(|b| b.has_item(i)) {
                    index = i as i32;
                    break;
                }
            }
        }
        index + 1
    }
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<Bag>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        let mut result = Vec::new();
        for line in input.lines() {
            result.push(Bag::new(line));
        }
        Some(result)
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(input.iter().map(Bag::both_sides).sum::<i32>().into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .chunks(3)
                .map(|bags| bags[0].get_group_badge(&[&bags[1], &bags[2]]))
                .sum::<i32>()
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 157)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 70)
    }
}
