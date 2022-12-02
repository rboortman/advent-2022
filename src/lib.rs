mod assignment_1;
mod assignment_2;

use reqwest::header::{COOKIE, USER_AGENT};
use std::{fmt::Display, time::Instant};

pub fn solve(day: u8) {
    let raw_input = get_input(&day);
    match day {
        1 => assignment_1::Solution::new().run(raw_input),
        2 => assignment_2::Solution::new().run(raw_input),
        d => println!("Day {} has not been solved yet", d),
    }
}

pub trait Assignment {
    type Input;
    type Output: Display;

    fn parse_input(&self, input: &String) -> Option<Self::Input>;

    fn silver(&self, input: &Self::Input) -> Option<Self::Output>;
    fn gold(&self, input: &Self::Input) -> Option<Self::Output>;

    fn timed_silver(&self, input: &Self::Input) -> Option<(Self::Output, u128)> {
        let before = Instant::now();
        let solution = self.silver(input)?;

        Some((solution, before.elapsed().as_micros()))
    }

    fn timed_gold(&self, input: &Self::Input) -> Option<(Self::Output, u128)> {
        let before = Instant::now();
        let solution = self.gold(input)?;

        Some((solution, before.elapsed().as_micros()))
    }

    fn run(&self, input: String) {
        let parsed_silver = self
            .parse_input(&input)
            .expect("Could not parse silver input");
        let parsed_gold = self
            .parse_input(&input)
            .expect("Could not parse gold input");
        let (silver_answer, silver_time) = self
            .timed_silver(&parsed_silver)
            .expect("Error while solving silver");
        let (gold_answer, gold_time) = self
            .timed_gold(&parsed_gold)
            .expect("Error while solving gold");
        println!(
            "----------\n| Silver | {} ({} µs)\n----------\n| Gold   | {} ({} µs)\n----------\n",
            silver_answer, silver_time, gold_answer, gold_time
        );
    }
}

#[tokio::main]
async fn get_input(assignment_id: &u8) -> String {
    let mut data_location = project_root::get_project_root().unwrap();
    data_location.push("src");
    data_location.push("data");

    std::fs::create_dir_all(&data_location).expect("Could not create data dir");
    data_location.push(format!("input_{}.txt", assignment_id));

    let contents = match std::fs::read_to_string(&data_location) {
        Ok(c) => c,
        Err(_) => {
            let client = reqwest::Client::new();
            let contents = client
                .get(format!(
                    "https://adventofcode.com/2022/day/{}/input",
                    assignment_id
                ))
                .header(
                    COOKIE,
                    format!("session={}", dotenv::var("ADVENT_SESSION_KEY").unwrap()),
                )
                .header(
                    USER_AGENT,
                    String::from(
                        "User-Agent: github.com/rboortman/advent-2022 by ron@techforce1.nl",
                    ),
                )
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            let _ = std::fs::write(&data_location, &contents);
            contents
        }
    };

    contents
}
