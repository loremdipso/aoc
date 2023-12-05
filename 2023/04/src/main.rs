#![allow(dead_code, unused_variables)]

use std::collections::{HashMap, HashSet};
mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let cards = utils::get_generic(contents, parse_card);

    let mut total = 0;
    for card in &cards {
        let mut temp = 0;
        for yours in &card.yours {
            if card.winning.contains(&yours) {
                if temp == 0 {
                    temp = 1;
                } else {
                    temp *= 2;
                }
            }
        }
        println!();
        total += temp;
    }

    // dbg!(&cards);
    dbg!(&total);
}

fn part_2(contents: &str) {
    let cards = utils::get_generic(contents, parse_card);
    let mut mapping = HashMap::<usize, usize>::new();

    // Get values per card
    for (index, card) in cards.iter().enumerate() {
        let mut num_matching = 0;
        for yours in &card.yours {
            if card.winning.contains(&yours) {
                num_matching += 1;
            }
        }

        // how many of THIS card do we have
        let self_value = *mapping.entry(index).or_insert(1);

        for new_index in index + 1..=index + num_matching {
            mapping
                .entry(new_index)
                .and_modify(|entry| *entry += self_value)
                .or_insert(1 + self_value);
        }
        // print_mapping(&mapping);
    }

    let total = mapping.values().fold(0, |a, b| a + b);
    // dbg!(&cards);
    dbg!(&total);
}

#[derive(Debug)]
struct Card {
    winning: HashSet<usize>,
    yours: Vec<usize>,
}

fn parse_card(line: &str) -> Card {
    let mut winning = HashSet::new();
    let mut yours = Vec::new();
    let (_, line) = line.split_once(':').unwrap();

    let (first, second) = line.split_once(" | ").unwrap();

    for piece in first.split(" ") {
        if piece.len() > 0 {
            let number = piece.parse().unwrap();
            winning.insert(number);
        }
    }

    for piece in second.split(" ") {
        if piece.len() > 0 {
            let number = piece.parse().unwrap();
            yours.push(number);
        }
    }
    return Card { winning, yours };
}

fn print_mapping(mapping: &HashMap<usize, usize>) {
    let mut sorted = mapping.keys().collect::<Vec<&usize>>();
    sorted.sort();
    for key in sorted {
        println!("{} => {}", key + 1, mapping.get(key).unwrap());
    }
    println!();
}
