#![allow(dead_code, unused_variables)]
mod utils;
use std::collections::HashSet;

fn main() {
    let filename = include_str!("../sample.txt");
    // let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let mut grid = utils::get_grid(contents);
    roll_north(&mut grid);
    utils::print_grid(&grid);
    dbg!(calculate_load(&grid));
}

// Idea: this will loop. When it does skip all that processing.
fn part_2(contents: &str) {
    let mut grid = utils::get_grid(contents);
    let mut seen = HashSet::new();

    let num_iterations = 1_000_000_000;
    let mut iter = 0;
    let print = false;
    let mut skipped = false;
    while iter < num_iterations {
        if !skipped && !seen.insert((iter % 4, grid.clone())) {
            skipped = true;
            println!("Loop!");
            let step = seen.len();
            while iter + step < num_iterations {
                iter += step;
            }
            continue;
        }

        if print {
            utils::print_grid(&grid);
        }

        match iter % 4 {
            0 => {
                roll_north(&mut grid);
                if print {
                    println!("(North)");
                    utils::print_grid(&grid);
                }
            }

            1 => {
                roll_west(&mut grid);
                if print {
                    println!("(West)");
                    utils::print_grid(&grid);
                }
            }

            2 => {
                roll_south(&mut grid);
                if print {
                    println!("(South)");
                    utils::print_grid(&grid);
                }
            }

            3 => {
                roll_east(&mut grid);
                if print {
                    println!("(East)");
                    utils::print_grid(&grid);
                }
            }

            _ => panic!(),
        };

        iter += 1;
        dbg!(iter % 4, calculate_load(&grid));
    }

    dbg!(calculate_load(&grid));
}

fn roll_north(grid: &mut Vec<Vec<char>>) {
    let num_lines = grid.len();
    let num_chars = grid[0].len();
    roll_generic(grid, 0..num_lines, 0..num_chars, (-1, 0));
}

fn roll_south(grid: &mut Vec<Vec<char>>) {
    let num_lines = grid.len();
    let num_chars = grid[0].len();
    roll_generic(grid, (0..num_lines).rev(), 0..num_chars, (1, 0));
}

fn roll_east(grid: &mut Vec<Vec<char>>) {
    let num_lines = grid.len();
    let num_chars = grid[0].len();
    roll_generic(grid, 0..num_lines, (0..num_chars).rev(), (0, 1));
}

fn roll_west(grid: &mut Vec<Vec<char>>) {
    let num_lines = grid.len();
    let num_chars = grid[0].len();
    roll_generic(grid, 0..num_lines, 0..num_chars, (0, -1));
}

fn roll_generic(
    grid: &mut Vec<Vec<char>>,
    line_range: impl IntoIterator<Item = usize> + Clone,
    char_range: impl IntoIterator<Item = usize> + Clone,
    delta: (isize, isize),
) {
    let num_lines = grid.len();
    let num_chars = grid[0].len();
    for line_index in line_range {
        for char_index in char_range.clone() {
            if grid[line_index][char_index] == 'O' {
                let mut old_line_index = line_index as isize;
                let mut old_char_index = char_index as isize;
                loop {
                    let new_line_index = old_line_index + delta.0;
                    let new_char_index = old_char_index + delta.1;
                    if new_line_index < 0
                        || new_line_index >= num_lines as isize
                        || new_char_index < 0
                        || new_char_index >= num_chars as isize
                    {
                        break;
                    }

                    match grid[new_line_index as usize][new_char_index as usize] {
                        '.' => {
                            grid[new_line_index as usize][new_char_index as usize] = 'O';
                            grid[old_line_index as usize][old_char_index as usize] = '.';
                        }
                        _ => break,
                    };

                    old_line_index = new_line_index;
                    old_char_index = new_char_index;
                }
            }
        }
    }
}

fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
    let num_lines = grid.len();
    let num_chars = grid[0].len();
    let mut total = 0;
    for line_index in 0..num_lines {
        for char_index in 0..num_chars {
            if grid[line_index][char_index] == 'O' {
                total += num_lines - line_index;
            }
        }
    }
    return total;
}
