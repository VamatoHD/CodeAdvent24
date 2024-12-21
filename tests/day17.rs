use codeadvent24::days::day17::*;

#[test]
fn preocessor_works1() {
    let (reg, code) = load("inputs/day17/input1.txt");
    let res = compute(&reg, &code);
    assert_eq!(res, "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn preocessor_works2() {
    let (reg, code) = load("inputs/day17/input2.txt");
    let res = compute(&reg, &code);
    assert_eq!(res, "4,2,5,6,7,7,7,7,3,1,0");
}

#[test]
fn preocessor_input() {
    let (reg, code) = load("inputs/day17/input3.txt");
    let res = compute(&reg, &code);
    assert_eq!(res, "1,0,2,0,5,7,2,1,3");
}
