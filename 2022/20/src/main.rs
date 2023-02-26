#![allow(dead_code, unused_variables)]

use std::collections::VecDeque;

mod utils;

fn main() {
    let filename = include_str!("../sample.txt");
    // let filename = include_str!("../input.txt");

    part_1(filename);
    // part_2(filename);
}

#[derive(Clone)]
struct Thing {
    index: usize,
    value: i64,
}

fn part_1(contents: &str) {
    let mut numbers = parse_file(contents);

    mix(&mut numbers);

    let position = numbers.iter().position(|thing| thing.value == 0).unwrap();
    dbg!(position);
    let answer = numbers[(position + 1000) % numbers.len()].value
        + numbers[(position + 2000) % numbers.len()].value
        + numbers[(position + 3000) % numbers.len()].value;
    dbg!(answer);
}

fn part_2(contents: &str) {
    let key = 811589153;
    let numbers = parse_file(contents);
    let mut numbers = numbers
        .iter()
        .map(|thing| Thing {
            index: thing.index,
            value: (thing.value * key) as i64,
        })
        .collect::<VecDeque<Thing>>();

    for _ in 0..10 {
        mix(&mut numbers);
    }

    let position = numbers.iter().position(|thing| thing.value == 0).unwrap();
    dbg!(position);
    // [(position + 1000) % numbers.len()].value
    let answer = numbers[(position + 1000) % numbers.len()].value
        + numbers[(position + 2000) % numbers.len()].value
        + numbers[(position + 3000) % numbers.len()].value;
    dbg!(answer);
}

fn mix(numbers: &mut VecDeque<Thing>) {
    for orig_index in 0..(numbers.len()) {
        let mut current_index = numbers
            .iter()
            .position(|thing| thing.index == orig_index)
            .unwrap() as i64;

        let mut magnitude = numbers[current_index as usize].value;

        let delta = if magnitude > 0 { 1 } else { -1 };
        loop {
            if magnitude == 0 {
                break;
            }

            let mut new_index = (current_index as i64) + delta;
            if new_index < 0 {
                new_index = (numbers.len() - 2) as i64;

                numbers.insert(
                    (new_index + 1) as usize,
                    numbers[current_index as usize].clone(),
                );
                numbers.remove(current_index as usize).unwrap();
            } else if delta < 0 && new_index == 0 {
                new_index = (numbers.len() - 1) as i64;

                numbers.insert(
                    (new_index + 1) as usize,
                    numbers[current_index as usize].clone(),
                );
                numbers.remove(current_index as usize).unwrap();
            } else if new_index >= numbers.len() as i64 {
                new_index = 1;
                let value = numbers.remove(current_index as usize).unwrap();
                numbers.insert(new_index as usize, value);
            } else if delta > 0 && new_index == (numbers.len() - 1) as i64 {
                new_index = 0;
                let value = numbers.remove(current_index as usize).unwrap();
                numbers.insert(new_index as usize, value);
            } else {
                // performance special case: can swap if right next to each other
                numbers.swap(current_index as usize, new_index as usize);
            }
            current_index = new_index;
            magnitude -= delta;
        }
    }
}

fn parse_file(contents: &str) -> VecDeque<Thing> {
    let mut rv = VecDeque::new();
    for (i, line) in contents.lines().enumerate() {
        rv.push_back(Thing {
            value: line.parse::<i64>().unwrap(),
            index: i,
        });
    }
    return rv;
}

fn print_numbers<'a>(numbers: impl Iterator<Item = &'a Thing>) {
    for number in numbers {
        print!("{:3} ", number.value);
    }
    println!();
}
