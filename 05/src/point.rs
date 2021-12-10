#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(line: &str) -> Point {
        let mut pieces = line.split(",");
        let x = pieces.next().unwrap().parse::<i64>().unwrap();
        let y = pieces.next().unwrap().parse::<i64>().unwrap();
        return Point { x, y };
    }
}
