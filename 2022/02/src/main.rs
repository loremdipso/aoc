#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let lines = utils::get_lines::<String>(filename);
    let mut score = 0;
    for line in &lines {
        let (theirs, mine) = line.split_once(" ").unwrap();
        let (theirs, mine) = (convert(theirs), convert(mine));
        score += raw_value(mine);
        score += win_value(mine, theirs);
    }
    dbg!(&lines);
    dbg!(score);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Value {
    ROCK,
    PAPER,
    SCISSORS,
}

fn convert(s: &str) -> Value {
    if s == "X" || s == "A" {
        return Value::ROCK;
    } else if s == "Y" || s == "B" {
        return Value::PAPER;
    } else if s == "Z" || s == "C" {
        return Value::SCISSORS;
    }
    panic!();
}

fn win_value(mine: Value, theirs: Value) -> i32 {
    if mine == theirs {
        return 3;
    } else if win(mine, theirs) {
        return 6;
    }
    return 0;
}

fn win(mine: Value, theirs: Value) -> bool {
    if mine == Value::ROCK && theirs == Value::SCISSORS {
        return true;
    }

    if mine == Value::PAPER && theirs == Value::ROCK {
        return true;
    }

    if mine == Value::SCISSORS && theirs == Value::PAPER {
        return true;
    }

    return false;
}

fn raw_value(mine: Value) -> i32 {
    match mine {
        Value::ROCK => 1,
        Value::PAPER => 2,
        Value::SCISSORS => 3,
    }
}

fn part_2(filename: &str) {
    let lines = utils::get_lines::<String>(filename);
    let mut score = 0;
    for line in &lines {
        let (theirs, mine) = line.split_once(" ").unwrap();
        let (theirs, mine) = (convert(theirs), convert(mine));

        let mine = match mine {
            Value::ROCK => {
                // must lose
                get_expected_value(theirs, 0)
            }
            Value::PAPER => {
                // must draw
                get_expected_value(theirs, 3)
            }
            Value::SCISSORS => {
                // must win
                get_expected_value(theirs, 6)
            }
        };

        score += raw_value(mine);
        score += win_value(mine, theirs);
    }
    dbg!(&lines);
    dbg!(score);
}

fn get_expected_value(theirs: Value, value: i32) -> Value {
    if win_value(Value::ROCK, theirs) == value {
        return Value::ROCK;
    }

    if win_value(Value::PAPER, theirs) == value {
        return Value::PAPER;
    }

    if win_value(Value::SCISSORS, theirs) == value {
        return Value::SCISSORS;
    }

    panic!();
}
