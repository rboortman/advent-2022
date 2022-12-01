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

    // let result: dyn Assignment = match assignment {
    //     1 => assignment_1::Solution::new(),
    //     3 => assignment_3::Solution::new(),
    //     _ => panic!("Couldn't find solutions for day {assignment}"),
    // };

    // let (silver, gold) = result.run(input);
}
