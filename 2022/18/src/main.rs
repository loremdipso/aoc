#![allow(dead_code, unused_variables)]

use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet, VecDeque};
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);

    // 4050 is too high
    part_2(filename);
}

type Point = (OrderedFloat<f64>, OrderedFloat<f64>, OrderedFloat<f64>);

fn parse(filename: &str) -> Vec<Point> {
    return utils::get_generic(filename, |l| -> Point {
        let mut split = l.split(",");
        (
            OrderedFloat(split.next().unwrap().parse::<f64>().unwrap()),
            OrderedFloat(split.next().unwrap().parse::<f64>().unwrap()),
            OrderedFloat(split.next().unwrap().parse::<f64>().unwrap()),
        )
    });
}

const SIDE_DELTAS: [(f64, f64, f64); 6] = [
    (0_f64, 0_f64, 0.5_f64),
    (0_f64, 0_f64, -0.5_f64),
    (0_f64, 0.5_f64, 0_f64),
    (0_f64, -0.5_f64, 0_f64),
    (0.5_f64, 0_f64, 0_f64),
    (-0.5_f64, 0_f64, 0_f64),
];

const CUBE_DELTAS: [(f64, f64, f64); 6] = [
    (0_f64, 0_f64, 1_f64),
    (0_f64, 0_f64, -1_f64),
    (0_f64, 1_f64, 0_f64),
    (0_f64, -1_f64, 0_f64),
    (1_f64, 0_f64, 0_f64),
    (-1_f64, 0_f64, 0_f64),
];

const MAX_ITS: usize = 1500;

const DELTA: f64 = 0.0001;

fn part_1(filename: &str) {
    let mut all_sides: HashSet<Point> = HashSet::new();
    let mut covered_sides: HashSet<Point> = HashSet::new();
    let cubes = parse(filename);

    for cube in &cubes {
        for delta in SIDE_DELTAS {
            let side = (cube.0 + delta.0, cube.1 + delta.1, cube.2 + delta.2);
            if all_sides.contains(&side) {
                covered_sides.insert(side);
            }
            all_sides.insert(side);
        }
    }

    let answer = all_sides.len() - covered_sides.len();
    dbg!(answer);
}

fn part_2(filename: &str) {
    let mut all_sides: HashSet<Point> = HashSet::new();
    let mut covered_sides: HashSet<Point> = HashSet::new();
    let cubes = parse(filename);

    for cube in &cubes {
        for delta in SIDE_DELTAS {
            let side = (cube.0 + delta.0, cube.1 + delta.1, cube.2 + delta.2);
            if all_sides.contains(&side) {
                covered_sides.insert(side);
            }
            all_sides.insert(side);
        }
    }

    // for side in &all_sides {
    //     if covered_sides.contains(side) {
    //         println!("{:4}, {:4}, {:4}", side.0, side.1, side.2);
    //     }
    // }

    // Now, we just need to recursively find all air pockets.
    // DFS every side? There aren't that many
    let mut internal_sides: usize = 0;
    for (index, side) in all_sides.iter().enumerate() {
        println!("{} / {}", index, all_sides.len());
        if !covered_sides.contains(side) {
            if is_internal(&cubes, &all_sides, side) {
                internal_sides += 1;
            }
        }
    }

    let answer = all_sides.len() - covered_sides.len() - internal_sides;
    dbg!(answer);
}

fn is_internal(cubes: &Vec<Point>, sides: &HashSet<Point>, side: &Point) -> bool {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut points: VecDeque<Point> = VecDeque::new();
    points.push_back(side.clone());

    let mut count = 0;
    while let Some(point) = points.pop_front() {
        count += 1;
        if count > MAX_ITS {
            return false;
        }

        for delta in SIDE_DELTAS {
            let new_point = (point.0 + delta.0, point.1 + delta.1, point.2 + delta.2);

            if valid_point(&new_point) {
                if !visited.contains(&new_point)
                    && !cubes.contains(&new_point)
                    && !sides.contains(&new_point)
                {
                    visited.insert(new_point);
                    points.push_front(new_point);
                }
            }
        }
    }

    return true;
}

fn valid_point(new_point: &Point) -> bool {
    let mut have_half = false;

    if new_point.0.abs() % 1_f64 > DELTA {
        if have_half {
            return false;
        }
        have_half = true;
    }

    if new_point.1.abs() % 1_f64 > DELTA {
        if have_half {
            return false;
        }
        have_half = true;
    }

    if new_point.2.abs() % 1_f64 > DELTA {
        if have_half {
            return false;
        }
        have_half = true;
    }

    return true;
}
