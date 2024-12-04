use codeadvent24::days::day04::*;
use codeadvent24::load::rows_to_chars;

#[test]
fn xmas_search_works() {
    let list = rows_to_chars("inputs/day04/test1.txt");
    let count = xmas_search(&list);
    assert_eq!(count, 18);
}

#[test]
fn xmas_search_input() {
    let list = rows_to_chars("inputs/day04/test2.txt");
    let count = xmas_search(&list);
    assert_eq!(count, 2483);
}

#[test]
fn x_mas_search_works() {
    let list = rows_to_chars("inputs/day04/test1.txt");
    let count = x_mas_search(&list);
    assert_eq!(count, 9);
}

#[test]
fn x_mas_search_input() {
    let list = rows_to_chars("inputs/day04/test2.txt");
    let count = x_mas_search(&list);
    assert_eq!(count, 1925);
}
