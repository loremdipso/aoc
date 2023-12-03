#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let games = utils::get_generic(contents, convert_line);
    let mut bag: HashMap<String, u32> = HashMap::new();
    bag.insert("red".into(), 12);
    bag.insert("green".into(), 13);
    bag.insert("blue".into(), 14);

    let mut total = 0;
    'outer: for (idx, game) in games.iter().enumerate() {
        println!("");
        for round in game {
            for (key, value) in round {
                if value > bag.get(key).unwrap() {
                    continue 'outer;
                }
            }
        }
        total += idx + 1;
    }

    dbg!(&games);
    dbg!(total);
}

fn part_2(contents: &str) {
    let games = utils::get_generic(contents, convert_line);

    let mut total = 0;
    for (idx, game) in games.iter().enumerate() {
        let mut maxes: HashMap<String, u32> = HashMap::new();

        for round in game {
            for (key, value) in round {
                maxes
                    .entry(key.to_string())
                    .and_modify(|old_value| *old_value = std::cmp::max(*old_value, *value))
                    .or_insert(*value);
            }
        }
        let mut product = 1;
        for max in maxes.values() {
            product *= max;
        }
        total += product;
    }

    dbg!(&games);
    dbg!(total);
}

fn convert_line(line: &str) -> Vec<HashMap<String, u32>> {
    let mut rv = Vec::new();
    for line in line.split(':').skip(1) {
        let line = line.trim();
        for game in line.split(';') {
            let mut temp = HashMap::new();
            for piece in game.split(',') {
                let mut pieces = piece.trim().split(' ');
                let value = pieces.next().unwrap().parse::<u32>().unwrap();
                temp.insert(pieces.next().unwrap().to_string(), value);
            }
            rv.push(temp);
        }
    }
    return rv;
}
