use std::vec;

use codeadvent24::days::day01::*;
use codeadvent24::load;

#[test]
fn shortest_distance_works() {
    let list1 = vec![3, 4, 2, 1, 3, 3];
    let list2 = vec![4, 3, 5, 3, 9, 3];
    let dist = shortest_distance(&list1, &list2);
    assert_eq!(dist, 11);
}

#[test]
fn similarity_works() {
    let list1 = vec![3, 4, 2, 1, 3, 3];
    let list2 = vec![4, 3, 5, 3, 9, 3];
    let dist = similarity(&list1, &list2);
    assert_eq!(dist, 31);
}

#[test]
fn shortest_input() {
    let (left_column, right_column) = load::dual_column("inputs/day01.txt");

    let dist = shortest_distance(&left_column, &right_column);
    assert_eq!(dist, 2769675);
}

#[test]
fn similarity_input() {
    let (left_column, right_column) = load::dual_column("inputs/day01.txt");

    let similar = similarity(&left_column, &right_column);
    assert_eq!(similar, 24643097);
}
