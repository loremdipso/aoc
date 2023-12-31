#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    let filename = include_str!("../sample.txt");
    //let filename = include_str!("../input.txt");

    part_1(filename);
    // part_2(filename);
}

fn part_1(contents: &str) {
    let mut lines = Vec::new();
    let mut starting_position: Position = (0, 0);
    for (line_index, line) in contents.lines().enumerate() {
        lines.push(parse_line(line));
        for (char_index, char) in line.chars().enumerate() {
            if char == 'S' {
                starting_position = (line_index, char_index);
            }
        }
    }
    dbg!(&lines);
}

type Position = (usize, usize);
fn parse_line(line: &str, line_index: usize) -> Vec<Vec<Position>> {
    let mut rv = Vec::new();
    for (index, char) in line.chars().enumerate() {
        match char {
            '.' => {}
            'S' => {}
            '|' => rv.push(vec![Direction::South, Direction::North]),
            '-' => rv.push(vec![Direction::East, Direction::West]),
            'L' => rv.push(vec![Direction::North, Direction::East]),
            'J' => rv.push(vec![Direction::North, Direction::West]),
            '7' => rv.push(vec![Direction::South, Direction::West]),
            'F' => rv.push(vec![Direction::South, Direction::East]),
            _ => panic!(),
        }
    }
    return rv;
}
