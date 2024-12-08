use codeadvent24::days::day07::*;

#[test]
fn bridge_repair_works() {
    let tests = load("inputs/day07/test1.txt");
    let res = repair_brige(&tests, 2);
    assert_eq!(res, 3749);
}

#[test]
fn bridge_repair_input() {
    let tests = load("inputs/day07/test2.txt");
    let res = repair_brige(&tests, 2);
    assert_eq!(res, 1289579105366);
}

#[test]
fn better_bridge_repair_works() {
    let tests = load("inputs/day07/test1.txt");
    let res = repair_brige(&tests, 3);
    assert_eq!(res, 11387);
}

// To slow
//#[test]
//fn better_bridge_repair_input() {
//    let tests = load("inputs/day07/test2.txt");
//    let res = repair_brige(&tests,3);
//    assert_eq!(res, 92148721834692);
//}
