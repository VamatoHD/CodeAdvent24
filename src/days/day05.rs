pub fn load(path: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let file_content = std::fs::read_to_string(path).unwrap();

    let mut rules = Vec::new();
    let mut tests = Vec::new();

    for line in file_content.lines() {
        if line.is_empty() {
            continue;
        };

        let c = if line.len() <= 5 { '|' } else { ',' };
        let temp: Vec<usize> = line.split(c).map(|x| x.parse::<usize>().unwrap()).collect();
        if line.len() <= 5 {
            rules.push(temp)
        } else {
            tests.push(temp);
        }
    }

    (rules, tests)
}

fn is_queue_valid(queue: &Vec<usize>, rules: &Vec<Vec<usize>>) -> bool {
    queue.iter().enumerate().all(|(i, &x)| {
        for rule in rules {
            //If rule doenst apply, skip
            if rule[0] != x {
                continue;
            }

            //If rule applies and is broken, it isnt valid
            if queue[0..i].contains(&rule[1]) {
                return false;
            }
        }

        true
    })
}

pub fn evaluate_print(print: &Vec<Vec<usize>>, rules: &Vec<Vec<usize>>) -> usize {
    print
        .iter()
        .map(|x| {
            if is_queue_valid(x, &rules) {
                x[x.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn fix_queue(queue: &Vec<usize>, rules: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut res = queue.clone();
    //Lets brute force
    for _ in 1..10 {
        for i in 0..queue.len() {
            for rule in rules {
                //If rule doenst apply, skip
                if rule[0] != res[i] {
                    continue;
                }

                //If rule applies and is broken, it isnt valid
                if let Some(index) = res[0..i].iter().position(|x| x == &rule[1]) {
                    let temp = res[i];
                    res[i] = res[index];
                    res[index] = temp;
                    break;
                }
            }
        }
    }

    res
}

pub fn fix_print(print: &Vec<Vec<usize>>, rules: &Vec<Vec<usize>>) -> usize {
    print
        .iter()
        .filter_map(|x| {
            if is_queue_valid(x, rules) {
                return None;
            }

            let valid = fix_queue(x, rules);
            Some(valid[valid.len() / 2])
        })
        .sum()
}
