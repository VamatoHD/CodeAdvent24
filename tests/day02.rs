use std::vec;

use codeadvent24::days::day02::*;
use codeadvent24::load;

#[test]
fn safest_works() {
    let l1 = vec![7, 6, 4, 2, 1];
    let l2 = vec![1, 2, 7, 8, 9];
    let l3 = vec![9, 7, 6, 2, 1];
    let l4 = vec![1, 3, 2, 4, 5];
    let l5 = vec![8, 6, 4, 4, 1];
    let l6 = vec![1, 3, 6, 7, 9];

    assert_eq!(is_safe(&l1), true);
    assert_eq!(is_safe(&l2), false);
    assert_eq!(is_safe(&l3), false);
    assert_eq!(is_safe(&l4), false);
    assert_eq!(is_safe(&l5), false);
    assert_eq!(is_safe(&l6), true);
}

#[test]
fn safest_input() {
    let tests: Vec<Vec<isize>> = load::rows("inputs/day02.txt");
    let mut res = 0;

    for value in &tests {
        if is_safe(value) {
            res += 1
        }
    }

    assert_eq!(res, 490)
}

#[test]
fn safest_removing_works() {
    let l1 = vec![7, 6, 4, 2, 1];
    let l2 = vec![1, 2, 7, 8, 9];
    let l3 = vec![9, 7, 6, 2, 1];
    let l4 = vec![1, 3, 2, 4, 5];
    let l5 = vec![8, 6, 4, 4, 1];
    let l6 = vec![1, 3, 6, 7, 9];

    assert_eq!(is_safe_by_removing(&l1), true);
    assert_eq!(is_safe_by_removing(&l2), false);
    assert_eq!(is_safe_by_removing(&l3), false);
    assert_eq!(is_safe_by_removing(&l4), true);
    assert_eq!(is_safe_by_removing(&l5), true);
    assert_eq!(is_safe_by_removing(&l6), true);
}

#[test]
fn safest_removing_input() {
    let tests: Vec<Vec<isize>> = load::rows("inputs/day02.txt");
    let mut res = 0;

    for value in &tests {
        if is_safe_by_removing(value) {
            res += 1
        }
    }

    assert_eq!(res, 536)
}
