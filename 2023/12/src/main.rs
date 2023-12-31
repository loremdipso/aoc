#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    let filename = include_str!("../sample.txt");
    //let filename = include_str!("../input.txt");

    part_1(filename);
    // part_2(filename);
}

fn part_1(contents: &str) {
    let lines = utils::get_generic(contents, parse_line);
    print_lines(&lines);
}

type Line = (Vec<char>, Vec<usize>);
fn parse_line(line: &str) -> Line {
    let (first, second) = line.split_once(' ').unwrap();
    return (
        first.chars().collect(),
        second
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect(),
    );
}

fn print_lines(lines: &Vec<Line>) {
    lines.iter().for_each(print_line);
}

fn print_line(line: &Line) {
    line.0.iter().for_each(|c| print!("{c}"));
    println!();
    line.1.iter().for_each(|c| print!("{c}"));
    println!();
    println!();
}
