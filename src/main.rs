mod reader;
mod solutions;

use solutions::day4::*;

fn main() {
    let input = include_str!("inputs/day4.txt");
    let result = part2(input);

    println!("Result: {}", result);
}
