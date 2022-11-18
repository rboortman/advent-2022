use advent_2022::Assignment;

pub struct Assignment1 {
    test_input: String,
}

impl Assignment1 {
    pub fn new() -> Assignment1 {
        Assignment1 {
            test_input: "199\n200\n208\n210\n200\n207\n240\n269\n260\n263".to_owned(),
        }
    }

    fn parse_input(&self, input: String) -> Vec<String> {
        input.lines().map(|s| s.to_owned()).collect()
    }

    fn silver(&self, input: &Vec<String>) -> String {
        "todo".to_owned()
    }

    fn gold(&self, input: &Vec<String>) -> String {
        "todo".to_owned()
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
