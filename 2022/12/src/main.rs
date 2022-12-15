#![allow(dead_code, unused_variables)]
mod utils;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    char: char,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let lines: Vec<Vec<char>> = utils::get_generic(filename, |line| line.chars().collect());
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut target = (0, 0);

    // Once through to find the start and end (could exist early, but eh)
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    for y_pos in 0..num_rows {
        for x_pos in 0..num_cols {
            let char = lines[y_pos][x_pos];
            if char == 'S' {
                visited.insert((x_pos, y_pos));
                heap.push(State {
                    cost: 0,
                    char,
                    position: (x_pos, y_pos),
                });
            } else if char == 'E' {
                target = (x_pos, y_pos);
            }
        }
    }

    // Gonna do the easy thing and not keep track of where we've been
    // Inefficient, but oh well
    while !heap.is_empty() {
        // dbg!(&heap);
        let state = heap.pop().unwrap();
        // dbg!(&state);
        let (x_pos, y_pos) = state.position;

        for delta in [(0_i64, 1_i64), (0, -1), (1, 0), (-1, 0)] {
            if (delta.0 < 0 && x_pos == 0) || (delta.1 < 0 && y_pos == 0) {
                continue;
            }

            let x_pos = if delta.0 < 0 {
                x_pos - 1
            } else {
                x_pos + delta.0 as usize
            };

            let y_pos = if delta.1 < 0 {
                y_pos - 1
            } else {
                y_pos + delta.1 as usize
            };

            if x_pos < num_cols && y_pos < num_rows && !visited.contains(&(x_pos, y_pos)) {
                // dbg!(state.cost);
                // if x_pos < num_cols && y_pos < num_rows {
                let next_char = lines[y_pos][x_pos];
                let cost = step_cost(state.char, next_char);
                if cost > 0 {
                    // print_position(&lines, state.position, (x_pos, y_pos));
                    let new_cost = state.cost + cost;
                    if next_char == 'E' {
                        println!("We found it!");
                        dbg!(new_cost);
                        return;
                    }

                    heap.push(State {
                        cost: new_cost,
                        char: next_char,
                        position: (x_pos, y_pos),
                    });

                    visited.insert((x_pos, y_pos));
                }
            }
        }
    }

    println!("yo");
}

fn part_2(filename: &str) {
    let lines: Vec<Vec<char>> = utils::get_generic(filename, |line| line.chars().collect());
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut target = (0, 0);

    // Once through to find the start and end (could exist early, but eh)
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    for y_pos in 0..num_rows {
        for x_pos in 0..num_cols {
            let char = lines[y_pos][x_pos];
            if char == 'S' || char == 'a' {
                visited.insert((x_pos, y_pos));
                heap.push(State {
                    cost: 0,
                    char,
                    position: (x_pos, y_pos),
                });
            } else if char == 'E' {
                target = (x_pos, y_pos);
            }
        }
    }

    // Gonna do the easy thing and not keep track of where we've been
    // Inefficient, but oh well
    while !heap.is_empty() {
        // dbg!(&heap);
        let state = heap.pop().unwrap();
        // dbg!(&state);
        let (x_pos, y_pos) = state.position;

        for delta in [(0_i64, 1_i64), (0, -1), (1, 0), (-1, 0)] {
            if (delta.0 < 0 && x_pos == 0) || (delta.1 < 0 && y_pos == 0) {
                continue;
            }

            let x_pos = if delta.0 < 0 {
                x_pos - 1
            } else {
                x_pos + delta.0 as usize
            };

            let y_pos = if delta.1 < 0 {
                y_pos - 1
            } else {
                y_pos + delta.1 as usize
            };

            if x_pos < num_cols && y_pos < num_rows && !visited.contains(&(x_pos, y_pos)) {
                // dbg!(state.cost);
                // if x_pos < num_cols && y_pos < num_rows {
                let next_char = lines[y_pos][x_pos];
                let cost = step_cost(state.char, next_char);
                if cost > 0 {
                    // print_position(&lines, state.position, (x_pos, y_pos));
                    let new_cost = state.cost + cost;
                    if next_char == 'E' {
                        println!("We found it!");
                        dbg!(new_cost);
                        return;
                    }

                    heap.push(State {
                        cost: new_cost,
                        char: next_char,
                        position: (x_pos, y_pos),
                    });

                    visited.insert((x_pos, y_pos));
                }
            }
        }
    }

    println!("yo");
}

fn print_position(lines: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {
    let num_rows = lines.len();
    let num_cols = lines[0].len();

    for y_pos in 0..num_rows {
        for x_pos in 0..num_cols {
            if (x_pos, y_pos) == start {
                print!("S");
            } else if (x_pos, y_pos) == end {
                print!("E");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn step_cost(char: char, next_char: char) -> usize {
    let char = if char == 'S' { 'a' } else { char };
    let next_char = if next_char == 'E' { 'z' } else { next_char };

    let char = char as u32;
    let next_char = next_char as u32;

    if next_char <= char {
        return 1;
    }

    if char + 1 == next_char {
        return 1;
    }

    return 0;
}
