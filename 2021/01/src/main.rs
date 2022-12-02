mod utils;
use utils::get_lines;

fn main() {
    let lines = get_lines::<i64>();
    //part_1(lines);
    part_2(lines);
}

fn part_1(lines: Vec<i64>) {
    let mut last = -1;
    let mut count = 0;
    for line in lines {
        if last >= 0 && line > last {
            count += 1;
        }
        last = line;
    }
    println!("Answer: {}", count);
}

fn part_2(lines: Vec<i64>) {
    let mut count = 0;
    for (index, _) in lines.iter().enumerate() {
        if index >= 3 {
            let range_a = lines[index - 1] + lines[index - 2] + lines[index - 3];
            let range_b = lines[index] + lines[index - 1] + lines[index - 2];
            if range_b > range_a {
                count += 1;
            }
        }
    }
    println!("Answer: {}", count);
}
