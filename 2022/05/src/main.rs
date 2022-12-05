#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

enum ParseStep {
    CRATES,
    NAMES,
    INSTRUCTIONS,
}

fn part_1(filename: &str) {
    solution(filename, execute_instruction);
}

fn part_2(filename: &str) {
    solution(filename, execute_instruction_v2);
}

fn solution(filename: &str, input_fn: fn(&mut Vec<Vec<char>>, &str)) {
    let lines = utils::get_lines::<String>(filename);
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut parse_step = ParseStep::CRATES;

    for line in &lines {
        if line.starts_with(" 1") {
            // next is the names. Right now we ignore that
            parse_step = ParseStep::NAMES;
            continue;
        }

        if line.len() == 0 {
            // next is the instructions
            parse_step = ParseStep::INSTRUCTIONS;
            continue;
        }

        match parse_step {
            ParseStep::CRATES => {
                // TODO
                let crates = parse_crate(line);
                for (index, c) in crates.iter().enumerate() {
                    if stacks.len() < (index + 1) {
                        stacks.push(Vec::new());
                    }

                    if let Some(c) = c {
                        stacks[index].push(*c);
                    }
                }
            }

            ParseStep::NAMES => {
                // skipping
            }

            ParseStep::INSTRUCTIONS => {
                input_fn(&mut stacks, line);
            }
        };
    }

    for stack in stacks {
        print!("{}", stack[0]);
    }
}

fn execute_instruction(stacks: &mut Vec<Vec<char>>, line: &str) {
    let re = regex::Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let caps = re.captures(line).unwrap();
    let (count, start, end): (usize, usize, usize) = parse_caps(&caps);

    for _ in 0..count {
        let char = stacks[start][0];
        stacks[start].remove(0);
        stacks[end].insert(0, char);
    }
}

fn execute_instruction_v2(stacks: &mut Vec<Vec<char>>, line: &str) {
    let re = regex::Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let caps = re.captures(line).unwrap();
    let (count, start, end): (usize, usize, usize) = parse_caps(&caps);

    let mut to_move: Vec<char> = vec![];
    for _ in 0..count {
        let char = stacks[start][0];
        stacks[start].remove(0);
        to_move.push(char);
    }

    for char in to_move.iter().rev() {
        stacks[end].insert(0, *char);
    }
}

fn parse_caps(caps: &regex::Captures) -> (usize, usize, usize) {
    return (
        caps.get(1).unwrap().as_str().parse().unwrap(),
        caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
        caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
    );
}

fn parse_crate(line: &str) -> Vec<Option<char>> {
    let mut rv = Vec::new();
    for chunk in line.chars().collect::<Vec<char>>().chunks(4) {
        if chunk[1] == ' ' {
            rv.push(None);
        } else {
            rv.push(Some(chunk[1]));
        }
    }
    return rv;
}
