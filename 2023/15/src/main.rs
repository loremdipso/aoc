#![allow(dead_code, unused_variables)]

mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let lines = utils::get_lines::<String>(contents)[0]
        .split(",")
        .map(|e| e.into())
        .collect::<Vec<String>>();

    let total = lines.iter().map(|e| calc_value(e)).sum::<usize>();
    dbg!(total);
    utils::copy_to_clipboard(format!("{total}"));
}

fn calc_value(line: &str) -> usize {
    let mut value = 0;
    for char in line.chars() {
        value += char as usize;
        value *= 17;
        value %= 256;
    }
    return value;
}

fn part_2(contents: &str) {
    let lines = utils::get_lines::<String>(contents)[0]
        .split(",")
        .map(|e| e.into())
        .collect::<Vec<String>>();

    let mut boxes: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];
    for line in &lines {
        if let Some((first, second)) = line.split_once('-') {
            let label = calc_value(&first);
            if let Some(position) = boxes[label].iter().position(|(label, _)| label == first) {
                boxes[label].remove(position);
            }
        } else if let Some((first, second)) = line.split_once('=') {
            let label = calc_value(&first);
            let value = second.parse().unwrap();

            if let Some(position) = boxes[label].iter().position(|(label, _)| label == first) {
                boxes[label][position] = (first.into(), value);
            } else {
                boxes[label].push((first.into(), value));
            }
        } else {
            panic!();
        }

        // println!("{line}");
        // print_boxes(&boxes);
        // println!();
    }

    let total: usize = boxes
        .iter()
        .enumerate()
        .map(|(box_index, items)| {
            items
                .iter()
                .enumerate()
                .map(|(item_index, (_, value))| {
                    return (box_index + 1) * (item_index + 1) * value;
                })
                .sum::<usize>()
        })
        .sum();

    dbg!(total);
    utils::copy_to_clipboard(format!("{total}"));
}

fn print_boxes(boxes: &Vec<Vec<(String, usize)>>) {
    for (i, items) in boxes.iter().enumerate() {
        if items.len() > 0 {
            print!("Box {i}: ");
            for (label, value) in items {
                print!("[{label} {value}] ");
            }
            println!();
        }
    }
}
