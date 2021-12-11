mod utils;
use utils::get_lines;

fn main() {
    let lines = get_lines("sample.txt");
    // let lines = get_lines("input.txt");
    part_1(lines);
}

fn part_1(mut lines: Vec<Vec<i64>>) {
    let mut count = 9;
    for i in 0..100 {
        count += simulate(&mut lines);
        println!("After step {}", i + 1);
        // puts(&lines);
        dbg!(count);
    }
    dbg!(count);
}

fn simulate(lines: &mut Vec<Vec<i64>>) -> i64 {
    let mut count = 0;

    // increase all by 1
    for value in lines.iter_mut().flatten() {
        *value += 1;
    }

    // puts(lines);
    let width = lines[0].len();
    loop {
        let mut did_it = false;
        for y in 0..lines.len() {
            for x in 0..width {
                if lines[y][x] > 9 {
                    count += 1;
                    lines[y][x] = 0;
                    did_it = true;
                    flash_neighbors(lines, x, y);
                }
            }
        }
        if !did_it {
            break;
        }
        // puts(lines);
    }

    return count;
}

fn puts(lines: &Vec<Vec<i64>>) {
    let width = lines[0].len();
    for y in 0..lines.len() {
        for x in 0..width {
            print!("{}", lines[y][x].min(9));
        }
        println!();
    }
    println!();
}

fn flash_neighbors(lines: &mut Vec<Vec<i64>>, x: usize, y: usize) {
    let width = lines[0].len();

    for xd in -1..2 {
        for yd in -1..2 {
            if xd == 0 && yd == 0 {
                continue;
            }
            let x = x as i64 + xd;
            let y = y as i64 + yd;
            if x >= 0 && x < (width as i64) && y >= 0 && y < lines.len() as i64 {
                if lines[y as usize][x as usize] > 0 {
                    lines[y as usize][x as usize] += 1;
                }
            }
        }
    }
}
