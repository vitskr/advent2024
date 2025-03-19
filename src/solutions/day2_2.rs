use itertools::{concat, Itertools};

pub fn is_safe(input: &Vec<i32>) -> bool {
    let diffs = input
        .iter()
        .tuple_windows()        
        .map(|(a, b)| b -a);

    let result = diffs.fold((None::<i32>, 0), | (p, count), n | {
        let previous_signum = match p {
            Some(signum) => signum,
            None => n.signum(),
        };

        if n.abs() > 3 {
            return (Some(previous_signum), count + 1)
        }

        if previous_signum != n.signum() {
            return (Some(previous_signum), count + 1)
        }

        (Some(previous_signum), count)
    });

    match result {
        (_, wrong) => wrong <= 1,
        _ => false,
    }
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
    fn test_is_safe_safe_remove_second_level_4() {
        assert_eq!(
            is_safe(&vec![8, 6, 4, 4, 1]),
            true,
            "Safe by removing the second level, 4"
        );
    }
    
    #[test]
    fn test_is_safe_safe_remove_second_level() {
        assert_eq!(
            is_safe(&vec![1, 3, 6, 7, 9]),
            true,
            "Safe by removing the second level"
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
