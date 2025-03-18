use itertools::Itertools;

pub fn is_safe(input: Vec<i32>) -> bool {    let mut sorted = input.clone();
    sorted.sort();
    sorted == input
}

pub fn run(input: Vec<Vec<i32>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]];

        let result = run(input);

        assert_eq!(result, 2);
    }
}
