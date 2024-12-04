pub mod days;

pub mod load {
    use std::fmt::Debug;
    use std::fs;
    use std::str::FromStr;

    pub fn load_file(path: &str) -> String {
        fs::read_to_string(path).unwrap()
    }

    pub fn dual_column<T>(path: &str) -> (Vec<T>, Vec<T>)
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let file_content = fs::read_to_string(path).unwrap();

        let mut column1 = Vec::new();
        let mut column2 = Vec::new();

        for line in file_content.lines() {
            let mut parts = line.split_ascii_whitespace();

            if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
                column1.push(a.parse::<T>().unwrap());
                column2.push(b.parse::<T>().unwrap());
            }
        }

        (column1, column2)
    }

    pub fn rows<T>(path: &str) -> Vec<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let file_content = fs::read_to_string(path).unwrap();
        let mut res = Vec::new();

        for line in file_content.lines() {
            let mut temp = Vec::new();

            for part in line.split_ascii_whitespace() {
                let value = part.parse::<T>().unwrap();
                temp.push(value);
            }
            res.push(temp);
        }

        res
    }

    pub fn rows_to_chars(path:&str) -> Vec<Vec<char>> {
        let file_content = fs::read_to_string(path).unwrap();
        let mut res = Vec::new();

        for line in file_content.lines() {
            let value : Vec<char> = line.chars().collect();
            res.push(value);
        }

        res
    }
}
