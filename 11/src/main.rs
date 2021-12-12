mod utils;
use utils::get_lines;

fn main() {
    //let lines = get_lines("sample.txt");
    let lines = get_lines("input.txt");
    //part_1(lines);
    part_2(lines);
}

fn part_1(mut lines: Vec<Vec<i64>>) {
    let mut count = 0;
    for i in 0..100 {
        count += simulate(&mut lines);
        println!("After step {}: {}", i + 1, count);
        //puts(&lines);
        //dbg!(count);
    }
    dbg!(count);
}

fn part_2(mut lines: Vec<Vec<i64>>) {
    let mut iteration = 0;
    let target = lines.len() * lines[0].len();
    loop {
        iteration += 1;
        let count = simulate(&mut lines);
        if count == target as i64 {
            break;
        }
    }
    dbg!(iteration);
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
        let mut count_this_round = 0;
        for y in 0..lines.len() {
            for x in 0..width {
                if lines[y][x] > 9 {
                    count_this_round += 1;
                    //count += 1;
                    lines[y][x] = 0;
                    did_it = true;
                    flash_neighbors(lines, x, y);
                }
            }
        }
        if !did_it {
            break;
        }
        //dbg!(count_this_round);
    }

    for value in lines.iter_mut().flatten() {
        if *value == 0 {
            count += 1;
        }
    }

    //puts(lines);
    return count;
}

fn puts(lines: &Vec<Vec<i64>>) {
    let width = lines[0].len();
    for y in 0..lines.len() {
        for x in 0..width {
            if lines[y][x] == 0 {
                print!("0");
            } else {
                print!(" ");
            }
            //print!("{}", lines[y][x].min(9));
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
