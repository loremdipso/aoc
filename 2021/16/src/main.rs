#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
mod utils;

/*
0 = 0000
1 = 0001
2 = 0010
3 = 0011
4 = 0100
5 = 0101
6 = 0110
7 = 0111
8 = 1000
9 = 1001
A = 1010
B = 1011
C = 1100
D = 1101
E = 1110
F = 1111
*/

fn main() {
    let filename = "sample.txt";
    // let filename = "input.txt";

    part_1(filename);
    // part_2(filename);
}

fn part_1(filename: &str) {
    let line = &utils::get_lines::<String>(filename)[0];
    let mut line = &binarilize_line(line)[..];

    let version = next_number(&mut line, 3);
    let mut total = 0;

    let packet_type = next_number(&mut line, 3);
    match packet_type {
        4 => {
            let byte = get_number(&mut line);
            dbg!(byte);
        }
        _ => {
            let operator = get_operator(&mut line);
            // uggg who designed this problem?
        }
    };

    // dbg!(packet_type);
    // dbg!(&line);
}

fn get_number(mut str: &mut &[char]) -> i32 {
    let mut chars = Vec::new();
    let byte_size = 5;
    loop {
        let should_continue = match pop(&mut str) {
            '0' => false,
            '1' => true,
            _ => panic!(),
        };

        chars.extend(pop_n_chars(str, 4));
        if !should_continue {
            break;
        }
    }

    return parse_binary_number(&chars);
}

fn pop(str: &mut &[char]) -> char {
    let temp = &str[0];
    *str = &str[1..];
    return *temp;
}

fn pop_n_chars(str: &mut &[char], to_consume: usize) -> Vec<char> {
    let temp = &str[0..to_consume];
    *str = &str[(to_consume)..];
    return Vec::from(temp);
}

fn next_number(str: &mut &[char], to_consume: usize) -> i32 {
    let rv = parse_binary_number(&str[0..to_consume]);
    *str = &str[(to_consume)..];
    return rv;
}

fn parse_binary_number(temp: &[char]) -> i32 {
    let mut rv = 0;
    for (index, char) in temp.iter().rev().enumerate() {
        let value = if *char == '0' { 0 } else { 1 };
        let value = value * (2_i32.pow(index.try_into().unwrap()));
        if index == 0 {
            rv = value;
        } else {
            rv += value;
        }
    }
    return rv;
}

fn binarilize_line(line: &str) -> Vec<char> {
    let mapping: HashMap<char, Vec<char>> = HashMap::from([
        ('0', Vec::from(['0', '0', '0', '0'])),
        ('1', Vec::from(['0', '0', '0', '1'])),
        ('2', Vec::from(['0', '0', '1', '0'])),
        ('3', Vec::from(['0', '0', '1', '1'])),
        ('4', Vec::from(['0', '1', '0', '0'])),
        ('5', Vec::from(['0', '1', '0', '1'])),
        ('6', Vec::from(['0', '1', '1', '0'])),
        ('7', Vec::from(['0', '1', '1', '1'])),
        ('8', Vec::from(['1', '0', '0', '0'])),
        ('9', Vec::from(['1', '0', '0', '1'])),
        ('A', Vec::from(['1', '0', '1', '0'])),
        ('B', Vec::from(['1', '0', '1', '1'])),
        ('C', Vec::from(['1', '1', '0', '0'])),
        ('D', Vec::from(['1', '1', '0', '1'])),
        ('E', Vec::from(['1', '1', '1', '0'])),
        ('F', Vec::from(['1', '1', '1', '1'])),
    ]);

    return line
        .chars()
        .flat_map(|c| mapping.get(&c).unwrap().clone())
        .collect::<Vec<char>>();
}
