fn shortest_distance(list1: &Vec<isize>, list2: &Vec<isize>) -> isize {
    let mut sl1 = list1.clone();
    sl1.sort();
    let mut sl2 = list2.clone();
    sl2.sort();

    return sl1
        .iter()
        .zip(sl2.iter())
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum();
}

#[cfg(test)]
mod tests {
    use super::shortest_distance;
    use std::vec;

    #[test]
    fn it_works() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        let dist = shortest_distance(&list1, &list2);
        assert_eq!(dist, 11);
    }

    #[test]
    fn from_input() {
        use std::fs;
        let file_content = fs::read_to_string("src/day1/input.json").unwrap();
        let raw_data: Vec<(isize, isize)> = serde_json::from_str(&file_content).unwrap();
        let left_column: Vec<isize> = raw_data.iter().map(|(left, _)| *left).collect();
        let right_column: Vec<isize> = raw_data.iter().map(|(_, right)| *right).collect();

        let dist = shortest_distance(&left_column, &right_column);
        assert_eq!(dist, 2769675);
    }
}
