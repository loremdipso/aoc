#![allow(dead_code, unused_variables)]
mod utils;
use std::cmp::{max, min};

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let (positions, empty_rows, empty_columns) = get_data(contents);
    let mut total = 0;
    for (index, p1) in positions.iter().enumerate() {
        for p2 in &positions[index + 1..] {
            let distance = get_distance_naive(p1, p2, &empty_rows, &empty_columns, 2);
            total += distance;
        }
    }
    dbg!(total);
}

fn part_2(contents: &str) {
    let (positions, empty_rows, empty_columns) = get_data(contents);
    let mut total = 0;
    for (index, p1) in positions.iter().enumerate() {
        for p2 in &positions[index + 1..] {
            let distance = get_distance_naive(p1, p2, &empty_rows, &empty_columns, 1_000_000);
            total += distance;
            // dbg!(distance);
            // dbg!(p1, p2, distance);
        }
    }
    dbg!(total);
}

type Position = (usize, usize);

fn get_distance_naive(
    p1: &Position,
    p2: &Position,
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
    mult: usize,
) -> usize {
    let min_row = min(p1.0, p2.0);
    let mut max_row = max(p1.0, p2.0);
    let min_col = min(p1.1, p2.1);
    let mut max_col = max(p1.1, p2.1);

    for row in min_row..max_row {
        if empty_rows.binary_search(&row).is_ok() {
            max_row += mult - 1;
        }
    }

    for col in min_col..max_col {
        if empty_columns.binary_search(&col).is_ok() {
            max_col += mult - 1;
        }
    }

    return (max_row - min_row) + (max_col - min_col);
}

fn get_data(contents: &str) -> (Vec<Position>, Vec<usize>, Vec<usize>) {
    let grid = utils::get_grid(contents);
    let mut positions = Vec::new();
    let mut empty_rows = Vec::new();
    let mut empty_columns = Vec::new();

    for (line_index, line) in grid.iter().enumerate() {
        let mut empty_row = true;
        for (char_index, char) in line.iter().enumerate() {
            if *char == '#' {
                positions.push((line_index, char_index));
                empty_row = false;
            }
        }
        if empty_row {
            empty_rows.push(line_index);
        }
    }

    let num_chars = grid[0].len();
    for char_index in 0..num_chars {
        let mut empty_column = true;
        for (line_index, line) in grid.iter().enumerate() {
            if line[char_index] == '#' {
                empty_column = false;
            }
        }
        if empty_column {
            empty_columns.push(char_index);
        }
    }

    return (positions, empty_rows, empty_columns);
}
