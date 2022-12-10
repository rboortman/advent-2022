mod assignment_1;
mod assignment_10;
mod assignment_2;
mod assignment_3;
mod assignment_4;
mod assignment_5;
mod assignment_6;
mod assignment_7;
mod assignment_8;
mod assignment_9;

use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE, USER_AGENT},
    redirect::Policy,
    Client,
};
use std::{
    fmt::{Display, Formatter, Result as DisplayResult},
    io,
    time::Instant,
};
use termion;

pub fn solve(day: u8) {
    let raw_input = get_input(&day);
    let (silver, gold) = match day {
        1 => assignment_1::Solution::new().run(raw_input),
        2 => assignment_2::Solution::new().run(raw_input),
        3 => assignment_3::Solution::new().run(raw_input),
        4 => assignment_4::Solution::new().run(raw_input),
        5 => assignment_5::Solution::new().run(raw_input),
        6 => assignment_6::Solution::new().run(raw_input),
        7 => assignment_7::Solution::new().run(raw_input),
        8 => assignment_8::Solution::new().run(raw_input),
        9 => assignment_9::Solution::new().run(raw_input),
        10 => assignment_10::Solution::new().run(raw_input),
        d => panic!("Day {} has not been solved yet", d),
    };

    let mut user_input = String::new();
    let stdin = io::stdin();
    println!(
        "Which answer would you like to commit? ({}{}s{}ilver/{}{}g{}old)",
        termion::style::Underline,
        termion::style::Bold,
        termion::style::Reset,
        termion::style::Underline,
        termion::style::Bold,
        termion::style::Reset,
    );
    stdin.read_line(&mut user_input).unwrap();

    let (level, answer) = match user_input.trim() {
        "s" | "silver" => (1, silver),
        "g" | "gold" => (2, gold),
        _ => {
            println!("nothing usefull");
            std::process::exit(0)
        }
    };

    send_answer(day, level, answer);
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

    fn run(&self, input: String) -> (Self::Output, Self::Output) {
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

        (silver_answer, gold_answer)
    }
}

fn build_client(session_cookie: &str, content_type: &str) -> Result<Client, String> {
    let cookie_header = HeaderValue::from_str(&format!("session={}", session_cookie.trim()))
        .map_err(|err| format!("Invalid session cookie: {}", err))?;
    let content_type_header = HeaderValue::from_str(content_type).unwrap();
    let user_agent = format!("User-Agent: github.com/rboortman/advent-2022 by ron@techforce1.nl");
    let user_agent_header = HeaderValue::from_str(&user_agent).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_header);
    headers.insert(CONTENT_TYPE, content_type_header);
    headers.insert(USER_AGENT, user_agent_header);

    Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()
        .map_err(|err| err.to_string())
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
            let client = build_client(
                dotenv::var("ADVENT_SESSION_KEY").unwrap().as_str(),
                "text/plain",
            )
            .unwrap();

            // let client = reqwest::Client::new();
            let contents = client
                .get(format!(
                    "https://adventofcode.com/2022/day/{}/input",
                    assignment_id
                ))
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

#[tokio::main]
async fn send_answer(day: u8, level: u8, answer: Output) {
    // let contents = std::fs::read_to_string("./src/temp.html").unwrap();
    let client = build_client(
        dotenv::var("ADVENT_SESSION_KEY").unwrap().as_str(),
        "application/x-www-form-urlencoded",
    )
    .unwrap();
    // let client = reqwest::Client::new();
    let response = client
        .post(format!("https://adventofcode.com/2022/day/{}/answer", day))
        .body(format!("level={}&answer={}", level, answer))
        .header(
            COOKIE,
            format!("session={}", dotenv::var("ADVENT_SESSION_KEY").unwrap()),
        )
        .header(
            USER_AGENT,
            String::from("User-Agent: github.com/rboortman/advent-2022 by ron@techforce1.nl"),
        )
        .send()
        .await
        .unwrap();

    // println!("{:?}", response.headers());
    if response.status().as_u16() == 302 {
        println!("Answer was already submitted!");
        std::process::exit(0);
    }

    let contents = response.text().await.unwrap();

    let document = scraper::Html::parse_document(contents.as_str());
    let selector = scraper::Selector::parse("article").unwrap();

    let article = document.select(&selector).next().unwrap();

    let response = article
        .text()
        .collect::<String>()
        .lines()
        .filter(|l| !l.trim().starts_with("["))
        .fold(String::from(""), |acc, line| acc + line.trim())
        .replace("You guessed", "You guessed: ");

    println!("{}", response);
}

macro_rules! impl_output_from {
    ( $( ($e:tt, $t:ty) ),* ) => {
        #[derive(Debug, Eq)]
        pub enum Output {
            $( $e($t), )*
        }

        $(
            impl From<$t> for Output {
                fn from(value: $t) -> Self {
                    Output::$e(value)
                }
            }
        )*
    };
}

impl_output_from! {
    (U8,     u8),
    (U16,    u16),
    (U32,    u32),
    (U64,    u64),
    (U128,   u128),
    (I8,     i8),
    (I16,    i16),
    (I32,    i32),
    (I64,    i64),
    (I128,   i128),
    (String, String),
    (Char,   char)
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Output::U8(v) => write!(f, "{v}"),
            Output::U16(v) => write!(f, "{v}"),
            Output::U32(v) => write!(f, "{v}"),
            Output::U64(v) => write!(f, "{v}"),
            Output::U128(v) => write!(f, "{v}"),
            Output::I8(v) => write!(f, "{v}"),
            Output::I16(v) => write!(f, "{v}"),
            Output::I32(v) => write!(f, "{v}"),
            Output::I64(v) => write!(f, "{v}"),
            Output::I128(v) => write!(f, "{v}"),
            Output::String(v) => write!(f, "{v}"),
            Output::Char(v) => write!(f, "{v}"),
        }
    }
}

/// Consider an output equal to any value where they can both be
/// coerced to the same string
impl<T: Display> PartialEq<T> for Output {
    fn eq(&self, other: &T) -> bool {
        *self.to_string() == other.to_string()
    }
}
