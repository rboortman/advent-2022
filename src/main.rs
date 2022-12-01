mod assignment_1;

use dotenv;
use reqwest::header::{COOKIE, USER_AGENT};

use advent_2022::Assignment;

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

fn main() {
    dotenv::dotenv().ok();

    let mut arguments = std::env::args().skip(1);
    let assignment: u8 = arguments
        .next()
        .expect("No 'assignment' input found")
        .parse()
        .expect("No assignment number given!");

    let input = get_input(&assignment);
    let result = match assignment {
        1 => {
            // let sol = assignment_1::Solution::new();
            // sol.run(input)
            assignment_1::Solution::new()
        }
        _ => panic!("Couldn't find solutions for day {assignment}"),
    };

    let (silver, gold) = result.run(input);
    println!(
        "----------\n| Silver | {}\n----------\n| Gold   | {}\n----------\n",
        silver, gold
    )
}
