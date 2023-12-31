#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let grids = contents
        .split("\n\n")
        .map(|chunk| utils::get_grid(chunk))
        .collect::<Vec<_>>();

    let mut total = 0;
    for grid in &grids {
        total += calculate_reflections(grid) * 100 + calculate_reflections(&rotate_grid(grid));
    }
    dbg!(total);
}

fn part_2(contents: &str) {
    let grids = contents
        .split("\n\n")
        .map(|chunk| utils::get_grid(chunk))
        .collect::<Vec<_>>();

    let mut total = 0;
    for grid in &grids {
        total += calculate_reflections_2(grid) * 100 + calculate_reflections_2(&rotate_grid(grid));
    }
    dbg!(total);
}

fn calculate_reflections(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    let num_lines = grid.len();
    for line_index in 0..num_lines {
        if matches_across_reflection(grid, line_index) {
            total += line_index;
        }
    }
    return total;
}

fn calculate_reflections_2(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    let num_lines = grid.len();
    for line_index in 0..num_lines {
        if matches_across_reflection_2(grid, line_index) {
            total += line_index;
        }
    }
    return total;
}

fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_vec = vec![vec!['z'; grid.len()]; grid[0].len()];
    for (line_index, line) in grid.iter().enumerate() {
        for (char_index, char) in line.iter().enumerate() {
            new_vec[char_index][line_index] = *char;
        }
    }
    return new_vec;
}

fn matches_across_reflection(grid: &Vec<Vec<char>>, line_index: usize) -> bool {
    let mut visited = false;
    let num_lines = grid.len();
    for delta in 1..num_lines {
        if delta > line_index || delta + line_index > num_lines {
            break;
        }

        let left = line_index - delta;
        let right = line_index + delta - 1;
        if grid[left] != grid[right] {
            return false;
        }
        visited = true;
    }
    return visited;
}

fn matches_across_reflection_2(grid: &Vec<Vec<char>>, line_index: usize) -> bool {
    let mut diff_chars = 0;
    let num_lines = grid.len();
    for delta in 1..num_lines {
        if delta > line_index || delta + line_index > num_lines {
            break;
        }

        let left = line_index - delta;
        let right = line_index + delta - 1;
        for i in 0..grid[0].len() {
            if grid[left][i] != grid[right][i] {
                diff_chars += 1;
            }
        }
        if diff_chars > 1 {
            return false;
        }
    }
    return diff_chars == 1;
}
