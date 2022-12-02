#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    let filename = "sample.txt";
    // let filename = "input.txt";

    part_1(filename);
    // part_2(filename);
}

fn part_1(filename: &str) {
    let lines = utils::get_lines::<i64>(filename);
    dbg!(&lines);
}

fn part_2(filename: &str) {
    let lines = utils::get_lines::<i64>(filename);
    dbg!(&lines);
}
