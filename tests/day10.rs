use codeadvent24::days::day10::*;

#[test]
fn trail_counter_works1() {
    let map = load("inputs/day10/input1.txt");
    let res = get_trailheads(&map);
    assert_eq!(res, 1);
}

#[test]
fn trail_counter_works2() {
    let map = load("inputs/day10/input2.txt");
    let res = get_trailheads(&map);
    assert_eq!(res, 36);
}

#[test]
fn trail_counter_input() {
    let map = load("inputs/day10/input3.txt");
    let res = get_trailheads(&map);
    assert_eq!(res, 582);
}

#[test]
fn trail_rater_works1() {
    let map = load("inputs/day10/input1.txt");
    let res = get_trail_rating(&map);
    assert_eq!(res, 16);
}

#[test]
fn trail_rater_works2() {
    let map = load("inputs/day10/input2.txt");
    let res = get_trail_rating(&map);
    assert_eq!(res, 81);
}

#[test]
fn trail_rater_input() {
    let map = load("inputs/day10/input3.txt");
    let res = get_trail_rating(&map);
    assert_eq!(res, 1302);
}
