use advent_2022::Assignment;

pub struct Assignment1 {
    test_input: String,
}

impl Assignment1 {
    pub fn new() -> Assignment1 {
        Assignment1 {
            test_input: String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"),
        }
    }

    fn parse_input(&self, input: String) -> Vec<i32> {
        input.lines().map(|s| s.parse().unwrap()).collect()
    }

    fn silver(&self, input: &Vec<i32>) -> String {
        let mut count = 0;
        for (i, height) in input.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if height > input.get(i - 1).unwrap() {
                count = count + 1;
            }
        }
        format!("{}", count)
    }

    fn gold(&self, input: &Vec<i32>) -> String {
        let mut count = 0;
        for (i, height) in input.iter().enumerate() {
            if i < 3 {
                continue;
            }
            if height > input.get(i - 3).unwrap() {
                count = count + 1;
            }
        }
        format!("{}", count)
    }
}

impl Assignment for Assignment1 {
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
