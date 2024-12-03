extern crate regex;
use regex::Regex;

pub fn mull_it(value: &str) -> usize {
    let regex = Regex::new(r"(?m)mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    regex
        .captures_iter(value)
        .map(|capture| {
            let n1 = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let n2 = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();

            n1 * n2
        })
        .sum()
}
