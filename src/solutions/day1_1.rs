use itertools::Itertools;

pub fn run(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    list1
        .iter()
        .sorted()
        .zip(list2.iter().sorted())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(run(v1, v2), 11);
    }
}
