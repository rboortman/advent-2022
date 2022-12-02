use crate::Assignment;

pub enum Played {
    Rock,
    Paper,
    Scissors,
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

fn translate_played(first: &Played, second: &Played) -> i32 {
    let symbol_score = match second {
        Played::Rock => 1,
        Played::Paper => 2,
        Played::Scissors => 3,
    };

    let win_score = match first {
        Played::Rock => match second {
            Played::Rock => 3,
            Played::Paper => 6,
            Played::Scissors => 0,
        },
        Played::Paper => match second {
            Played::Rock => 0,
            Played::Paper => 3,
            Played::Scissors => 6,
        },
        Played::Scissors => match second {
            Played::Rock => 6,
            Played::Paper => 0,
            Played::Scissors => 3,
        },
    };

    symbol_score + win_score
}

impl Assignment for Solution {
    type Input = Vec<(Played, Played)>;
    type Output = i32;

    fn parse_input(&self, input: &String, parse_gold: bool) -> Option<Self::Input> {
        let mut result = Vec::new();
        for line in input.lines() {
            let (first, second) = line
                .split_once(" ")
                .expect(format!("Could not split line {}", line).as_str());

            let first_played = match first {
                "A" => Played::Rock,
                "B" => Played::Paper,
                "C" => Played::Scissors,
                s => panic!("Unknown symbol {s}"),
            };
            let second_played: Played;
            if parse_gold {
                second_played = match second {
                    "X" => match first_played {
                        Played::Rock => Played::Scissors,
                        Played::Paper => Played::Rock,
                        Played::Scissors => Played::Paper,
                    },
                    "Y" => match first_played {
                        Played::Rock => Played::Rock,
                        Played::Paper => Played::Paper,
                        Played::Scissors => Played::Scissors,
                    },
                    "Z" => match first_played {
                        Played::Rock => Played::Paper,
                        Played::Paper => Played::Scissors,
                        Played::Scissors => Played::Rock,
                    },
                    s => panic!("Unknown symbol {s}"),
                };
            } else {
                second_played = match second {
                    "X" => Played::Rock,
                    "Y" => Played::Paper,
                    "Z" => Played::Scissors,
                    s => panic!("Unknown symbol {s}"),
                };
            }

            result.push((first_played, second_played));
        }
        Some(result)
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(input.iter().map(|(a, b)| translate_played(a, b)).sum())
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(input.iter().map(|(a, b)| translate_played(a, b)).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned(), false);
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 15)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned(), true);
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 12)
    }
}
