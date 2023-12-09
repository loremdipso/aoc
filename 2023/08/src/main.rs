#![allow(dead_code, unused_variables)]
mod utils;
use prime_factorization::Factorization;
use std::collections::HashSet;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let (directions, nodes) = utils::get_data(contents);
    let mut position = "AAA".to_string();
    let mut count = 0;
    'outer_loop: loop {
        for char in directions.chars() {
            if position == "ZZZ" {
                break 'outer_loop;
            }
            count += 1;
            let node = nodes.get(&position).unwrap();
            match char {
                'L' => position = node.left.clone(),
                'R' => position = node.right.clone(),
                _ => panic!(),
            };
        }
    }

    dbg!(&directions, &nodes);
    dbg!(count);
}

fn part_2(contents: &str) {
    let (directions, nodes) = utils::get_data(contents);
    let positions = nodes
        .keys()
        .filter(|key| key.chars().rev().next().unwrap() == 'A')
        .map(|key| key.into())
        .collect::<Vec<String>>();

    // Fast - least common multiple?

    let mut multiples = Vec::new();
    for position in &positions {
        let mut seen = HashSet::new();
        let mut temp_multiples = Vec::new();
        let mut temp_count = 0;
        let mut position = position.clone();
        'outer_loop: loop {
            for (char_index, char) in directions.chars().enumerate() {
                // dbg!(&position);
                // End position
                if position.chars().rev().next().unwrap() == 'Z' {
                    if !seen.insert((position.clone(), char_index)) {
                        break 'outer_loop;
                    } else {
                        temp_multiples.push(temp_count);
                    }
                }
                temp_count += 1;
                let node = nodes.get(&position).unwrap();
                match char {
                    'L' => position = node.left.clone(),
                    'R' => position = node.right.clone(),
                    _ => panic!(),
                };
            }
        }
        println!();

        multiples.push(temp_multiples);
    }

    dbg!(&multiples);
    let mut min_factors: Vec<u128> = multiples
        .iter()
        .flat_map(|list| Factorization::run(*list.iter().min().unwrap()).factors)
        .collect();
    min_factors.sort();
    min_factors.dedup();

    let answer = min_factors.iter().fold(1, |acc, value| acc * value);
    dbg!(min_factors, answer);

    // dbg!(&directions, &nodes);
    // dbg!(count);
}
