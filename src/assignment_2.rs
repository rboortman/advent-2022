use advent_2022::Assignment;

enum Direction {
    Up,
    Down,
    Forward,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
            Direction::Forward => write!(f, "forward"),
        }
    }
}
pub struct Solution {
    test_input: String,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            test_input: String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n"),
        }
    }

    fn parse_input(&self, input: String) -> Vec<(Direction, i32)> {
        input
            .lines()
            .map(|s| s.split_once(' ').unwrap())
            .map(|(k, v)| {
                (
                    match k {
                        "up" => Direction::Up,
                        "down" => Direction::Down,
                        "forward" => Direction::Forward,
                        _ => panic!("Could not parse direction"),
                    },
                    v.parse::<i32>().unwrap(),
                )
            })
            .collect()
    }

    fn silver(&self, input: &Vec<(Direction, i32)>) -> String {
        let mut forward = 0;
        let mut down = 0;

        for (dir, speed) in input {
            match dir {
                Direction::Up => down = down - speed,
                Direction::Down => down = down + speed,
                Direction::Forward => forward = forward + speed,
            }
        }

        format!("{}", forward * down)
    }
    fn gold(&self, input: &Vec<(Direction, i32)>) -> String {
        let mut forward = 0;
        let mut down = 0;
        let mut aim = 0;

        for (dir, speed) in input {
            match dir {
                Direction::Up => aim = aim - speed,
                Direction::Down => aim = aim + speed,
                Direction::Forward => {
                    forward = forward + speed;
                    down = down + speed * aim;
                }
            }
        }

        format!("{}", forward * down)
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
