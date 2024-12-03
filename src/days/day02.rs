pub fn is_safe(list: &Vec<isize>) -> bool {
    let slope = list.windows(2).any(|x| (x[0] - x[1]).abs() > 3);

    let increasing = list.windows(2).all(|x| x[0] < x[1]);
    let decreasing = list.windows(2).all(|x| x[0] > x[1]);

    return !slope && (increasing || decreasing);
}

pub fn is_safe_by_removing(list: &Vec<isize>) -> bool {
    is_safe(list)
        || (0..list.len()).any(|i| {
            let mut l = list.clone();
            l.remove(i);
            is_safe(&l)
        })
}
