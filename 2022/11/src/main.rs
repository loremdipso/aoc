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
    Add,
    Mult,
    Square,
}

#[derive(Debug)]
struct Monkey {
    name: usize,
    items: Vec<isize>,
    total: usize,
    operation: Operation,
    worry_arg: Option<isize>,
    test_arg: isize,
    true_value: usize,
    false_value: usize,
}

impl Monkey {
    pub fn worry(old: isize, operation: &Operation, worry_arg: &Option<isize>) -> isize {
        match operation {
            Operation::Add => old + worry_arg.unwrap(),
            Operation::Mult => old * worry_arg.unwrap(),
            Operation::Square => old * old,
        }
    }

    pub fn worry_2(
        old: isize,
        operation: &Operation,
        worry_arg: &Option<isize>,
        lcd: isize,
    ) -> isize {
        let old = old % lcd;
        match operation {
            Operation::Add => old + worry_arg.unwrap(),
            Operation::Mult => old * worry_arg.unwrap(),
            Operation::Square => old * old,
        }
    }

    pub fn test(value: isize, test_arg: isize) -> bool {
        return (value % test_arg) == 0;
    }
}

fn part_1(filename: &str) {
    let mut monkeys = get_monkeys(filename);

    for _round in 0..20 {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            let mut changes = Vec::new();
            monkey.total += monkey.items.len();
            for item in monkey.items.drain(..) {
                let value = Monkey::worry(item, &monkey.operation, &monkey.worry_arg) / 3;
                if Monkey::test(value, monkey.test_arg) {
                    changes.push((monkey.true_value, value));
                } else {
                    changes.push((monkey.false_value, value));
                }
            }

            for change in changes {
                monkeys[change.0].items.push(change.1);
            }
        }
    }

    let mut totals: Vec<usize> = monkeys.iter().map(|m| m.total).collect();
    totals.sort();
    dbg!(&totals);
    dbg!(totals[totals.len() - 1] * totals[totals.len() - 2]);
}

fn part_2(filename: &str) {
    let mut monkeys = get_monkeys(filename);

    let mut lcd = 1;
    for monkey in &monkeys {
        lcd *= monkey.test_arg;
    }

    for _round in 0..10_000 {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            let mut changes = Vec::new();
            monkey.total += monkey.items.len();
            for item in monkey.items.drain(..) {
                let value = Monkey::worry_2(item, &monkey.operation, &monkey.worry_arg, lcd);
                if Monkey::test(value, monkey.test_arg) {
                    changes.push((monkey.true_value, value));
                } else {
                    changes.push((monkey.false_value, value));
                }
            }

            for change in changes {
                monkeys[change.0].items.push(change.1);
            }
        }
    }

    let mut totals: Vec<usize> = monkeys.iter().map(|m| m.total).collect();
    totals.sort();
    dbg!(&totals);
    dbg!(totals[totals.len() - 1] * totals[totals.len() - 2]);
}

fn get_monkeys(filename: &str) -> Vec<Monkey> {
    let lines = utils::get_groups::<String>(filename);
    let monkeys: Vec<Monkey> = lines
        .iter()
        .map(|lines| -> Monkey {
            return Monkey {
                name: get_name(&lines[0]),
                total: 0,
                items: get_items(&lines[1]),
                operation: get_operation(&lines[2]),
                worry_arg: get_worry_arg(&lines[2]),
                test_arg: get_test_arg(&lines[3]),
                true_value: get_test_value(&lines[4]),
                false_value: get_test_value(&lines[5]),
            };
        })
        .collect();
    return monkeys;
}

/*
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
*/

fn get_test_value(line: &str) -> usize {
    let arg = line.trim().split(' ').nth(5).unwrap();
    return arg.parse().unwrap();
}

fn get_test_arg(line: &str) -> isize {
    let arg = line.trim().split(' ').nth(3).unwrap();
    return arg.parse().unwrap();
}

fn get_worry_arg(line: &str) -> Option<isize> {
    let arg = line.trim().split(' ').nth(5).unwrap();
    if arg == "old" {
        return None;
    }
    return Some(arg.parse().unwrap());
}

fn get_operation(line: &str) -> Operation {
    let pieces: Vec<&str> = line.trim().split(' ').collect();
    if pieces[5] == "old" {
        return Operation::Square;
    }

    return match pieces[4] {
        "+" => Operation::Add,
        "*" => Operation::Mult,
        _ => panic!(),
    };
}

fn get_items(line: &str) -> Vec<isize> {
    let temp = line.split_once(": ").unwrap().1;
    return temp.split(", ").map(|item| item.parse().unwrap()).collect();
}

fn get_name(line: &str) -> usize {
    let temp = line.split_once(" ").unwrap().1;
    let temp = &temp[..temp.len() - 1];
    return temp.parse().unwrap();
}
