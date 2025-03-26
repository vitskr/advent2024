mod reader;
mod solutions;

use solutions::day3::*;

fn main() {
    let input = include_str!("inputs/day3.txt");
    let result = part2(input);

    println!("Result: {}", result);
}
