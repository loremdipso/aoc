#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::VecDeque;

mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

#[derive(Clone, Debug)]
struct Thing {
    index: usize,
    value: isize,
}

fn part_1(contents: &str) {
    let mut numbers = parse_file(contents);

    mix(&mut numbers);

    // print_numbers(numbers.iter());

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
            value: (thing.value * key) as isize,
        })
        .collect::<VecDeque<Thing>>();

    for index in 0..10 {
        println!("mix iteration: {}", index + 1);
        mix(&mut numbers);
    }

    let position = numbers.iter().position(|thing| thing.value == 0).unwrap();
    dbg!(position);
    let answer = numbers[(position + 1000) % numbers.len()].value
        + numbers[(position + 2000) % numbers.len()].value
        + numbers[(position + 3000) % numbers.len()].value;
    dbg!(answer);
}

fn mix(numbers: &mut VecDeque<Thing>) {
    let count = numbers.len() as isize;
    for orig_index in 0..(numbers.len()) {
        let mut current_index = numbers
            .iter()
            .position(|thing| thing.index == orig_index)
            .unwrap() as isize;

        // NOTE: after struggling mightily with this solution I caved
        // and got this gem from https://github.com/schubart/AdventOfCode_2022_Rust/blob/master/day20/src/lib.rs
        let mut magnitude = numbers[current_index as usize].value;

        // Truncate. Use euclid because new_index might be negative.
        // Length minus 1 because of problem statement: Moving an element
        // by (n - 1) places in a list of length n leaves list unchanged.
        let new_index = (current_index + magnitude).rem_euclid(count - 1);

        let thing = numbers.remove(current_index as usize).unwrap();
        numbers.insert(new_index as usize, thing);
    }
}

fn parse_file(contents: &str) -> VecDeque<Thing> {
    let mut rv = VecDeque::new();
    for (i, line) in contents.lines().enumerate() {
        rv.push_back(Thing {
            value: line.parse::<isize>().unwrap(),
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
