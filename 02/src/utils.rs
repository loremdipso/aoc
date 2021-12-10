use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::FORWARD),
            "down" => Ok(Direction::DOWN),
            "up" => Ok(Direction::UP),
            _ => Err(()),
        }
    }
}

pub fn get_lines() -> Vec<(Direction, i64)> {
    let mut rv = vec![];
    if let Ok(lines) = read_lines(&env::args().skip(1).collect::<Vec<String>>()[0]) {
        for line in lines {
            if let Ok(line) = line {
                if line.len() > 0 {
                    let mut pieces = line.split(" ");
                    let direction = pieces.next().unwrap().parse::<Direction>().unwrap();
                    let count = pieces.next().unwrap().parse::<i64>().unwrap();
                    rv.push((direction, count));
                }
            }
        }
    }

    return rv;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
