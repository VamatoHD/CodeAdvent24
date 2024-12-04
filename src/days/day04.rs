fn is_xmas(c1: char, c2: char, c3: char, c4: char) -> bool {
    (c1 == 'X' && c2 == 'M' && c3 == 'A' && c4 == 'S')
        || (c1 == 'S' && c2 == 'A' && c3 == 'M' && c4 == 'X')
}

pub fn xmas_search(list: &Vec<Vec<char>>) -> usize {
    let l = list.len();
    let mut res = 0;

    for x in 0..l {
        for y in 0..l - 3 {
            if is_xmas(list[x][y], list[x][y + 1], list[x][y + 2], list[x][y + 3]) {
                res += 1;
            }
        }
    }

    for x in 0..l - 3 {
        for y in 0..l {
            if is_xmas(list[x][y], list[x + 1][y], list[x + 2][y], list[x + 3][y]) {
                res += 1;
            }
        }
    }

    for x in 0..l - 3 {
        for y in 0..l - 3 {
            if is_xmas(
                list[x][y],
                list[x + 1][y + 1],
                list[x + 2][y + 2],
                list[x + 3][y + 3],
            ) {
                res += 1;
            }

            if is_xmas(
                list[x + 3][y + 0],
                list[x + 2][y + 1],
                list[x + 1][y + 2],
                list[x + 0][y + 3],
            ) {
                res += 1;
            }
        }
    }

    res
}

pub fn is_x_mas(list: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let v1 = list[x + 1][y + 1] == 'A';
    let v2 = list[x][y] == 'M' && list[x + 2][y + 2] == 'S';
    let v3 = list[x][y] == 'S' && list[x + 2][y + 2] == 'M';
    let v4 = list[x + 2][y] == 'M' && list[x][y + 2] == 'S';
    let v5 = list[x + 2][y] == 'S' && list[x][y + 2] == 'M';

    v1 && (v2 || v3) && (v4 || v5)
}

pub fn x_mas_search(list: &Vec<Vec<char>>) -> usize {
    let l = list.len();
    let mut res = 0;

    for x in 0..l - 2 {
        for y in 0..l - 2 {
            if is_x_mas(list, x, y) {
                res += 1
            }
        }
    }

    res
}
