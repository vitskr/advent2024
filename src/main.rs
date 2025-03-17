mod reader;
mod solutions;

use reader::read;

use solutions::day1_1::run;

fn main() {
    let (list1, list2) = read(include_str!("inputs/day1_a.txt"));
    let result = run(list1, list2);

    println!("Result: {}", result);
}
