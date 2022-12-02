use crate::Assignment;

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

impl Assignment for Solution {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(&self, input: String) -> Option<Self::Input> {
        None
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        None
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, -1)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, -1)
    }
}
