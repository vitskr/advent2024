mod reader;
mod solutions;

use solutions::day4::*;

fn main() {
    let input = include_str!("inputs/day3.txt");
    let result = part1(input);

    println!("Result: {}", result);
}
