use codeadvent24::days::day11::*;

#[test]
fn blink_works() {
    let stones = load("inputs/day11/input1.txt");
    assert_eq!(blink_count(&stones, 6), 22);
    assert_eq!(blink_count(&stones, 25), 55312);
}

#[test]
fn blink_inputs() {
    let stones = load("inputs/day11/input2.txt");
    assert_eq!(blink_count(&stones, 25), 183484);
    assert_eq!(blink_count(&stones, 75), 218817038947400);
}
