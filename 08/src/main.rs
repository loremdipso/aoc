mod line;
use std::collections::HashMap;

use line::Line;
mod utils;
use utils::get_lines;

fn main() {
    // let lines = get_lines("sample.txt");
    let lines = get_lines("input.txt");
    // part_1(lines);
    part_2(lines);
}

fn part_1(lines: Vec<Line>) {
    let mut num_unique = 0;
    for line in lines {
        num_unique += Line::num_unique_output(&line.outputs);
    }
    dbg!(num_unique);
}

fn part_2(lines: Vec<Line>) {
    let mut answer = 0;
    for line in lines {
        let mut values = "".to_string();
        let mapping = get_mapping(&line.patterns);
        // dbg!(&mapping);
        for output in line.outputs {
            let value = decode(&output, &mapping);
            // dbg!(&value);
            values += &value;
        }
        dbg!(&values);
        let number = values.parse::<i64>().unwrap();
        answer += number;
    }
    dbg!(answer);
}

fn get_mapping(patterns: &Vec<String>) -> HashMap<String, String> {
    let mut char_mapping: HashMap<char, i64> = HashMap::new();
    for pattern in patterns {
        for char in pattern.chars() {
            *char_mapping.entry(char).or_insert(0) += 1;
        }
    }

    let mut mapping = HashMap::new();
    let mut reverse_mapping = HashMap::new();
    for pattern in patterns {
        match pattern.len() {
            2 => {
                mapping.insert(pattern.to_string(), "1".to_string());
                reverse_mapping.insert(1, pattern.to_string());
            }
            3 => {
                mapping.insert(pattern.to_string(), "7".to_string());
                reverse_mapping.insert(7, pattern.to_string());
            }
            4 => {
                mapping.insert(pattern.to_string(), "4".to_string());
                reverse_mapping.insert(4, pattern.to_string());
            }
            7 => {
                mapping.insert(pattern.to_string(), "8".to_string());
                reverse_mapping.insert(8, pattern.to_string());
            }
            _ => {}
        };
    }

    for pattern in patterns {
        match pattern.len() {
            6 => {
                // 3 of these
                if count_common_chars(pattern, reverse_mapping.get(&1).unwrap()) == 1 {
                    mapping.insert(pattern.to_string(), "6".to_string());
                    reverse_mapping.insert(6, pattern.to_string());
                } else if count_common_chars(pattern, reverse_mapping.get(&4).unwrap()) == 4 {
                    mapping.insert(pattern.to_string(), "9".to_string());
                    reverse_mapping.insert(9, pattern.to_string());
                } else {
                    mapping.insert(pattern.to_string(), "0".to_string());
                    reverse_mapping.insert(0, pattern.to_string());
                }
            }
            _ => {}
        };
    }

    for pattern in patterns {
        match pattern.len() {
            5 => {
                // 3 of these
                if count_common_chars(pattern, reverse_mapping.get(&1).unwrap()) == 2 {
                    mapping.insert(pattern.to_string(), "3".to_string());
                } else if count_common_chars(pattern, reverse_mapping.get(&9).unwrap()) == 5 {
                    mapping.insert(pattern.to_string(), "5".to_string());
                } else {
                    mapping.insert(pattern.to_string(), "2".to_string());
                }
            }
            _ => {}
        };
    }

    return mapping;
}

fn decode(output: &str, mapping: &HashMap<String, String>) -> String {
    return mapping.get(output).unwrap().to_string();
}

fn count_common_chars(pattern: &str, number: &str) -> usize {
    return number
        .chars()
        .into_iter()
        .filter(|c| pattern.contains(&c.to_string()))
        .count();
}
