#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

#[derive(Debug)]
enum Operation {
    Noop,
    Addx(i32),
}

fn part_1(filename: &str) {
    let lines = utils::get_generic(filename, |line: &str| -> Operation {
        if line.contains(" ") {
            let (_, amount_s) = line.split_once(" ").unwrap();
            return Operation::Addx(amount_s.parse().unwrap());
        }
        return Operation::Noop;
    });

    // dbg!(&lines);

    let mut clock = 0;
    let mut total = 0;
    let mut strength = 1;
    let clocks = Vec::from([20, 60, 100, 140, 180, 220]);
    for line in lines {
        clock += 1;

        if clocks.contains(&clock) {
            total += clock * strength;
            // dbg!(clock, strength);
            // println!();
        }

        match line {
            Operation::Noop => {}
            Operation::Addx(amount) => {
                clock += 1;
                if clocks.contains(&clock) {
                    total += clock * strength;
                    // dbg!(clock, strength);
                    // println!();
                }
                strength += amount;
            }
        }
    }

    dbg!(strength, total, clock);
}

fn part_2(filename: &str) {
    let lines = utils::get_generic(filename, |line: &str| -> Operation {
        if line.contains(" ") {
            let (_, amount_s) = line.split_once(" ").unwrap();
            return Operation::Addx(amount_s.parse::<i32>().unwrap());
        }
        return Operation::Noop;
    });

    // dbg!(&lines);

    let mut clock = 0;
    let mut strength = 1;
    let mut output = Vec::new();
    for line in lines {
        clock += 1;
        maybe_push_output(&mut output, clock, strength);

        match line {
            Operation::Noop => {}
            Operation::Addx(amount) => {
                clock += 1;

                maybe_push_output(&mut output, clock, strength);
                // output.push('.');

                strength += amount;
            }
        }
    }

    for chunk in output.chunks(40) {
        for char in chunk {
            print!("{}", char);
        }
        println!();
    }
}

fn maybe_push_output(output: &mut Vec<char>, clock: i32, strength: i32) {
    if is_visible(clock, strength) {
        output.push('#');
    } else {
        output.push('.');
    }

    // if clock < 40 {
    //     for char in output {
    //         print!("{}", char);
    //     }
    //     println!();
    //     println!();
    // }
}

fn is_visible(clock: i32, strength: i32) -> bool {
    let temp_clock = (clock - 1) % 40;
    let rv = if (temp_clock - strength).abs() < 2 {
        true
    } else {
        false
    };

    // dbg!(clock, strength, rv);
    // println!();

    return rv;
}
