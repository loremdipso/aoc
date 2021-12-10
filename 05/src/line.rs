use std::{cmp::max, cmp::min, collections::HashSet, mem::swap};

use crate::Point;

#[derive(Debug)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn intersections(l1: &Line, l2: &Line) -> HashSet<Point> {
        let mut intersections = HashSet::new();

        if l1.is_diagonal() || l2.is_diagonal() {
            let overlap = Line::diagonal_overlap(l1, l2);
            for point in overlap {
                intersections.insert(point);
            }
        } else if l1.is_horizontal() && l2.is_horizontal() {
            // both are horizontal
            if l1.p1.y == l2.p1.y {
                let overlap = Line::contiguous_overlap(l1.p1.x, l1.p2.x, l2.p1.x, l2.p2.x);
                let y = l1.p1.y;
                for location in overlap {
                    intersections.insert(Point { x: location, y });
                }
            }
        } else if l1.is_vertical() && l2.is_vertical() {
            if l1.p1.x == l2.p1.x {
                let overlap = Line::contiguous_overlap(l1.p1.y, l1.p2.y, l2.p1.y, l2.p2.y);
                let x = l1.p1.x;
                for location in overlap {
                    intersections.insert(Point { x, y: location });
                }
            }
        } else {
            // one vertical one horizontal, at most one point
            if let Some(point) = Line::overlap(l1, l2) {
                intersections.insert(point);
            }
        }

        return intersections;
    }

    pub fn is_diagonal(&self) -> bool {
        return !(self.is_vertical() || self.is_horizontal());
    }

    pub fn is_vertical(&self) -> bool {
        return self.p1.x == self.p2.x;
    }

    pub fn is_horizontal(&self) -> bool {
        return self.p1.y == self.p2.y;
    }

    pub fn contains(&self, point: &Point) -> bool {
        // TODO: ewww, make this less gross
        if self.is_diagonal() {
            // find out if it's positive or negative up
            let (m, b) = self.get_mb();
            let y = point.x * m + b;
            return y == point.y
                && y >= min(self.p1.y, self.p2.y)
                && y <= max(self.p1.y, self.p2.y);
        } else {
            return ((point.x >= self.p1.x && point.x <= self.p2.x)
                || (point.x >= self.p2.x && point.x <= self.p1.x))
                && ((point.y >= self.p1.y && point.y <= self.p2.y)
                    || (point.y >= self.p2.y && point.y <= self.p1.y));
        }
    }

    fn diagonal_overlap(l1: &Line, l2: &Line) -> Vec<Point> {
        let mut rv = vec![];
        if l1.is_diagonal() {
            for point in l1.points() {
                if l2.contains(&point) {
                    rv.push(point);
                }
            }
        } else {
            for point in l2.points() {
                if l1.contains(&point) {
                    rv.push(point);
                }
            }
        }
        return rv;
    }

    fn get_mb(&self) -> (i64, i64) {
        let m = if self.p1.x < self.p2.x {
            if self.p1.y < self.p2.y {
                1
            } else {
                -1
            }
        } else {
            if self.p2.y < self.p1.y {
                1
            } else {
                -1
            }
        };
        let b = self.p1.y - (self.p1.x * m);
        return (m, b);
    }

    pub fn points<'a>(&'a self) -> impl std::iter::Iterator<Item = Point> + 'a {
        let mut current = if self.p1.x < self.p2.x {
            self.p1.clone()
        } else {
            self.p2.clone()
        };

        let (m, b) = self.get_mb();
        std::iter::from_fn(move || {
            let rv = current.clone();

            // move position
            current.x += 1;
            current.y = m * current.x + b;

            if self.contains(&rv) {
                return Some(rv.clone());
            }
            return None;
        })
    }

    fn overlap(l1: &Line, l2: &Line) -> Option<Point> {
        let x = if l1.is_vertical() { l1.p1.x } else { l2.p1.x };
        let y = if l1.is_horizontal() { l1.p1.y } else { l2.p1.y };
        let point = Point { x, y };
        if l1.contains(&point) && l2.contains(&point) {
            return Some(point);
        }
        return None;
    }

    fn contiguous_overlap(mut s1: i64, mut e1: i64, mut s2: i64, mut e2: i64) -> Vec<i64> {
        let mut rv = vec![];

        // make sure they go from least to greatest
        if s1 > e1 {
            swap(&mut s1, &mut e1);
        }
        if s2 > e2 {
            swap(&mut s2, &mut e2);
        }

        // make sure s1 starts before s2
        if s1 > s2 {
            swap(&mut s1, &mut s2);
            swap(&mut e1, &mut e2);
        }

        let mut position = s2;
        while position <= e1 && position <= e2 {
            rv.push(position);
            position += 1;
        }

        return rv;
    }
}
