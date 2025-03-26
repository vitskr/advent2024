use itertools::Itertools;

// pub fn is_safe(input: &Vec<i32>) -> bool {
//     input
//         .iter()
//         .tuple_windows()
//         .map(|(a, b)| {
//             let diff = b - a;
//             if diff.abs() > 3 {
//                 None
//             } else {
//                 Some(diff.signum())
//             }
//         })
//         .all_equal()
// }

pub fn is_safe(input: &Vec<i32>) -> bool {
    let sig = (input[0] - input[1]).signum();

    for i in 1..input.len() {
        let diff = input[i - 1] - input[i];

        if diff.abs() > 3 || diff.abs() == 0 || diff.signum() != sig {
            return false;
        }
    }

    true
}

pub fn run(input: Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .fold(0, |c, row| if is_safe(row) { c + 1 } else { c })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_safe_all_decreasing() {
        assert_eq!(
            is_safe(&vec![7, 6, 4, 2, 1]),
            true,
            "Safe because the levels are all decreasing by 1 or 2"
        );
    }

    #[test]
    fn test_is_safe_unsafe_2_7_increase_of_5() {
        assert_eq!(
            is_safe(&vec![1, 2, 7, 8, 9]),
            false,
            "Unsafe because 2 7 is an increase of 5"
        );
    }

    #[test]
    fn test_is_safe_unsafe_6_2_decrease_of_4() {
        assert_eq!(
            is_safe(&vec![9, 7, 6, 2, 1]),
            false,
            "Unsafe because 6 2 is a decrease of 4"
        );
    }

    #[test]
    fn test_is_safe_unsafe_1_3_increase_but_3_2_is_dec() {
        assert_eq!(
            is_safe(&vec![1, 3, 2, 4, 5]),
            false,
            "Unsafe because 1 3 is increasing but 3 2 is decreasing"
        );
    }

    #[test]
    fn test_is_safe_unsafe_4_4_neither_inc_or_dec() {
        assert_eq!(
            is_safe(&vec![8, 6, 4, 4, 1]),
            false,
            "Unsafe because 4 4 is neither an increase or a decrease"
        );
    }

    #[test]
    fn test_is_safe_because_all_inc() {
        assert_eq!(
            is_safe(&vec![1, 3, 6, 7, 9]),
            true,
            "Unsafe because 4 4 is neither an increase or a decrease"
        );
    }

    #[test]
    fn test() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let result = run(input);

        assert_eq!(result, 2);
    }
}
