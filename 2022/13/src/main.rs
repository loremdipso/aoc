#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use std::{cmp::Ordering, iter::Peekable, rc::Rc, slice::Iter};
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

#[derive(Debug, Clone, PartialEq)]
enum Item {
    List(Vec<Item>),
    Value(usize),
}

fn part_1(filename: &str) {
    let groups = utils::get_groups::<String>(filename);
    let groups: Vec<(Item, Item)> = groups.iter().map(|g| parse_groups(g)).collect();

    let mut indices = Vec::new();
    for (group_index, group) in groups.iter().enumerate() {
        match right_order(&group.0, &group.1) {
            OrderResult::Maybe | OrderResult::Yes => {
                indices.push(group_index + 1);
            }
            _ => {}
        }
    }

    // dbg!(&groups);
    dbg!(&indices);
    dbg!(&indices.iter().sum::<usize>());
}

fn part_2(filename: &str) {
    let raw_groups = utils::get_groups::<String>(filename);
    let mut groups: Vec<Item> = vec![];

    for group in raw_groups {
        for item in group {
            groups.push(parse_group(
                &mut item.chars().collect::<Vec<char>>()[1..].iter().peekable(),
            ));
        }
    }

    let packet1 = Item::List(vec![Item::List(vec![Item::Value(2)])]);
    let packet2 = Item::List(vec![Item::List(vec![Item::Value(6)])]);
    groups.push(packet1.clone());
    groups.push(packet2.clone());

    groups.sort_by(|a, b| -> Ordering {
        match right_order(a, b) {
            OrderResult::Maybe => Ordering::Equal,
            OrderResult::Yes => Ordering::Less,
            OrderResult::No => Ordering::Greater,
        }
    });

    let mut packet1_location = 0;
    let mut packet2_location = 0;
    for (index, group) in groups.iter().enumerate() {
        if *group == packet1 {
            packet1_location = index + 1;
        } else if *group == packet2 {
            packet2_location = index + 1;
        }
    }

    dbg!(&groups);
    dbg!(packet1_location, packet2_location);
    dbg!(packet1_location * packet2_location);
}

#[derive(Debug)]
enum OrderResult {
    Yes,
    No,
    Maybe,
}

fn right_order(left: &Item, right: &Item) -> OrderResult {
    match (left, right) {
        (Item::Value(left_value), Item::Value(right_value)) => {
            if left_value == right_value {
                return OrderResult::Maybe;
            } else if left_value < right_value {
                return OrderResult::Yes;
            } else {
                return OrderResult::No;
            }
        }
        (Item::List(left_value), Item::Value(right_value)) => {
            return right_order(left, &Item::List(vec![right.clone()]));
        }
        (Item::Value(left_value), Item::List(right_value)) => {
            return right_order(&Item::List(vec![left.clone()]), right);
        }
        (Item::List(left_values), Item::List(right_values)) => {
            let mut lit = left_values.iter().peekable();
            let mut rit = right_values.iter().peekable();

            loop {
                let left = lit.peek();
                let right = rit.peek();

                if right.is_none() {
                    if left.is_none() {
                        return OrderResult::Maybe;
                    }
                    return OrderResult::No;
                }

                if left.is_none() {
                    // equal size, can't determine
                    // if right.is_none() {
                    // return OrderResult::Yes;
                    // }
                    return OrderResult::Yes;
                }

                match right_order(left.unwrap(), right.unwrap()) {
                    OrderResult::Maybe => {
                        // same, continue on
                        lit.next();
                        rit.next();
                    }
                    OrderResult::No => {
                        return OrderResult::No;
                    }
                    OrderResult::Yes => {
                        return OrderResult::Yes;
                    }
                };
            }
        }
    };
}

fn parse_groups(g: &Vec<String>) -> (Item, Item) {
    return (
        parse_group(&mut g[0].chars().collect::<Vec<char>>()[1..].iter().peekable()),
        parse_group(&mut g[1].chars().collect::<Vec<char>>()[1..].iter().peekable()),
    );
}

fn parse_group(chars: &mut Peekable<Iter<char>>) -> Item {
    match chars.peek() {
        None => {
            return Item::List(Vec::new());
        }
        _ => {}
    };

    let mut items: Vec<Item> = Vec::new();
    let mut current_number: String = String::new();

    while let Some(char) = chars.next() {
        match char {
            '[' => {
                deal_with_current_number(&mut items, &mut current_number);
                items.push(parse_group(chars));
            }

            ']' => {
                deal_with_current_number(&mut items, &mut current_number);
                return Item::List(items);
            }

            ',' => {
                deal_with_current_number(&mut items, &mut current_number);
            }

            _ => {
                current_number.push(*char);
            }
        }
    }

    panic!();
}

fn deal_with_current_number(items: &mut Vec<Item>, current_number: &mut String) {
    if current_number.len() > 0 {
        let number = &current_number.parse::<usize>().unwrap();
        items.push(Item::Value(*number));
        current_number.clear();
    }
}
