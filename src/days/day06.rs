use std::collections::HashSet;

pub fn load(path: &str) -> (Vec<bool>, Guard) {
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut map = Vec::new();
    let mut guard = Guard::new(0, '^');

    let grid_content: Vec<char> = file_content.lines().flat_map(|line| line.chars()).collect();

    for (i, c) in grid_content.iter().enumerate() {
        match c {
            '#' => map.push(true),
            '^' | '>' | '<' | 'v' => {
                guard = Guard::new(i, *c);
                map.push(false);
            }
            '\n' => {}
            _ => map.push(false),
        }
    }

    (map, guard)
}

#[derive(Clone, Copy)]
pub struct Guard {
    pub pos: usize,
    pub start: usize,
    // 0 = up
    // 1 = right
    // ...
    pub state: u8,
}
impl Guard {
    pub fn new(pos: usize, c: char) -> Guard {
        Guard {
            pos,
            start: pos,
            state: match c {
                '^' => 0,
                '>' => 1,
                'v' => 2,
                '<' => 3,
                _ => 0,
            },
        }
    }

    pub fn next(&self, len: usize) -> Option<usize> {
        match self.state {
            1 => {
                if (self.pos + 1) % len == 0 || self.pos + 1 >= len * len {
                    None
                } else {
                    Some(self.pos + 1)
                }
            }
            2 => {
                let n = self.pos + len;

                if n > len * len {
                    None
                } else {
                    Some(n)
                }
            }
            3 => {
                if self.pos % len == 0 || self.pos == 0 {
                    None
                } else {
                    Some(self.pos - 1)
                }
            }
            0 | _ => {
                if self.pos < len {
                    None
                } else {
                    Some(self.pos - len)
                }
            }
        }
    }
}

pub fn move_guard(map: &Vec<bool>, guard: Guard) -> usize {
    guard_path(map, guard)
        .0
        .iter()
        .filter_map(|&b| if b { Some(1) } else { None })
        .sum()
}

///retuns the path and if loops
fn guard_path(map: &Vec<bool>, mut guard: Guard) -> (Vec<bool>, bool) {
    let l = (map.len() as f64).sqrt() as usize;
    let mut visited = vec![false; map.len()];
    let mut hit_pos: HashSet<(usize, u8)> = HashSet::new();

    loop {
        let next = guard.next(l);

        if let Some(pos) = next {
            if map[pos] {
                if hit_pos.contains(&(guard.pos, guard.state)) {
                    break (visited, true);
                } else {
                    hit_pos.insert((guard.pos, guard.state));
                    guard.state = (guard.state + 1) % 4
                }
            } else {
                guard.pos = pos;
                if !visited[pos] {
                    visited[pos] = true;
                }
            }
        } else {
            break (visited, false);
        }
    }
}

pub fn get_boundaries(map: &Vec<bool>, guard: Guard) -> usize {
    let mut res = 0;
    let mut map_new = map.clone();
    for (i, &b) in guard_path(map, guard.clone()).0.iter().enumerate() {
        if !b || i == guard.start || map_new[i] == true {
            continue;
        }

        //What if we placed a barrier here?
        map_new[i] = true;
        if guard_path(&map_new, guard.clone()).1 {
            res += 1
        }
        map_new[i] = false;
    }

    res
}
