mod line;
mod point;
mod utils;
use line::Line;
use point::Point;
use std::collections::HashSet;

use utils::get_lines;

fn main() {
    // let lines = get_lines::<String>("sample.txt");
    let lines = get_lines::<String>("input.txt");
    let segments = lines.iter().map(|line| parse(&line)).collect::<Vec<Line>>();

    // part_1(segments);
    part_2(segments);
}

fn part_1(mut segments: Vec<Line>) {
    // only keep vertical/horizontal
    segments.retain(|line| !line.is_diagonal());
    let mut unique_intersections: HashSet<Point> = HashSet::new();
    for (ia, line_a) in segments.iter().enumerate() {
        for line_b in segments.iter().skip(ia + 1) {
            let intersections = Line::intersections(line_a, line_b);
            for intersection in intersections {
                unique_intersections.insert(intersection);
            }
        }
    }
    dbg!(&unique_intersections);
    dbg!(unique_intersections.len());
}

fn part_2(segments: Vec<Line>) {
    let mut unique_intersections: HashSet<Point> = HashSet::new();
    for (ia, line_a) in segments.iter().enumerate() {
        for line_b in segments.iter().skip(ia + 1) {
            let intersections = Line::intersections(line_a, line_b);
            for intersection in intersections {
                unique_intersections.insert(intersection);
            }
        }
    }
    dbg!(&unique_intersections);
    dbg!(unique_intersections.len());
}

fn parse(line: &str) -> Line {
    let mut pieces = line.split(" -> ");
    return Line {
        p1: Point::new(pieces.next().unwrap()),
        p2: Point::new(pieces.next().unwrap()),
    };
}
