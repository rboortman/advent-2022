use dotenv;

use advent_2022::solve;

fn main() {
    dotenv::dotenv().ok();

    let mut arguments = std::env::args().skip(1);
    let assignment: u8 = arguments
        .next()
        .expect("No 'assignment' input found")
        .parse()
        .expect("No assignment number given!");

    solve(assignment);
}
