use codeadvent24::days::day06::*;

#[test]
fn guard_movement_works() {
    let (map, guard) = load("inputs/day06/test1.txt");
    let res = move_guard(&map, guard);
    assert_eq!(res, 41);
}
#[test]
fn guard_movement_input() {
    let (map, guard) = load("inputs/day06/test2.txt");
    let res = move_guard(&map, guard);
    assert_eq!(res, 4602);
}

#[test]
fn guard_boundaries_works() {
    let (map, guard) = load("inputs/day06/test1.txt");
    let res = get_boundaries(&map, guard);
    assert_eq!(res, 6);
}

#[test]
fn guard_boundaries_input() {
    let (map, guard) = load("inputs/day06/test2.txt");
    let res = get_boundaries(&map, guard);
    assert_eq!(res, 1703);
}
