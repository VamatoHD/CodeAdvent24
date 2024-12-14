pub fn load(path: &str) -> Vec<Robot> {
    let file_content = std::fs::read_to_string(path).unwrap();

    file_content
        .lines()
        .into_iter()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let p1 = parts[0].parse::<isize>().unwrap();
            let p2 = parts[1].parse::<isize>().unwrap();
            let v1 = parts[2].parse::<isize>().unwrap();
            let v2 = parts[3].parse::<isize>().unwrap();

            Robot::new(p1, p2, v1, v2)
        })
        .collect()
}

#[derive(Debug)]
pub struct Robot {
    pub x: isize,
    pub y: isize,
    pub vx: isize,
    pub vy: isize,
}

impl Robot {
    fn new(x: isize, y: isize, vx: isize, vy: isize) -> Self {
        Robot { x, y, vx, vy }
    }
}

fn step(p: isize, v: isize, w: isize, steps: isize) -> isize {
    return (p + v * steps).rem_euclid(w);
}

pub fn predict_robot(robots: &Vec<Robot>, s: (isize, isize), steps: isize) -> usize {
    let mw = s.0 / 2;
    let mh = s.1 / 2;

    let res = move_robots(robots, s, steps);

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for (x, y) in res {
        if x < mw && y < mh {
            q1 += 1
        } else if x > mw && y < mh {
            q2 += 1
        } else if x < mw && y > mh {
            q3 += 1
        } else if x > mw && y > mh {
            q4 += 1
        }
    }

    q1 * q2 * q3 * q4
}

pub fn move_robots(robots: &Vec<Robot>, s: (isize, isize), steps: isize) -> Vec<(isize, isize)> {
    robots
        .iter()
        .map(|r| {
            let x = step(r.x, r.vx, s.0, steps);
            let y = step(r.y, r.vy, s.1, steps);
            (x, y)
        })
        .collect::<Vec<(isize, isize)>>()
}
