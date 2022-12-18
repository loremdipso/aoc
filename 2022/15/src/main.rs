#![allow(dead_code, unused_variables)]

use std::collections::HashSet;
mod utils;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: &str, y: &str) -> Self {
        return Self::new_basic(x.parse().unwrap(), y.parse().unwrap());
    }
    pub fn new_basic(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let points = utils::get_generic(filename, parse_line);

    // let y = 10;
    let y = 2000000;
    let count = count_cols(&points, y);
    // dbg!(&points);
    dbg!(count);
}

fn part_2(filename: &str) {
    let points = utils::get_generic(filename, parse_line);

    let beacon = find_beacon(&points);
    dbg!(&beacon);

    let answer = beacon.x * 4000000 + beacon.y;
    dbg!(answer);
}

fn count_cols(points: &Vec<(Point, Point)>, y: isize) -> usize {
    let mut map: HashSet<Point> = HashSet::new();

    for (sensor, beacon) in points {
        let distance_to_sensor = manhattan_distance(sensor, beacon);

        // start at middle, go left and right simultaneously since that's fine
        let mut x_delta = 0;
        loop {
            let x = sensor.x;
            let point_a = Point::new_basic(x + x_delta, y);
            let point_b = Point::new_basic(x - x_delta, y);
            if manhattan_distance(&point_a, sensor) <= distance_to_sensor {
                map.insert(point_a);
                map.insert(point_b);
            } else {
                break;
            }
            x_delta += 1;
        }
    }
    panic!();
}

fn find_beacon(points: &Vec<(Point, Point)>) -> Point {
    // Idea: go along the edge of each sensor and make sure there's no intersection.
    // It must be on the edge, otherwise there'd be multiple points that match.
    for (sensor, beacon) in points {
        let distance_to_sensor = manhattan_distance(sensor, beacon);

        let max_delta_y = distance_to_sensor + 2;
        for y_delta in -max_delta_y..max_delta_y {
            let temp = (distance_to_sensor + 1 - y_delta.abs()).abs();
            for x_delta in temp..temp + 2 {
                let y = sensor.y + y_delta;

                let point_a = Point::new_basic(sensor.x + x_delta, y);

                // sanity check
                if manhattan_distance(&point_a, sensor) <= distance_to_sensor {
                    panic!();
                }

                if valid(&point_a) && !intersects(points, &point_a) {
                    return point_a;
                }

                let point_b = Point::new_basic(sensor.x - x_delta, y);
                if valid(&point_b) && !intersects(points, &point_b) {
                    return point_b;
                }
            }
        }
    }

    panic!();
}

// <ake sure the point is within range
fn valid(point: &Point) -> bool {
    return point.x >= 0 && point.y >= 0 && point.x < 4000000 && point.y < 4000000;
}

// Does some point intersect with any of the sensor ranges?
fn intersects(points: &Vec<(Point, Point)>, point: &Point) -> bool {
    for (sensor, beacon) in points {
        let distance_to_beacon = manhattan_distance(sensor, beacon);
        let distance_to_point = manhattan_distance(sensor, point);
        if distance_to_point <= distance_to_beacon {
            return true;
        }
    }
    return false;
}

fn manhattan_distance(sensor: &Point, beacon: &Point) -> isize {
    return (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
}

fn parse_line(line: &str) -> (Point, Point) {
    let (sensor, beacon) = line.split_once(": ").unwrap();
    let (sensor_x, sensor_y) = sensor.split_once(", ").unwrap();
    let (beacon_x, beacon_y) = beacon.split_once(", ").unwrap();
    let (_, sensor_x) = sensor_x.split_once("=").unwrap();
    let (_, sensor_y) = sensor_y.split_once("=").unwrap();
    let (_, beacon_x) = beacon_x.split_once("=").unwrap();
    let (_, beacon_y) = beacon_y.split_once("=").unwrap();

    return (
        Point::new(sensor_x, sensor_y),
        Point::new(beacon_x, beacon_y),
    );
}
