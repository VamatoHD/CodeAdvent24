pub fn load(path: &str) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    let file_content = std::fs::read_to_string(path).unwrap();

    for line in file_content.lines() {
        let a = line.chars().position(|x| x == ':').unwrap();

        let goal = line[0..a].parse::<usize>().unwrap();
        let mut rest = line[a + 1..line.len()]
            .split(' ')
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.parse::<usize>().unwrap())
                }
            })
            .collect::<Vec<usize>>();

        rest.push(goal);
        res.push(rest);
    }

    res
}

fn process(x: usize, y: usize, b: usize) -> usize {
    match b {
        0 => x + y,
        2 => {
            let power = (y as f64).log(10.0).floor() as u32 + 1;
            x * 10_usize.pow(power) + y
        }
        1 | _ => x * y,
    }
}

fn generate_case(poss: usize, length: usize) -> Vec<Vec<usize>> {
    if length == 0 {
        return vec![vec![]];
    }
    let mut res = Vec::with_capacity(poss.pow(length as u32));
    let smaller = generate_case(poss, length - 1);

    for i in 0..poss {
        for mut j in smaller.iter().cloned() {
            j.push(i);
            res.push(j)
        }
    }

    res
}

fn is_valid(test: &[usize], goal: usize, n_cases: usize) -> bool {
    let cases = generate_case(n_cases, test.len() - 1);

    cases.into_iter().any(|case| {
        let mut res = test[0];
        for (i, &c) in case.iter().enumerate() {
            res = process(res, test[i + 1], c)
        }
        res == goal
    })
}

pub fn repair_brige(tests: &Vec<Vec<usize>>, cases: usize) -> usize {
    tests
        .iter()
        .map(|test| {
            let &goal = test.last().unwrap();
            if is_valid(&test[0..test.len() - 1], goal, cases) {
                goal
            } else {
                0
            }
        })
        .sum()
}
