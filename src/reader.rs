pub fn read(input: &'static str) -> (Vec<i32>, Vec<i32>) {
    let (list1, list2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|x| {
            let mut iter = x.split_whitespace().map(str::parse::<i32>);
            if let (Some(Ok(first)), Some(Ok(second))) = (iter.next(), iter.next()) {
                Some((first, second))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .unzip();

    (list1, list2)
}

pub fn read_2(input: &'static str) -> Vec<Vec<i32>> {
    let result: Vec<Vec<i32>> = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        
    result
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_read_2() {
        let v1 = 
            "7 6
             1 2";

        let result = read_2(v1);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0][0], 7);
        assert_eq!(result[0][1], 6);
        assert_eq!(result[1][0], 1);
        assert_eq!(result[1][1], 2);
    }
}
