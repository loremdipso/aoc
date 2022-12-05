#![allow(dead_code, unused_variables)]

use std::mem::swap;
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let mut count = 0;
    let lines = utils::get_lines::<String>(filename);
    for line in &lines {
        let (first, second) = line.split_once(",").unwrap();
        let first_range = get_range(first);
        let second_range = get_range(second);
        if (first_range.0 <= second_range.0 && first_range.1 >= second_range.1)
            || (second_range.0 <= first_range.0 && second_range.1 >= first_range.1)
        {
            count += 1;
        }
    }
    dbg!(&lines);
    dbg!(&count);
}

fn part_2(filename: &str) {
    let mut count = 0;
    let lines = utils::get_lines::<String>(filename);
    for line in &lines {
        let (first, second) = line.split_once(",").unwrap();
        let mut first_range = get_range(first);
        let mut second_range = get_range(second);

        // normalize
        if first_range.0 > second_range.0 {
            swap(&mut first_range, &mut second_range);
        }

        if first_range.1 >= second_range.0 {
            count += 1;
        }
    }

    dbg!(&lines);
    dbg!(&count);
}

fn get_range(range: &str) -> (usize, usize) {
    let (first, second) = range.split_once("-").unwrap();
    return (
        first.parse::<usize>().unwrap(),
        second.parse::<usize>().unwrap(),
    );
}
