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
