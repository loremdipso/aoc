mod utils;
use std::collections::{HashMap, HashSet};

use utils::get_lines;

fn main() {
    // let lines = get_lines("sample.txt");
    let lines = get_lines("input.txt");
    // part_1(lines);
    part_2(lines);
}

fn part_1(lines: Vec<Vec<i64>>) {
    let mut num_low_points = 0;
    let width = lines[0].len();
    for y in 0..lines.len() {
        for x in 0..width {
            if lowest(&lines, x, y) {
                let value = lines[y][x];
                // dbg!((x, y));
                // dbg!(value);
                num_low_points += 1 + value;
            }
        }
    }
    dbg!(num_low_points);
}

fn lowest(lines: &Vec<Vec<i64>>, x: usize, y: usize) -> bool {
    let value = lines[y][x];
    return adj_points(lines, x, y)
        .iter()
        .all(|(x, y)| value < lines[*y][*x]);
}

fn part_2(lines: Vec<Vec<i64>>) {
    let mut basins = vec![];
    let width = lines[0].len();
    for y in 0..lines.len() {
        for x in 0..width {
            if lowest(&lines, x, y) {
                basins.push(size_of_basin(&lines, x, y));
            }
        }
    }
    basins.sort();
    let basins = &basins[(basins.len() - 3)..];
    dbg!(&basins);
    let mut answer = 1;
    for basin in basins {
        answer *= basin;
    }
    dbg!(answer);
}

fn adj_points(lines: &Vec<Vec<i64>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut rv = vec![];
    let width = lines[0].len();
    let height = lines.len();
    if x > 0 {
        rv.push((x - 1, y));
    }
    if x < width - 1 {
        rv.push((x + 1, y));
    }
    if y > 0 {
        rv.push((x, y - 1));
    }
    if y < height - 1 {
        rv.push((x, y + 1));
    }
    return rv;
}

fn size_of_basin(lines: &Vec<Vec<i64>>, x: usize, y: usize) -> usize {
    let mut basin: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = vec![(x, y)];
    loop {
        if let Some((x, y)) = stack.pop() {
            basin.insert((x, y));
            for point in adj_points(&lines, x, y) {
                let (x, y) = point;
                let point_value = lines[y][x];
                if point_value < 9 && !basin.contains(&point) {
                    basin.insert(point);
                    stack.push(point);
                }
            }
        } else {
            break;
        }
    }
    return basin.len();
}
