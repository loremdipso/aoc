#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    let filename = "sample.txt";
    // let filename = "input.txt";

    part_1(filename);
    // part_2(filename);
}

fn part_1(filename: &str) {
    let lines = utils::get_generic(filename, get_line);
    dbg!(&lines);
}

fn get_line(line: &str) -> (char, usize) {
    let (a, b) = line.split_once(" = ").unwrap();
    let a = a.chars().nth(0).unwrap();
    let b = b.parse::<usize>().unwrap();
    return (a, b);
}
