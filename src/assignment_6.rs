use crate::{Assignment, Output};

use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

fn is_unique(input: &VecDeque<&char>) -> bool {
    input
        .iter()
        .all(|c| input.iter().filter(|a| *a == c).count() == 1)
}

impl Assignment for Solution {
    type Input = VecDeque<char>;
    type Output = Output;

    fn parse_input(&self, input: &str) -> Option<Self::Input> {
        Some(input.chars().collect())
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut sliding_window = VecDeque::new();
        let mut result: usize = 0;

        for (i, c) in input.iter().enumerate() {
            if i < 4 {
                sliding_window.push_back(c);
                continue;
            }

            if is_unique(&sliding_window) {
                result = i;
                break;
            } else {
                sliding_window.pop_front();
                sliding_window.push_back(c)
            }
        }
        Some((result as u32).into())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut sliding_window = VecDeque::new();
        let mut result: usize = 0;

        for (i, c) in input.iter().enumerate() {
            if i < 14 {
                sliding_window.push_back(c);
                continue;
            }

            if is_unique(&sliding_window) {
                result = i;
                break;
            } else {
                sliding_window.pop_front();
                sliding_window.push_back(c)
            }
        }
        Some((result as u32).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: [(&str, i32, i32); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        for (raw, expected, _) in TEST_INPUT {
            let input = sol.parse_input(&raw.to_owned());
            let result = sol.silver(&input.unwrap()).unwrap();
            assert_eq!(result, expected)
        }
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        for (raw, _, expected) in TEST_INPUT {
            let input = sol.parse_input(&raw.to_owned());
            let result = sol.gold(&input.unwrap()).unwrap();
            assert_eq!(result, expected)
        }
    }
}
