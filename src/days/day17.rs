pub fn load(path: &str) -> (Vec<usize>, Vec<(usize, usize)>) {
    let file_content = std::fs::read_to_string(path).unwrap();

    let register = file_content
        .lines()
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let code_values = file_content
        .lines()
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let code = code_values.chunks_exact(2).map(|c| (c[0], c[1])).collect();
    (register, code)
}

pub fn compute(reg: &Vec<usize>, code: &Vec<(usize, usize)>) -> String {
    let mut res: Vec<usize> = Vec::new();

    let mut reg_a = reg[0];
    let mut reg_b = reg[1];
    let mut reg_c = reg[2];

    let mut pointer = 0;

    loop {
        if pointer >= code.len() {
            break res
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }

        let (op, operand) = code[pointer];

        let combo = match operand {
            0..=3 => operand,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => 0,
        };

        match op {
            0 => reg_a = reg_a / (1 << combo),
            1 => reg_b = reg_b ^ operand,
            2 => reg_b = combo.rem_euclid(8) & 0b111,
            3 => {
                if reg_a != 0 {
                    pointer = operand / 2;
                    continue;
                }
            }
            4 => reg_b = reg_b ^ reg_c,
            5 => res.push(combo.rem_euclid(8)),
            6 => reg_b = reg_a / (1 << combo),
            7 => reg_c = reg_a / (1 << combo),
            _ => {}
        }

        pointer += 1;
    }
}
