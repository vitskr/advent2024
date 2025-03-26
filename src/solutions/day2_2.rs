use itertools::{Itertools, concat};

use super::day2_1;

pub fn is_safe(input: &Vec<i32>) -> bool {
    if day2_1::is_safe(input) {
        return true;
    }

    for i in 0..input.len() {
        let mut copy = input.clone();
        copy.remove(i);

        if day2_1::is_safe(&copy) {
            return true;
        }
    }

    false
}
// pub fn is_safe(input: &Vec<i32>) -> bool {
//     let mut errors = 0;
//     let mut signum : Option<i32> = None;

//     let mut i = 0;
//     let mut y = 1;

//     while y < input.len() {
//         if errors > 1 {
//             break;
//         }

//         let distance = input[i] - input[y];

//         let previous_signum = match signum {
//             Some(x) => x,
//             None => distance.signum()
//         };

//         if distance.signum() != previous_signum ||
//             distance.abs() > 3
//         {
//             errors += 1;
//             y += 1;
//             continue;
//         }

//         signum = Some(distance.signum());

//         i += 1;
//         y += 1;
//     }

//     errors <= 1
// }

pub fn run(input: Vec<Vec<i32>>) -> i32 {
    let mut safe = 0;

    for row in &input {
        if is_safe(&row) {
            safe += 1;
        }
    }

    safe
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
            "Safe without removing any level"
        );
    }

    #[test]
    fn test_is_safe_unsafe_large_increase() {
        assert_eq!(
            is_safe(&vec![1, 2, 7, 8, 9]),
            false,
            "Unsafe regardless of which level is removed"
        );
    }

    #[test]
    fn test_is_safe_unsafe_large_decrease() {
        assert_eq!(
            is_safe(&vec![9, 7, 6, 2, 1]),
            false,
            "Unsafe regardless of which level is removed"
        );
    }

    #[test]
    fn test_is_safe_safe_remove_second_level_3() {
        assert_eq!(
            is_safe(&vec![1, 3, 2, 4, 5]),
            true,
            "Safe by removing the second level, 3"
        );
    }

    #[test]
    fn test_is_safe_safe_remove_third_level_4() {
        assert_eq!(
            is_safe(&vec![8, 6, 4, 4, 1]),
            true,
            "Safe by removing the third level, 4"
        );
    }

    #[test]
    fn test_is_safe_safe_remove_second_level() {
        assert_eq!(
            is_safe(&vec![1, 3, 6, 7, 9]),
            true,
            "Safe without removing any level"
        );
    }

    #[test]
    fn test_is_safe_by_removing_signum_change() {
        assert_eq!(
            is_safe(&vec![6, 10, 5, 3, 2]),
            true,
            "Safe by removing second level, chaning signum"
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

        assert_eq!(result, 4);
    }
}
