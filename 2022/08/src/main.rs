#![allow(dead_code, unused_variables)]

use std::collections::HashSet;
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let trees = utils::get_trees(filename);

    // idea: iterate left and right, right to left, up/down, down/up,
    // setting all visible trees to negative. Then count the positive trees that are left.
    let rows = trees.len();
    let cols = trees[0].len();

    let mut visible_trees = HashSet::new();

    let mut max = 0;
    for i in 0..rows {
        for j in (0..cols).chain((0..cols).rev()) {
            let is_border = i == 0 || j == 0 || i == rows - 1 || j == cols - 1;
            let value = trees[i][j];

            if is_border || value > max {
                // tree is definitely visible (or a border)
                max = value;
                visible_trees.insert((i, j));
            }
        }
    }

    for j in 0..cols {
        for i in (0..rows).chain((0..rows).rev()) {
            let is_border = i == 0 || j == 0 || i == rows - 1 || j == cols - 1;
            let value = trees[i][j];

            if is_border || value > max {
                // tree is definitely visible (or a border)
                max = value;
                visible_trees.insert((i, j));
            }
        }
    }

    for (i, row) in trees.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if visible_trees.contains(&(i, j)) {
                print!("  -{}", value);
            } else {
                print!("   {}", value);
            }
        }
        println!("");
    }

    dbg!(visible_trees.len());
}

fn part_2(filename: &str) {
    let trees = utils::get_trees(filename);

    let rows = trees.len();
    let cols = trees[0].len();

    let mut max_score = 0;
    for i in 0..rows {
        for j in 0..cols {
            let score = get_score(&trees, i, j);
            if score > max_score {
                max_score = score;
                // dbg!(max_score, (i, j));
            }
        }
    }
    dbg!(max_score);

    // dbg!(get_score(&trees, 2, 0));
}

fn get_score(trees: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let rows = trees.len();
    let cols = trees[0].len();

    let mut values = Vec::new();
    let current_tree = trees[i][j];

    // down
    let mut count = 0;
    for i in (i + 1)..rows {
        count += 1;
        if i == rows - 1 || trees[i][j] >= current_tree {
            break;
        }
    }
    values.push(count);

    // up
    let mut count = 0;
    for i in (0..i).rev() {
        count += 1;
        if i == 0 || trees[i][j] >= current_tree {
            break;
        }
    }
    values.push(count);

    // right
    let mut count = 0;
    for j in (j + 1)..cols {
        count += 1;
        if j == rows - 1 || trees[i][j] >= current_tree {
            break;
        }
    }
    values.push(count);

    // left
    let mut count = 0;
    for j in (0..j).rev() {
        count += 1;
        if j == 0 || trees[i][j] >= current_tree {
            break;
        }
    }
    values.push(count);

    // dbg!(&values);
    return values.iter().fold(1, |acc, v| v * acc);
}
