pub mod days;

pub mod load {
    use std::fmt::Debug;
    use std::fs;
    use std::str::FromStr;

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
}
