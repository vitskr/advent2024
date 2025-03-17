use itertools::Itertools;

pub fn run(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let grouped = list2.into_iter()
        .into_grouping_map_by(|&n| n)
        .fold(0,|acc, _key, _val| acc + 1);

    list1.into_iter()
        .map(|&n| {
            let a = match grouped.get(n) {
                Some( count) => count,
                None => 0
            };

        })
        .collect();

    println!("{:?}", grouped);

    0

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(run(v1, v2), 31);
    }
}
