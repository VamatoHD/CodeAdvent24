use std::collections::HashMap;

pub fn load(path: &str) -> Vec<u64> {
    let file_content = std::fs::read_to_string(path).unwrap();

    file_content
        .lines()
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn digits(mut n: u64) -> u64 {
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count.max(1)
}

pub fn blink_count(stones: &Vec<u64>, steps: u64) -> u64 {
    let mut stone_counts: HashMap<u64, u64> = HashMap::new();

    for &stone in stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut new_count: HashMap<u64, u64> = HashMap::new();

        for (stone, count) in stone_counts {
            let d = digits(stone);

            let new_stones = {
                if stone == 0 {
                    vec![1]
                } else if d % 2 == 0 {
                    let pow = 10u64.pow(d as u32 / 2);
                    let first = stone / pow;
                    let second = stone - (first * pow);

                    vec![first, second]
                } else {
                    vec![stone * 2024]
                }
            };

            for new_stone in new_stones {
                *new_count.entry(new_stone).or_insert(0) += count;
            }
        }

        stone_counts = new_count;
    }

    stone_counts.values().sum()
}
