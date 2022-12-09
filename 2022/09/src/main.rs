#![allow(dead_code, unused_variables)]

use std::{collections::HashSet, io::stdin};
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let lines = utils::get_lines::<String>(filename);
    let mut positions = HashSet::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    positions.insert(tail);

    for line in lines {
        let (direction, count) = line.split_once(" ").unwrap();
        let count = count.parse::<i32>().unwrap();

        // move head
        match direction {
            "R" => {
                head.0 += count;
            }
            "L" => {
                head.0 -= count;
            }
            "U" => {
                head.1 += count;
            }
            "D" => {
                head.1 -= count;
            }
            _ => panic!(),
        };

        let mut delta_x = head.0 - tail.0;
        let mut delta_y = head.1 - tail.1;

        // move diagonally (this is slow)
        while delta_x != 0 && delta_y != 0 && delta_x.abs() + delta_y.abs() > 2 {
            if delta_x.is_negative() {
                tail.0 -= 1;
            } else {
                tail.0 += 1;
            }

            if delta_y.is_negative() {
                tail.1 -= 1;
            } else {
                tail.1 += 1;
            }

            delta_x = head.0 - tail.0;
            delta_y = head.1 - tail.1;

            positions.insert(tail);
        }

        // y's are the same, adjust just the x's
        if delta_y == 0 {
            while delta_x.abs() > 1 {
                if delta_x.is_negative() {
                    tail.0 -= 1;
                } else if delta_x.is_positive() {
                    tail.0 += 1;
                }

                delta_x = head.0 - tail.0;
                positions.insert(tail);
            }
        }

        // x's are the same, adjust just the y's
        if delta_x == 0 {
            while delta_y.abs() > 1 {
                if delta_y.is_negative() {
                    tail.1 -= 1;
                } else if delta_y.is_positive() {
                    tail.1 += 1;
                }

                delta_y = head.1 - tail.1;
                positions.insert(tail);
            }
        }
        // positions.insert(head);
    }

    print_positions(&positions, &head, &tail);
    dbg!(positions.len());
}

fn print_positions(positions: &HashSet<(i32, i32)>, head: &(i32, i32), tail: &(i32, i32)) {
    for y in (0..5).rev() {
        for x in 0..7 {
            if head == &(x, y) && head == tail {
                print!("B");
            } else if head == &(x, y) {
                print!("H");
            } else if tail == &(x, y) {
                print!("T");
            } else if positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_positions_2(positions: &HashSet<(i32, i32)>, head: (i32, i32), tails: &[(i32, i32)]) {
    let tail = *tails.last().unwrap();

    for y in (-20..20).rev() {
        for x in -15..20 {
            if x == 0 && y == 0 {
                print!("s");
            } else if head == (x, y) && head == tail {
                print!("B");
            } else if head == (x, y) {
                print!("H");
            } else if tail == (x, y) {
                print!("T");
            } else if positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part_2(filename: &str) {
    let lines = utils::get_lines::<String>(filename);
    let mut positions = HashSet::new();
    let mut head: (i32, i32) = (0, 0);

    let mut tails: [(i32, i32); 9] = [(0, 0); 9];

    positions.insert(tails.last().unwrap().clone());

    // let mut s = "".into();
    for line in lines {
        // print_positions(&positions, &head, &tail);
        // let input = stdin().read_line(&mut s);

        let (direction, count) = line.split_once(" ").unwrap();
        let mut count = count.parse::<i32>().unwrap();
        // dbg!(&head);

        while count > 0 {
            count -= 1;

            // move head
            match direction {
                "R" => {
                    head.0 += 1;
                }
                "L" => {
                    head.0 -= 1;
                }
                "U" => {
                    head.1 += 1;
                }
                "D" => {
                    head.1 -= 1;
                }
                _ => panic!(),
            };

            do_move(&mut tails, &head, &mut positions);

            // let mut s = "".to_string();
            // print_positions_2(&positions, head, &tails);
            // let input = stdin().read_line(&mut s);
        }
    }

    print_positions_2(&positions, head, &tails);
    dbg!(positions.len());
}

fn do_move(tails: &mut [(i32, i32)], head: &(i32, i32), positions: &mut HashSet<(i32, i32)>) {
    let mut head = head.clone();
    let num_tails = tails.len();
    for (index, tail) in tails.iter_mut().enumerate() {
        let is_last = index == num_tails - 1;
        let mut delta_x = head.0 - tail.0;
        let mut delta_y = head.1 - tail.1;

        // move diagonally (this is slow)
        while delta_x != 0 && delta_y != 0 && delta_x.abs() + delta_y.abs() > 2 {
            if delta_x.is_negative() {
                (*tail).0 -= 1;
            } else {
                tail.0 += 1;
            }

            if delta_y.is_negative() {
                tail.1 -= 1;
            } else {
                tail.1 += 1;
            }

            delta_x = head.0 - tail.0;
            delta_y = head.1 - tail.1;

            if is_last {
                positions.insert(*tail);
            }
        }

        // y's are the same, adjust just the x's
        if delta_y == 0 {
            while delta_x.abs() > 1 {
                if delta_x.is_negative() {
                    tail.0 -= 1;
                } else if delta_x.is_positive() {
                    tail.0 += 1;
                }

                delta_x = head.0 - tail.0;
                if is_last {
                    positions.insert(*tail);
                }
            }
        }

        // x's are the same, adjust just the y's
        if delta_x == 0 {
            while delta_y.abs() > 1 {
                if delta_y.is_negative() {
                    tail.1 -= 1;
                } else if delta_y.is_positive() {
                    tail.1 += 1;
                }

                delta_y = head.1 - tail.1;
                if is_last {
                    positions.insert(*tail);
                }
            }
        }
        head = tail.clone();
    }
}
