#![allow(dead_code, unused_variables)]
mod utils;
use utils::get_lines;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";
    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let lines = get_lines(filename);
    dbg!(&lines);
    dbg!(&lines.iter().max());
}

fn part_2(filename: &str) {
    let mut lines = get_lines(filename);
    lines.sort_by(|a, b| b.cmp(a));
    dbg!(&lines);
    dbg!(&lines[0..3]);
    dbg!(&lines[0..3].iter().sum::<i64>());
}
