#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let lines = utils::get_generic(contents, |l| {
        l.split(" ")
            .map(|p| p.parse::<isize>().unwrap().clone())
            .collect::<Vec<isize>>()
    });
    let total: isize = lines.iter().map(|l| predict_next(l)).sum();

    dbg!(&total);
}

fn predict_next(numbers: &Vec<isize>) -> isize {
    let mut lines = vec![numbers.clone()];
    loop {
        if lines.last().unwrap().iter().all(|n| *n == 0) {
            break;
        }

        let mut new_line = Vec::new();
        for pair in lines.last().unwrap().windows(2) {
            new_line.push(pair[1] - pair[0]);
        }
        lines.push(new_line);
    }

    let mut last_number = 0;
    for index in (0..lines.len() - 1).rev() {
        last_number = lines[index].last().unwrap() + last_number;
    }
    dbg!(last_number);
    return last_number;
}

fn part_2(contents: &str) {
    let lines = utils::get_generic(contents, |l| {
        l.split(" ")
            .map(|p| p.parse::<isize>().unwrap().clone())
            .collect::<Vec<isize>>()
    });
    let total: isize = lines.iter().map(|l| predict_first(l)).sum();

    dbg!(&total);
}

fn predict_first(numbers: &Vec<isize>) -> isize {
    let mut lines = vec![numbers.clone()];
    loop {
        if lines.last().unwrap().iter().all(|n| *n == 0) {
            break;
        }

        let mut new_line = Vec::new();
        for pair in lines.last().unwrap().windows(2) {
            new_line.push(pair[1] - pair[0]);
        }
        lines.push(new_line);
    }

    let mut first_number = 0;
    for index in (0..lines.len() - 1).rev() {
        first_number = lines[index].first().unwrap() - first_number;
    }
    return first_number;
}
