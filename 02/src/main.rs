mod utils;
use utils::{get_lines, Direction, Direction::*};

fn main() {
    let lines = get_lines();
    // part_1(lines);
    part_2(lines);
}

fn part_1(lines: Vec<(Direction, i64)>) {
    let mut horizontal = 0;
    let mut vertical = 0;
    for (dir, amount) in lines {
        match dir {
            FORWARD => {
                horizontal += amount;
            }
            UP => {
                vertical -= amount;
            }
            DOWN => {
                vertical += amount;
            }
        };
    }

    println!("Answer: {}", horizontal * vertical);
}

fn part_2(lines: Vec<(Direction, i64)>) {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for (dir, amount) in lines {
        match dir {
            FORWARD => {
                horizontal += amount;
                vertical += aim * amount;
            }
            UP => {
                aim -= amount;
            }
            DOWN => {
                aim += amount;
            }
        };
    }

    println!("Answer: {}", horizontal * vertical);
}
