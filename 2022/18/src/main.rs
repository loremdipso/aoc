#![allow(dead_code, unused_variables)]

use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
};
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point(isize, isize, isize);

struct Bounds(isize, isize, isize, isize, isize, isize);
impl Bounds {
    fn within_bounds(&self, adj_point: Point) -> bool {
        let slop = 1;
        return adj_point.0 > self.0 - slop
            && adj_point.0 < self.1 + slop
            && adj_point.0 > self.2 - slop
            && adj_point.0 < self.3 + slop
            && adj_point.0 > self.4 - slop
            && adj_point.0 < self.5 + slop;
    }
}

impl Add for Point {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

fn parse(filename: &str) -> HashSet<Point> {
    return utils::get_generic_set(filename, |l| -> Point {
        let mut split = l.split(",");
        Point(
            split.next().unwrap().parse::<isize>().unwrap(),
            split.next().unwrap().parse::<isize>().unwrap(),
            split.next().unwrap().parse::<isize>().unwrap(),
        )
    });
}

const DELTAS: [Point; 6] = [
    Point(0, 0, 1),
    Point(0, 0, -1),
    Point(0, 1, 0),
    Point(0, -1, 0),
    Point(1, 0, 0),
    Point(-1, 0, 0),
];

const DELTA: f64 = 0.0001;

fn part_1(filename: &str) {
    let cubes: HashSet<Point> = parse(filename);

    let mut total = 0;
    for cube in &cubes {
        for delta in DELTAS {
            let adj_point = *cube + delta;
            if !cubes.contains(&adj_point) {
                total += 1;
            }
        }
    }

    dbg!(total);
}

fn part_2(filename: &str) {
    let cubes = parse(filename);
    let mut land_locked: HashSet<Point> = HashSet::new();

    let bounds = Bounds(
        // x
        cubes.iter().map(|c| c.0).min().unwrap(),
        cubes.iter().map(|c| c.0).max().unwrap(),
        // y
        cubes.iter().map(|c| c.1).min().unwrap(),
        cubes.iter().map(|c| c.1).max().unwrap(),
        // z
        cubes.iter().map(|c| c.2).min().unwrap(),
        cubes.iter().map(|c| c.2).max().unwrap(),
    );

    let mut total = 0;
    for (index, cube) in cubes.iter().enumerate() {
        if index % 50 == 0 {
            println!("{} / {}", index, cubes.len());
        }

        for delta in DELTAS {
            let adj_point = *cube + delta;
            if !cubes.contains(&adj_point) {
                if can_reach_water(&cubes, &land_locked, &bounds, &adj_point) {
                    total += 1;
                } else {
                    land_locked.insert(adj_point);
                }
            }
        }
    }

    dbg!(total);
}

fn can_reach_water(
    cubes: &HashSet<Point>,
    land_locked: &HashSet<Point>,
    bounds: &Bounds,
    point: &Point,
) -> bool {
    let mut seen = HashSet::new();
    let mut points = VecDeque::new();
    points.push_back(*point);

    while let Some(point) = points.pop_front() {
        for delta in DELTAS {
            let adj_point = point + delta;

            // if we escaped the bounds then we're gucci
            if !bounds.within_bounds(adj_point) {
                return true;
            }

            // if we touched a square we already tried then we're not gucci :/
            // (slight optimization)
            if land_locked.contains(&adj_point) {
                return false;
            }

            if !seen.contains(&adj_point) && !cubes.contains(&adj_point) {
                seen.insert(adj_point);
                points.push_back(adj_point);
            }
        }
    }

    return false;
}
