use codeadvent24::days::day03::*;
use codeadvent24::load::load_file;

#[test]
fn mull_it_works() {
    let test = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(mull_it(test), 161);
}

#[test]
fn mull_it_input() {
    let test = load_file("inputs/day03.txt");
    assert_eq!(mull_it(test.as_str()), 188192787);
}
