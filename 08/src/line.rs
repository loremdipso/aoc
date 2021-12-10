#[derive(Debug)]
pub struct Line {
    pub patterns: Vec<String>,
    pub outputs: Vec<String>,
}

impl Line {
    pub fn num_unique_output(data: &Vec<String>) -> i64 {
        let mut count = 0;
        for datum in data {
            count += match datum.len() {
                2 => 1,
                3 => 1,
                4 => 1,
                7 => 1,
                _ => 0,
            };
        }
        return count;
    }
}
