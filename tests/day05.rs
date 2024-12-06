use codeadvent24::days::day05::*;

#[test]
fn print_queue_works() {
    let (rules, tests) = load("inputs/day05/test1.txt");
    let res = evaluate_print(&tests, &rules);
    assert_eq!(res, 143);
}

#[test]
fn print_queue_input() {
    let (rules, tests) = load("inputs/day05/test2.txt");
    let res = evaluate_print(&tests, &rules);
    assert_eq!(res, 5108);
}

#[test]
fn fix_print_works() {
    let (rules, tests) = load("inputs/day05/test1.txt");
    let res = fix_print(&tests, &rules);
    assert_eq!(res, 123);
}

#[test]
fn fix_print_input() {
    //So slow, could use some improvements lol
    let (rules, tests) = load("inputs/day05/test2.txt");
    let res = fix_print(&tests, &rules);
    assert_eq!(res, 7380);
}
