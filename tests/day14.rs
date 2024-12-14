use codeadvent24::days::day14::*;

#[test]
fn bridge_repair_works() {
    let tests = load("inputs/day14/test1.txt");
    let res = predict_robot(&tests, (11, 7), 100);
    assert_eq!(res, 12);
}

#[test]
fn bridge_repair_input() {
    let tests = load("inputs/day14/test2.txt");
    let res = predict_robot(&tests, (101, 103), 100);
    assert_eq!(res, 215476074);
}
