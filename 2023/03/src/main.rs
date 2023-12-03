#![allow(dead_code, unused_variables)]
mod utils;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let lines = utils::get_lines::<String>(contents);
    let mut total = 0;
    for (line_index, line) in lines.iter().enumerate() {
        let mut temp_number = String::new();
        let num_chars = line.len() - 1;
        for (char_index, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                temp_number.push(char);
            }

            if (!char.is_digit(10) || char_index == num_chars) && temp_number.len() > 0 {
                let number = temp_number.parse::<u32>().unwrap();
                if (char_index - temp_number.len()..char_index)
                    .any(|temp_char_index| is_part_number(&lines, line_index, temp_char_index))
                {
                    println!("{}", &temp_number);
                    total += number;
                }
                temp_number.clear();
            }
        }
    }

    // dbg!(&lines);
    dbg!(total);
}

fn is_part_number(lines: &Vec<String>, line_index: usize, char_index: usize) -> bool {
    let num_chars = lines[0].len();
    for line_index in line_index.checked_sub(1).unwrap_or(0)..=min(line_index + 1, lines.len() - 1)
    {
        for char_index in
            char_index.checked_sub(1).unwrap_or(0)..=min(char_index + 1, num_chars - 1)
        {
            let char = lines[line_index].chars().skip(char_index).next().unwrap();
            if !char.is_digit(10) && char != '.' {
                return true;
            }
        }
    }
    return false;
}

fn part_2(contents: &str) {
    let lines = utils::get_lines::<String>(contents);
    let mut ratios = HashMap::<(usize, usize), Vec<u32>>::new();
    for (line_index, line) in lines.iter().enumerate() {
        let mut temp_number = String::new();
        let num_chars = line.len() - 1;
        for (char_index, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                temp_number.push(char);
            }

            if (!char.is_digit(10) || char_index == num_chars) && temp_number.len() > 0 {
                let number = temp_number.parse::<u32>().unwrap();
                let mut locations = HashSet::new();
                for temp_char_index in char_index - temp_number.len()..char_index {
                    for gear_location in
                        adjacent_gear_locations(&lines, line_index, temp_char_index)
                    {
                        locations.insert(gear_location);
                    }
                }
                for location in locations {
                    ratios.entry(location).or_default().push(number);
                }
                temp_number.clear();
            }
        }
    }

    let answer = ratios.iter().fold(
        0,
        |acc, (_, e)| if e.len() == 2 { acc + e[0] * e[1] } else { acc },
    );
    // dbg!(&lines);
    dbg!(answer);
}

fn adjacent_gear_locations(
    lines: &Vec<String>,
    line_index: usize,
    char_index: usize,
) -> Vec<(usize, usize)> {
    let mut rv = vec![];
    let num_chars = lines[0].len();
    for line_index in line_index.checked_sub(1).unwrap_or(0)..=min(line_index + 1, lines.len() - 1)
    {
        for char_index in
            char_index.checked_sub(1).unwrap_or(0)..=min(char_index + 1, num_chars - 1)
        {
            let char = lines[line_index].chars().skip(char_index).next().unwrap();
            if char == '*' {
                rv.push((line_index, char_index));
            }
        }
    }
    return rv;
}
