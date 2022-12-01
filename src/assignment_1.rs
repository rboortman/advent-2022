use advent_2022::Assignment;

pub struct Solution {
    test_input: String,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            test_input: String::from(
                "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n",
            ),
        }
    }

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

    fn silver(&self, input: &Vec<i32>) -> String {
        let largest = input.iter().max().unwrap();
        format!("{}", largest)
    }

    fn gold(&self, input: &Vec<i32>) -> String {
        let mut clone_input = input.clone();
        clone_input.sort();
        clone_input.reverse();
        format!("{}", &clone_input[..3].into_iter().sum::<i32>())
    }
}

impl Assignment for Solution {
    fn run(&self, input: String, is_debug: bool) -> (String, String) {
        let input = if is_debug {
            self.test_input.clone()
        } else {
            input
        };
        let parsed_input = self.parse_input(input);
        (self.silver(&parsed_input), self.gold(&parsed_input))
    }
}
