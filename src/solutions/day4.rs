use regex::Regex;
use regex::RegexSet;

pub fn part1(input: &str) -> i32 {
    println!("Input: {}", input);
    0
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"), 18);
    }

    #[test]
    fn test_part2() {

    }
}
