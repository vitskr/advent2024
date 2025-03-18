use itertools::Itertools;

pub fn is_safe(input: Vec<i32>) -> bool {
    let mut previous_distance : Option<i32> = None;
    for (previous,current) in input.iter().tuple_windows() {
        let distance = current - previous;
        if distance.abs() > 3 {
            return false;
        }

        if let Some(d) = previous_distance  {
            if d * distance < 0 {
                return false;
            } 
        }

        previous_distance = Some(distance);
    }

    return true;
        
}

pub fn run(input: Vec<Vec<i32>>) -> i32 {
    input.iter().fold(0, |c, row| {
        if is_safe(*row) { c + 1 } else { c }
    })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_safe() {
        let safe = vec![7, 6, 4, 2, 1];
        let not_safe_more_than_2 = vec![1, 2, 7, 8, 9];
        let not_safe_not_increasing = vec![1, 3, 2, 4, 5];

        assert_eq!(is_safe(safe), true, "Safe because the levels are all decreasing by 1 or 2");
        assert_eq!(is_safe(not_safe_more_than_2), false, "Unsafe because 2 7 is an increase of 5");
        assert_eq!(is_safe(not_safe_not_increasing), false, "Unsafe because 1 3 is increasing but 3 2 is decreasing");
    }

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
