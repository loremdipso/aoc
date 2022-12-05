#![allow(dead_code, unused_variables)]

use std::{collections::HashSet, vec};
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let lines = utils::get_lines::<String>(filename);
    let mut total = 0;
    for line in &lines {
        let (first, last) = line.split_at(line.len() / 2);

        let mut first_map = HashSet::new();
        for char in first.chars() {
            first_map.insert(char);
        }

        for char in last.chars() {
            if first_map.contains(&char) {
                total += value(char);
                break;
            }
        }
    }

    dbg!(&lines);
    dbg!(total);
}

fn value(char: char) -> u32 {
    // dbg!(&char);
    let mut char_value = char as u32;
    if char.is_lowercase() {
        char_value += 1;
        char_value -= 'a' as u32;
    } else {
        char_value += 27;
        char_value -= 'A' as u32;
    }
    // dbg!(&char_value);
    return char_value;
}

fn part_2(filename: &str) {
    let lines = utils::get_lines::<String>(filename);
    let mut total = 0;
    for chunk in lines.chunks(3) {
        // dbg!(chunk);
        // let (first, last) = line.split_at(line.len() / 2);
        let mut sets = Vec::new();
        for line in chunk {
            let mut temp_set = HashSet::new();
            for char in line.chars() {
                temp_set.insert(char);
            }
            sets.push(temp_set)
        }

        for char in chunk[0].chars() {
            let mut valid = true;
            for set in &sets {
                if !set.contains(&char) {
                    valid = false;
                    break;
                }
            }

            if valid {
                dbg!(&char);
                total += value(char);
                break;
            }
        }
    }

    dbg!(total);
}
