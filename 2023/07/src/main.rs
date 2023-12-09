#![allow(dead_code, unused_variables)]
mod hand_1;
mod hand_2;
mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let mut hands = utils::get_generic(contents, hand_1::Hand::parse_hand);
    hands.sort_by(|a, b| a.values.cmp(&b.values));

    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        dbg!(index + 1, hand.bid);
        total += (index + 1) * hand.bid;
    }

    // 252180334 is too low
    dbg!(total);
    // dbg!(&lines);
}

fn part_2(contents: &str) {
    let mut hands = utils::get_generic(contents, hand_2::Hand::parse_hand);
    hands.sort_by(|a, b| a.values.cmp(&b.values));

    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        // dbg!(index + 1, hand.bid);
        total += (index + 1) * hand.bid;
    }

    dbg!(total);
    // dbg!(&lines);
}
