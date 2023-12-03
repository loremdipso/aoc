#![allow(dead_code, unused_variables)]

mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let lines = utils::get_lines::<String>(contents);
    let mut total = 0;
    for line in &lines {
        let mut first = u32::MAX;
        let mut last = u32::MAX;
        for char in line.chars() {
            if char.is_numeric() {
                if first == u32::MAX {
                    first = char.to_digit(10).unwrap();
                } else {
                    last = char.to_digit(10).unwrap();
                }
            }
        }

        if last == u32::MAX {
            last = first;
        }
        let temp = first * 10 + last;
        total += temp;
    }

    dbg!(total);
}

fn part_2(contents: &str) {
    let lines = utils::get_lines::<String>(contents);
    let numbers: Vec<(u32, &str)> = vec![
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];

    let mut total = 0;
    for line in &lines {
        let mut first = u32::MAX;
        let mut last = u32::MAX;
        let mut temp_line = String::new();
        for char in line.chars() {
            if char.is_numeric() {
                temp_line.clear();
                if first == u32::MAX {
                    first = char.to_digit(10).unwrap();
                } else {
                    last = char.to_digit(10).unwrap();
                }
            } else {
                temp_line.push(char);

                // Look for numbers
                for (value, string) in &numbers {
                    if temp_line.contains(string) {
                        // The end of one number can lead into the next, so
                        // let's not totally clear the line here
                        while temp_line.len() > string.len() {
                            temp_line.remove(0);
                        }
                        if first == u32::MAX {
                            first = *value;
                        } else {
                            last = *value;
                        }
                    }
                }
            }
        }

        if last == u32::MAX {
            last = first;
        }
        let temp = first * 10 + last;
        total += temp;
    }

    dbg!(total);
}
