use crate::Assignment;

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
        for (i, c) in self.left.iter().enumerate() {
            if c <= &0 {
                continue;
            } else if self.right[i] <= 0 {
                continue;
            } else {
                index = i as i32;
                break;
            }
        }
        index + 1
    }

    fn has_item(&self, index: usize) -> bool {
        self.left[index] > 0 || self.right[index] > 0
    }

    fn get_group_badge(&self, b1: &Bag, b2: &Bag) -> i32 {
        let mut index = -1;
        for (i, _) in self.left.iter().enumerate() {
            if self.has_item(i) && b1.has_item(i) && b2.has_item(i) {
                index = i as i32;
                break;
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
    type Output = i32;

    fn parse_input(&self, input: &String) -> Option<Self::Input> {
        let mut result = Vec::new();
        for line in input.lines() {
            result.push(Bag::new(line));
        }
        Some(result)
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(input.iter().map(|bag| bag.both_sides()).sum::<i32>())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut result = 0;
        for i in 0..(input.len() / 3) {
            result += input
                .get(i * 3)
                .unwrap()
                .get_group_badge(input.get(i * 3 + 1).unwrap(), input.get(i * 3 + 2).unwrap());
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 157)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 70)
    }
}
