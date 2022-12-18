#![allow(dead_code, unused_variables)]

use std::collections::{HashMap, HashSet};
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let directions = &utils::get_lines::<String>(filename)[0];
    let mut directions = directions.chars().cycle();

    let mut board = Board::new(6);

    let mut count = 0;
    // let max_pieces = 2;
    let max_pieces = 2022;
    for tetromino in get_tetrominos().iter().cycle() {
        count += 1;
        if count > max_pieces {
            break;
        }

        let mut tetromino = board.initialize(tetromino);

        loop {
            // board.print(&tetromino);
            board.push(&mut tetromino, directions.next().unwrap());
            if !board.fall(&mut tetromino) {
                break;
            }
        }

        board.insert(&tetromino);
        // board.print(&tetromino);
    }

    dbg!(board.max_y);
}

fn part_2(filename: &str) {
    let directions = &utils::get_lines::<String>(filename)[0];
    let num_chars = directions.len();
    let mut directions = directions.chars().cycle();

    let mut board = Board::new(6);

    let mut count = 0;
    // let max_pieces: isize = 10;
    let max_pieces: isize = 1_000_000_000_000;
    let tetrominos = get_tetrominos();
    let mut char_index = 0;
    let mut map: HashMap<(Vec<isize>, usize), (isize, isize)> = HashMap::new();
    let mut extra_height = 0;
    let mut did_it = false;
    for (index, tetromino) in tetrominos.iter().cycle().enumerate() {
        // find cycle, use that to save time
        if !did_it && index > 0 && index % tetrominos.len() == 0 {
            let current_top = board.get_top_row();
            let key = (current_top, char_index % num_chars);
            if let Some((old_count, old_max_y)) = map.get(&key) {
                let height_gain_per_it = board.max_y - old_max_y;
                let blocks_per_it = count - old_count;

                let extra_its = (max_pieces - count) / blocks_per_it;
                extra_height = extra_its * height_gain_per_it;
                count += extra_its * blocks_per_it;
                did_it = true;
            } else {
                map.insert(key, (count, board.max_y));
            }
        }

        count += 1;
        if count > max_pieces {
            break;
        }

        let mut tetromino = board.initialize(tetromino);

        loop {
            // board.print(&tetromino);
            board.push(&mut tetromino, directions.next().unwrap());
            char_index += 1;
            if !board.fall(&mut tetromino) {
                break;
            }
        }

        board.insert(&tetromino);
        // board.print(&tetromino);
    }

    dbg!(board.max_y + extra_height);
}

fn get_tetrominos() -> Vec<Tetromino> {
    return vec![
        Tetromino {
            // horizontal line
            points: vec![
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(5, 0),
            ],
        },
        Tetromino {
            // +
            points: vec![
                Point::new(3, 2),
                Point::new(2, 1),
                Point::new(3, 1),
                Point::new(4, 1),
                Point::new(3, 0),
            ],
        },
        Tetromino {
            // reverse L
            points: vec![
                Point::new(4, 2),
                Point::new(4, 1),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
            ],
        },
        Tetromino {
            // vertical line
            points: vec![
                Point::new(2, 3),
                Point::new(2, 2),
                Point::new(2, 1),
                Point::new(2, 0),
            ],
        },
        Tetromino {
            // square
            points: vec![
                Point::new(2, 1),
                Point::new(3, 1),
                Point::new(2, 0),
                Point::new(3, 0),
            ],
        },
    ];
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
struct Tetromino {
    points: Vec<Point>,
}

#[derive(Default)]
struct Board {
    points: HashSet<Point>,
    max_y: isize,
    max_x: isize,
}

impl Board {
    pub fn new(max_x: isize) -> Self {
        let mut temp = Self::default();
        temp.max_x = max_x;
        return temp;
    }

    pub fn get_top_row(&self) -> Vec<isize> {
        let mut top_row = vec![isize::MAX; (self.max_x + 1) as usize];

        // get in relation to max_y
        for point in &self.points {
            let x = point.x as usize;
            let y = (self.max_y - point.y).abs();
            top_row[x] = top_row[x].min(y);
        }

        return top_row;
    }

    pub fn print(&self, tetromino: &Tetromino) {
        for y in (0..(self.max_y + 6)).rev() {
            for x in 0..(self.max_x + 1) {
                if self.points.contains(&Point::new(x, y)) {
                    print!("#");
                } else if tetromino.points.contains(&Point::new(x, y)) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }

    pub fn initialize(&self, tetromino: &Tetromino) -> Tetromino {
        let mut tetromino = tetromino.clone();
        for point in &mut tetromino.points {
            point.y += self.max_y + 3;
        }
        return tetromino;
    }

    pub fn fall(&self, tetromino: &mut Tetromino) -> bool {
        return self.attempt_move(tetromino, Point::new(0, -1));
    }

    pub fn push(&self, tetromino: &mut Tetromino, direction: char) -> bool {
        match direction {
            '>' => {
                return self.attempt_move(tetromino, Point::new(1, 0));
            }
            '<' => {
                return self.attempt_move(tetromino, Point::new(-1, 0));
            }
            _ => panic!(),
        };
    }

    fn attempt_move(&self, tetromino: &mut Tetromino, delta: Point) -> bool {
        let mut new_points = vec![];
        for point in &tetromino.points {
            let point = Point::new(point.x + delta.x, point.y + delta.y);
            if point.x < 0 || point.x > self.max_x || point.y < 0 {
                return false;
            }

            if self.intersects(&point) {
                return false;
            }

            new_points.push(point);
        }

        tetromino.points = new_points;
        return true;
    }

    pub fn insert(&mut self, tetromino: &Tetromino) {
        for point in &tetromino.points {
            if point.y + 1 > self.max_y {
                self.max_y = point.y + 1;
            }
            self.points.insert(point.clone());
        }
    }

    fn intersects(&self, point: &Point) -> bool {
        if self.points.contains(&point) {
            return true;
        }

        return false;
    }
}
