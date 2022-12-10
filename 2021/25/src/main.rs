#![allow(dead_code, unused_variables)]

mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    part_1(filename);
    // part_2(filename);
}

fn part_1(filename: &str) {
    let mut lines = utils::get_fishes(filename);

    let mut count = 0;
    loop {
        dbg!(count);
        print_fishies(&lines);
        // _ = utils::readline();

        let did_move_east = move_fishes(&mut lines, is_good_fishie_east, get_next_pos_east);

        // print_fishies(&lines);
        // _ = utils::readline();

        let did_move_south = move_fishes(&mut lines, is_good_fishie_south, get_next_pos_south);

        count += 1;
        if !did_move_east && !did_move_south {
            break;
        }
    }

    // dbg!(&lines);
    dbg!(count);
}

fn print_fishies(lines: &Vec<Vec<char>>) {
    for line in lines {
        for char in line {
            print!("{}", char);
        }
        println!();
    }
    println!();
}

fn move_fishes(
    fishes: &mut Vec<Vec<char>>,
    is_good_fishie: IsGoodFishieFn,
    get_next_pos: GetNextPosFn,
) -> bool {
    // for all east-facing fishes, find the ones that can move, then
    // move them.
    let mut movable = Vec::new();
    for (row_index, row) in fishes.iter().enumerate() {
        for (col_index, fish) in row.iter().enumerate() {
            if is_good_fishie(fishes[row_index][col_index]) {
                let next_pos = get_next_pos(&fishes, row_index, col_index);
                if fishes[next_pos.0][next_pos.1] == '.' {
                    // dbg!(&next_pos);
                    movable.push((row_index, col_index));
                }
            }
        }
    }

    for (current_row_index, current_col_index) in &movable {
        let (next_row_index, next_col_index) =
            get_next_pos(&fishes, *current_row_index, *current_col_index);
        fishes[next_row_index][next_col_index] = fishes[*current_row_index][*current_col_index];
        fishes[*current_row_index][*current_col_index] = '.';
    }

    return movable.len() > 0;
}

type GetNextPosFn = fn(fishes: &Vec<Vec<char>>, row: usize, col: usize) -> (usize, usize);
type IsGoodFishieFn = fn(fishie: char) -> bool;

fn get_next_pos_east(
    fishes: &Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
) -> (usize, usize) {
    let mut col_index = col_index + 1;
    if col_index >= fishes[0].len() {
        col_index = 0;
    }
    return (row_index, col_index);
}

fn get_next_pos_south(
    fishes: &Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
) -> (usize, usize) {
    let mut row_index = row_index + 1;
    if row_index >= fishes.len() {
        row_index = 0;
    }
    return (row_index, col_index);
}

fn is_good_fishie_east(fishie: char) -> bool {
    return fishie == '>';
}

fn is_good_fishie_south(fishie: char) -> bool {
    return fishie == 'v';
}
