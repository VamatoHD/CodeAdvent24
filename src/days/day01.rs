pub fn shortest_distance(list1: &Vec<isize>, list2: &Vec<isize>) -> isize {
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

pub fn similarity(list1: &Vec<isize>, list2: &Vec<isize>) -> isize {
    list1
        .iter()
        .map(|x| {
            let mut count = 0;
            for v in list2.iter() {
                if *v == *x {
                    count += 1;
                }
            }
            x * count
        })
        .sum()
}
