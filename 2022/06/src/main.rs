#![allow(dead_code, unused_variables)]

use std::collections::{HashSet, VecDeque};
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    solution(filename, 4);
}

fn part_2(filename: &str) {
    solution(filename, 14);
}

fn solution(filename: &str, min_message_size: usize) {
    let lines = utils::get_lines::<String>(filename);
    let mut sequence = VecDeque::new();
    for (position, char) in lines[0].chars().enumerate() {
        sequence.push_back(char);
        if sequence.len() > min_message_size {
            sequence.pop_front();
        }
        if sequence.len() == min_message_size {
            // make sure all four characters are different
            let mut set = HashSet::new();
            for char in &sequence {
                set.insert(char);
            }
            if set.len() == min_message_size {
                println!("First position: {}", position + 1);
                return;
            }
        }
    }
}
