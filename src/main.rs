mod reader;
mod solutions;

use reader::{read_2};

use solutions::day2_2::run;

fn main() {
    let input = read_2(include_str!("inputs/day2_1.txt"));
    let result = run(input);

    println!("Result: {}", result);
}
