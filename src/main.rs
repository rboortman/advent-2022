mod assignment_1;

use dotenv;
use project_root;
use reqwest::header::COOKIE;

use advent_2022::Assignment;
use assignment_1::Assignment1;

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
                    "https://adventofcode.com/2021/day/{}/input",
                    assignment_id
                ))
                .header(
                    COOKIE,
                    format!("session={}", dotenv::var("ADVENT_SESSION_KEY").unwrap()),
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
    let debug_case = match arguments.next() {
        None => false,
        _ => true,
    };

    let input = get_input(&assignment);
    let result = match assignment {
        1 => {
            let ass = Assignment1::new();
            ass.run(input, debug_case)
        }
        _ => (
            String::from("No answer found"),
            String::from("No answer found"),
        ),
    };
    println!(
        "----------\n| Silver | {}\n----------\n| Gold   | {}\n----------\n",
        result.0, result.1
    )
}
