mod utils;
use core::num;

use utils::get_lines;

fn main() {
    let lines = get_lines::<String>();
    // part_1(lines);
    part_2(lines);
}

fn part_1(lines: Vec<String>) {
    let count = lines[0].len();
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    for i in 0..count {
        let mut num_zeros = 0;
        let mut num_ones = 0;
        for line in &lines {
            if line.chars().nth(i).unwrap() == '0' {
                num_zeros += 1;
            } else {
                num_ones += 1;
            }
        }
        if num_zeros > num_ones {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }

    // NOTE: could bitwise not these, probably, but I'm lazy and dumb
    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    dbg!(gamma);
    dbg!(epsilon);
    dbg!(epsilon * gamma);
}

fn part_2(lines: Vec<String>) {
    let count = lines[0].len();
    let oxygen: isize;
    let cotwo: isize;

    // oxygen
    {
        let mut lines = lines.clone();
        let mut position = 0;
        while position < count && lines.len() > 1 {
            dbg!(&lines);
            if let Some(bit) = most_common_bit(&lines, position) {
                lines.retain(|line| getchar(line, position) == bit);
            } else {
                lines.retain(|line| getchar(line, position) == '1');
            }

            position += 1;
        }
        oxygen = isize::from_str_radix(&lines[0], 2).unwrap();

        dbg!(&lines);
        dbg!(oxygen);
    }

    // co2
    {
        let mut lines = lines.clone();
        let mut position = 0;
        while position < count && lines.len() > 1 {
            dbg!(&lines);
            if let Some(bit) = most_common_bit(&lines, position) {
                lines.retain(|line| getchar(line, position) != bit);
            } else {
                lines.retain(|line| getchar(line, position) == '0');
            }

            position += 1;
        }
        cotwo = isize::from_str_radix(&lines[0], 2).unwrap();

        dbg!(&lines);
        dbg!(cotwo);
    }

    dbg!(oxygen * cotwo);
}

fn most_common_bit(lines: &Vec<String>, i: usize) -> Option<char> {
    let mut num_zeros = 0;
    let mut num_ones = 0;
    for line in lines {
        if getchar(line, i) == '0' {
            num_zeros += 1;
        } else {
            num_ones += 1;
        }
    }
    if num_zeros == num_ones {
        return None;
    }
    return Some(if num_zeros > num_ones { '0' } else { '1' });
}

fn getchar(line: &String, i: usize) -> char {
    return line.chars().nth(i).unwrap();
}
