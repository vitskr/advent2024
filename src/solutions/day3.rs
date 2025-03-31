use regex::Regex;
use regex::RegexSet;

pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?<fst>\d{1,3}),(?<snd>\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|c| {
            let fst = c.name("fst").unwrap().as_str().parse::<i32>().unwrap();
            let snd = c.name("snd").unwrap().as_str().parse::<i32>().unwrap();

            fst * snd
        })
        .sum()
}

enum Operation {
    Mul(i32),
    Do,
    Dont,
}

pub fn part2(input: &str) -> i32 {
    let re =
        Regex::new(r"(mul\((?<fst>\d{1,3}),(?<snd>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\)))")
            .unwrap();

    let operations = re.captures_iter(input).map(|c| {
        if c.name("do").is_some() {
            return Operation::Do;
        }

        if c.name("dont").is_some() {
            return Operation::Dont;
        }

        let fst = c.name("fst").unwrap().as_str().parse::<i32>().unwrap();
        let snd = c.name("snd").unwrap().as_str().parse::<i32>().unwrap();

        Operation::Mul(fst * snd)
    });

    let mut current_multilplier = 1;
    let mut sum = 0;

    for op in operations {
        match op {
            Operation::Mul(x) => {
                sum += current_multilplier * x;
            }
            Operation::Do => {
                current_multilplier = 1;
            }
            Operation::Dont => {
                current_multilplier = 0;
            }
        }
    }

    println!("{:?}", sum);

    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)mul(8111,5345))";
        let result = part1(input);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part2(input);

        assert_eq!(result, 48);
    }
}
