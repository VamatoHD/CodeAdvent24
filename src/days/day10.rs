pub fn load(path: &str) -> Vec<Vec<u8>> {
    let file_content = std::fs::read_to_string(path).unwrap();
    //let res = Vec::new();

    file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn index(map: &Vec<Vec<u8>>, pos: (usize, usize)) -> u8 {
    map[pos.1][pos.0]
}

fn get_neigh(pos: (usize, usize), w: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    if pos.0 > 0 {
        res.push((pos.0 - 1, pos.1));
    }

    if pos.1 > 0 {
        res.push((pos.0, pos.1 - 1));
    }

    if pos.0 < w - 1 {
        res.push((pos.0 + 1, pos.1));
    }

    if pos.1 < w - 1 {
        res.push((pos.0, pos.1 + 1));
    }

    res
}

fn search(map: &Vec<Vec<u8>>, pos: (usize, usize)) -> usize {
    let mut to_visit = vec![pos];
    let mut visited = Vec::new();

    loop {
        if to_visit.len() == 0 {
            break visited.len();
        }

        let cur = match to_visit.pop() {
            Some(v) => v,
            None => break visited.len(),
        };
        let cur_v = index(map, cur);

        if cur_v >= 9 && !visited.contains(&cur) {
            visited.push(cur.clone());
            continue;
        }

        to_visit.extend(get_neigh(cur, map.len()).iter().filter(|pos| {
            let v = index(map, **pos);
            v == cur_v + 1
        }));
    }
}

pub fn get_trailheads(map: &Vec<Vec<u8>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(
                    |(x, &value)| {
                        if value == 0 {
                            search(map, (x, y))
                        } else {
                            0
                        }
                    },
                )
                .sum::<usize>()
        })
        .sum()
}

fn get_trails_recursive(map: &Vec<Vec<u8>>, pos: (usize, usize)) -> usize {
    let cur = index(map, pos);
    if cur >= 9 {
        return 1;
    }

    get_neigh(pos, map.len())
        .iter()
        .map(|p| {
            if index(map, *p) == cur + 1 {
                get_trails_recursive(map, *p)
            } else {
                0
            }
        })
        .sum()
}

pub fn get_trail_rating(map: &Vec<Vec<u8>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, &value)| {
                    if value == 0 {
                        get_trails_recursive(map, (x, y))
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}
