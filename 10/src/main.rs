mod utils;
use utils::get_lines;

fn main() {
    // let lines = get_lines::<String>("sample.txt");
    let lines = get_lines::<String>("input.txt");
    // part_1(lines);
    part_2(lines);
}

fn part_1(lines: Vec<String>) {
    let mut total = 0;
    for line in lines {
        // println!();
        let mut stack = vec![];
        for char in line.chars() {
            // println!("{} + {}", String::from_iter(&stack), char);
            if opening(char) {
                stack.push(char);
            } else {
                let top = stack.pop();
                if !matching(top, char) {
                    let score = corrupted_value(char);
                    dbg!(&score);
                    total += score;
                    break;
                }
            }
        }
    }

    dbg!(total);
}

fn part_2(lines: Vec<String>) {
    let mut totals = vec![];
    for line in lines {
        let mut total = 0;
        println!();
        let mut stack = vec![];
        let mut corrupted = false;
        for char in line.chars() {
            // println!("{} + {}", String::from_iter(&stack), char);
            if opening(char) {
                stack.push(char);
            } else {
                let top = stack.pop();
                if !matching(top, char) {
                    corrupted = true;
                    break;
                }
            }
        }

        if !corrupted {
            println!("{}", line);
            println!("{}", String::from_iter(&stack));
            for char in stack.iter().rev() {
                total *= 5;
                total += incomplete_value(*char);
                dbg!(total);
            }
            if total > 0 {
                dbg!(total);
                totals.push(total);
            }
        }
    }

    totals.sort();
    dbg!(&totals);

    // hacky way of getting middle bit
    dbg!(totals[(&totals.len() - 2) / 2 + 1]);
}

fn opening(char: char) -> bool {
    match char {
        '{' | '(' | '[' | '<' => true,
        _ => false,
    }
}

fn matching(top: Option<char>, char: char) -> bool {
    match top {
        Some('{') => char == '}',
        Some('(') => char == ')',
        Some('<') => char == '>',
        Some('[') => char == ']',
        None => false,
        _ => panic!(),
    }
}

fn corrupted_value(char: char) -> i64 {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn incomplete_value(char: char) -> i64 {
    match char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!(),
    }
}
