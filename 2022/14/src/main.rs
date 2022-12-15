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
    let lines: Vec<Vec<(isize, isize)>> = utils::get_generic(filename, parse_line);

    let mut map: HashSet<(isize, isize)> = HashSet::new();
    let mut floor = 0;
    for line in lines {
        let mut current_point: Option<(isize, isize)> = None;
        for point in line {
            match current_point {
                None => {}
                Some(old_point) => {
                    let min_x = point.0.min(old_point.0);
                    let min_y = point.1.min(old_point.1);
                    let max_x = point.0.max(old_point.0) + 1;
                    let max_y = point.1.max(old_point.1) + 1;

                    for x in min_x..max_x {
                        for y in min_y..max_y {
                            if y > floor {
                                floor = y;
                            }
                            map.insert((x, y));
                        }
                    }
                }
            };
            current_point = Some(point);
        }
    }

    let mut count = 0;
    loop {
        if spawn_sand(&mut map, (500, 0), floor) {
            count += 1;
        } else {
            break;
        }
    }

    dbg!(&count);
}

fn part_2(filename: &str) {
    let lines: Vec<Vec<(isize, isize)>> = utils::get_generic(filename, parse_line);

    let mut map: HashSet<(isize, isize)> = HashSet::new();
    let mut floor = 0;
    for line in lines {
        let mut current_point: Option<(isize, isize)> = None;
        for point in line {
            match current_point {
                None => {}
                Some(old_point) => {
                    let min_x = point.0.min(old_point.0);
                    let min_y = point.1.min(old_point.1);
                    let max_x = point.0.max(old_point.0) + 1;
                    let max_y = point.1.max(old_point.1) + 1;

                    for x in min_x..max_x {
                        for y in min_y..max_y {
                            if y > floor {
                                floor = y;
                            }
                            map.insert((x, y));
                        }
                    }
                }
            };
            current_point = Some(point);
        }
    }

    let mut count = 0;
    loop {
        // infinite floor is two below
        if spawn_sand_2(&mut map, (500, 0), floor + 2) {
            count += 1;
        } else {
            break;
        }
    }

    dbg!(&count);
}

fn spawn_sand(
    map: &mut HashSet<(isize, isize)>,
    mut position: (isize, isize),
    floor: isize,
) -> bool {
    loop {
        let positions = [
            (position.0, position.1 + 1),
            (position.0 - 1, position.1 + 1),
            (position.0 + 1, position.1 + 1),
        ];

        match positions.iter().find(|p| !map.contains(p)) {
            None => {
                map.insert(position);
                return true;
            }
            Some(new_position) => {
                position = *new_position;
            }
        };

        if position.1 > floor {
            return false;
        }
    }
}

fn spawn_sand_2(
    map: &mut HashSet<(isize, isize)>,
    mut position: (isize, isize),
    floor: isize,
) -> bool {
    // we've covere the exit
    if map.contains(&position) {
        return false;
    }

    loop {
        let positions = [
            (position.0, position.1 + 1),
            (position.0 - 1, position.1 + 1),
            (position.0 + 1, position.1 + 1),
        ];

        // hacky - just build any kind of floor as we need it. Waaaay more insertions
        // than necessary
        for position in positions {
            if position.1 >= floor {
                map.insert(position);
            }
        }

        match positions.iter().find(|p| !map.contains(p)) {
            None => {
                map.insert(position);
                return true;
            }
            Some(new_position) => {
                position = *new_position;
            }
        };
    }
}

// 498,4 -> 498,6 -> 496,6
fn parse_line(line: &str) -> Vec<(isize, isize)> {
    return line
        .split(" -> ")
        .map(|point| -> (isize, isize) {
            let (a, b) = point.split_once(",").unwrap();
            return (a.parse().unwrap(), b.parse().unwrap());
        })
        .collect();
}
