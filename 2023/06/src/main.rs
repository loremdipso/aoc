#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let lines = utils::get_generic(contents, parse_line);
    let mut total = 1;
    for i in 0..lines[0].len() {
        let value = calculate_game(lines[0][i], lines[1][i]);
        dbg!(value);
        total *= value;
    }
    dbg!(&total);
}

fn part_2(contents: &str) {
    let values = utils::get_generic(contents, parse_line_2);
    let value = calculate_game(values[0], values[1]);
    dbg!(value);
}

fn parse_line(line: &str) -> Vec<isize> {
    let (_, line) = line.split_once(':').unwrap();
    return line
        .split(' ')
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|l| l.parse::<isize>().unwrap())
        .collect();
}

fn parse_line_2(line: &str) -> isize {
    let (_, line) = line.split_once(':').unwrap();
    return line.replace(" ", "").trim().parse::<isize>().unwrap();
}

fn calculate_game(time: isize, distance: isize) -> usize {
    let mut num_wins = 0;
    for velocity in 0..time {
        let position = velocity * (time - velocity);
        if position > distance {
            num_wins += 1;
        }
    }
    return num_wins;
}
