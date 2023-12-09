#![allow(dead_code, unused_variables)]
mod utils;
use std::collections::{HashMap, HashSet};

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

type Data = Vec<Position>;
type Position = (isize, isize);

fn part_1(contents: &str) {
    let data = utils::get_generic(contents, parse_line);
    let mut positions: Data = Vec::new();

    let num_lines = data.len();
    let num_chars = data[0].len();
    for line_index in 0..num_lines {
        for char_index in 0..num_chars {
            if data[line_index][char_index] == '#' {
                positions.push((line_index as isize, char_index as isize));
            }
        }
    }

    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        // print_positions(&positions, None);
        simulate(&mut positions, &directions);
        // print_positions(&positions, None);

        let direction = directions.remove(0);
        directions.push(direction);
        println!();
    }

    print_positions(&positions, None);
    dbg!(get_min_rect(&positions));
}

fn part_2(contents: &str) {
    let data = utils::get_generic(contents, parse_line);
    let mut positions: Data = Vec::new();

    let num_lines = data.len();
    let num_chars = data[0].len();
    for line_index in 0..num_lines {
        for char_index in 0..num_chars {
            if data[line_index][char_index] == '#' {
                positions.push((line_index as isize, char_index as isize));
            }
        }
    }

    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut count = 1;
    while (simulate(&mut positions, &directions)) {
        let direction = directions.remove(0);
        directions.push(direction);
        count += 1;
        dbg!(count);
    }

    // print_positions(&positions, None);
    dbg!(count);
}

fn simulate(data: &mut Data, directions: &Vec<Direction>) -> bool {
    let mut proposals: Vec<(usize, Position)> = Vec::new();
    let mut proposed_directions: HashMap<Position, Direction> = HashMap::new();
    let mut counts: HashMap<Position, isize> = HashMap::new();

    for (index, old_position) in data.iter().enumerate() {
        let valid_directions = get_valid_directions(data, old_position);
        if let Some((new_position, direction)) =
            get_new_position(directions, &valid_directions, old_position)
        {
            let entry = counts.entry(new_position).or_default();
            *entry += 1;
            proposals.push((index, new_position));
            proposed_directions.insert(*old_position, direction);
        }
    }

    // print_positions(&data, Some(&proposed_directions));

    let mut moved = false;
    for (index, new_position) in &proposals {
        if counts[new_position] == 1 {
            data[*index] = new_position.clone();
            moved = true;
        }
    }

    // 1115 is too low
    return moved;
}

fn get_new_position(
    directions: &Vec<Direction>,
    valid_directions: &HashSet<Direction>,
    old_position: &Position,
) -> Option<(Position, Direction)> {
    for direction in directions {
        if valid_directions.contains(direction) {
            return Some((direction.move_position(old_position), *direction));
        }
    }
    return None;
}

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn move_position(&self, old_position: &Position) -> Position {
        match self {
            Direction::North => (old_position.0 - 1, old_position.1),
            Direction::South => (old_position.0 + 1, old_position.1),
            Direction::East => (old_position.0, old_position.1 + 1),
            Direction::West => (old_position.0, old_position.1 - 1),
        }
    }

    pub fn get_char(&self) -> char {
        match self {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
        }
    }
}

fn get_valid_directions(data: &Data, old_position: &Position) -> HashSet<Direction> {
    let mut valid_positions: HashSet<Direction> = HashSet::new();
    valid_positions.insert(Direction::North);
    valid_positions.insert(Direction::South);
    valid_positions.insert(Direction::West);
    valid_positions.insert(Direction::East);

    for new_position in data {
        let delta_x = (new_position.0 - old_position.0).abs();
        let delta_y = (new_position.1 - old_position.1).abs();
        if delta_x <= 1 && delta_y <= 1 && (delta_x > 0 || delta_y > 0) {
            let (line_index, char_index) = *new_position;
            if line_index < old_position.0 {
                valid_positions.remove(&Direction::North);
            }
            if line_index > old_position.0 {
                valid_positions.remove(&Direction::South);
            }
            if char_index < old_position.1 {
                valid_positions.remove(&Direction::West);
            }
            if char_index > old_position.1 {
                valid_positions.remove(&Direction::East);
            }
        }
    }

    // If we can move in any direction we don't need to move at all
    if valid_positions.len() == 4 {
        valid_positions.clear();
    }

    return valid_positions;
}

fn parse_line(line: &str) -> Vec<char> {
    return line.chars().collect();
}

fn print_positions(positions: &Vec<Position>, directions: Option<&HashMap<Position, Direction>>) {
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    for (y, x) in positions {
        if x < &min_x {
            min_x = *x;
        }
        if x > &max_x {
            max_x = *x;
        }
        if y < &min_y {
            min_y = *y;
        }
        if y > &max_y {
            max_y = *y;
        }
    }

    let mut buffer: Vec<Vec<char>> = Vec::new();
    for _ in 0..=max_y - min_y {
        let mut line = Vec::new();
        for _ in 0..=max_x - min_x {
            line.push('.');
        }
        buffer.push(line);
    }

    for (y, x) in positions {
        let new_y = max_y - y;
        let new_x = max_x - x;
        buffer[new_y as usize][new_x as usize] = '#';
        if let Some(directions) = directions {
            if let Some(direction) = directions.get(&(*y, *x)) {
                buffer[new_y as usize][new_x as usize] = direction.get_char();
            }
        }
    }

    // Reversing because I'm not smart enough to do this properly
    let mut count = 0;
    for line in buffer.iter().rev() {
        for char in line.iter().rev() {
            print!("{char}");
        }
        println!();
    }
    println!();
}

fn get_min_rect(positions: &Vec<Position>) -> isize {
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    for (y, x) in positions {
        if x < &min_x {
            min_x = *x;
        }
        if x > &max_x {
            max_x = *x;
        }
        if y < &min_y {
            min_y = *y;
        }
        if y > &max_y {
            max_y = *y;
        }
    }

    let num_squares = (max_y - min_y + 1) * (max_x - min_x + 1);
    return num_squares - positions.len() as isize;
}
