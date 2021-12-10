use crate::line::Line;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_lines(filename: &str) -> Vec<Line> {
    let mut rv = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.len() > 0 {
                    let mut pieces = line.split(" | ");
                    let patterns = parse_line(pieces.next().unwrap());
                    let outputs = parse_line(pieces.next().unwrap());
                    rv.push(Line { patterns, outputs });
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

fn parse_line(line: &str) -> Vec<String> {
    return line
        .split(" ")
        .filter(|p| p.len() > 0)
        .map(|p| sort_chars(p))
        .collect::<Vec<String>>();
}

fn sort_chars(string: &str) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    return String::from_iter(chars);
}
