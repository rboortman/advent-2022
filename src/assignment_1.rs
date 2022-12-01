use advent_2022::Assignment;

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(&self, input: String) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = 0;
        for line in input.lines() {
            match line {
                "" => {
                    result.push(current);
                    current = 0;
                }
                _ => current = current + line.parse::<i32>().unwrap(),
            }
        }
        result.push(current);
        result
    }

    fn silver(&self, input: &Vec<i32>) -> i32 {
        input.iter().max().unwrap().to_owned()
    }

    fn gold(&self, input: &Vec<i32>) -> i32 {
        let mut clone_input = input.clone();
        clone_input.sort();
        clone_input.iter().rev().take(3).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silver() {
        let test_input =
            String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n");
        let sol = Solution::new();
        let input = sol.parse_input(test_input);
        let result = sol.silver(&input);
        assert_eq!(result, 24000)
    }

    #[test]
    fn test_gold() {
        let test_input =
            String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n");
        let sol = Solution::new();
        let input = sol.parse_input(test_input);
        let result = sol.gold(&input);
        assert_eq!(result, 45000)
    }
}
