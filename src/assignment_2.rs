use crate::Assignment;

pub enum Played {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Loss,
    Draw,
    Win,
}

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
}

fn translate_played(played: &Played) -> i32 {
    match played {
        Played::Rock => 1,
        Played::Paper => 2,
        Played::Scissors => 3,
    }
}
fn translate_outcome(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn translate_score(played: &Played, outcome: &Outcome) -> i32 {
    translate_played(played) + translate_outcome(outcome)
}

fn get_outcome(first: &Played, second: &Played) -> Outcome {
    match (first, second) {
        (Played::Rock, Played::Scissors) => Outcome::Loss,
        (Played::Rock, Played::Paper) => Outcome::Win,
        (Played::Paper, Played::Rock) => Outcome::Loss,
        (Played::Paper, Played::Scissors) => Outcome::Win,
        (Played::Scissors, Played::Paper) => Outcome::Loss,
        (Played::Scissors, Played::Rock) => Outcome::Win,
        _ => Outcome::Draw,
    }
}

fn get_played(first: &Played, outcome: &Outcome) -> Played {
    match (first, outcome) {
        (Played::Rock, Outcome::Loss) => Played::Scissors,
        (Played::Rock, Outcome::Draw) => Played::Rock,
        (Played::Rock, Outcome::Win) => Played::Paper,
        (Played::Paper, Outcome::Loss) => Played::Rock,
        (Played::Paper, Outcome::Draw) => Played::Paper,
        (Played::Paper, Outcome::Win) => Played::Scissors,
        (Played::Scissors, Outcome::Loss) => Played::Paper,
        (Played::Scissors, Outcome::Draw) => Played::Scissors,
        (Played::Scissors, Outcome::Win) => Played::Rock,
    }
}

impl Assignment for Solution {
    type Input = Vec<(Played, Played, Outcome)>;
    type Output = i32;

    fn parse_input(&self, input: &String) -> Option<Self::Input> {
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
            second_played = match second {
                "X" => Played::Rock,
                "Y" => Played::Paper,
                "Z" => Played::Scissors,
                s => panic!("Unknown symbol {s}"),
            };
            let out = match second {
                "X" => Outcome::Loss,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                s => panic!("Unknown symbol {s}"),
            };

            result.push((first_played, second_played, out));
        }
        Some(result)
    }

    fn silver(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .iter()
                .map(|(a, b, _outcome)| translate_score(b, &get_outcome(&a, &b)))
                .sum(),
        )
    }

    fn gold(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .iter()
                .map(|(a, _b, outcome)| translate_score(&get_played(&a, &outcome), &outcome))
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn test_silver() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.silver(&input.unwrap()).unwrap();
        assert_eq!(result, 15)
    }

    #[test]
    fn test_gold() {
        let sol = Solution::new();
        let input = sol.parse_input(&TEST_INPUT.to_owned());
        let result = sol.gold(&input.unwrap()).unwrap();
        assert_eq!(result, 12)
    }
}
