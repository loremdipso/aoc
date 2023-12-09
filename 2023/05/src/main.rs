#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    // let filename = include_str!("../sample.txt");
    let filename = include_str!("../input.txt");

    // part_1(filename);
    part_2(filename);
}

fn part_1(contents: &str) {
    let (seeds, maps) = utils::get_maps(contents);
    let mut min_location = isize::MAX;
    for seed in &seeds {
        let location = find_location(&maps, "seed", "location", *seed);
        if location < min_location {
            min_location = location;
        }
    }
    dbg!(min_location);
    dbg!(&maps);
}

fn part_2(contents: &str) {
    let (mut seeds, maps) = utils::get_maps(contents);
    let mut min_location = isize::MAX;
    // let direction = find_direction(&maps);
    for range in seeds.chunks_mut(2) {
        for seed in range[0]..range[0] + range[1] {
            let location = find_location(&maps, "seed", "location", seed);
            if location < min_location {
                min_location = location;
            }
        }
    }
    dbg!(min_location);
    // dbg!(&maps);
}

fn find_location(
    maps: &utils::Map,
    start_prop: &str,
    target_prop: &str,
    current_location: isize,
) -> isize {
    for (end_prop, mappings) in maps.get(start_prop).unwrap() {
        let new_location = get_location(mappings, current_location);
        if end_prop == target_prop {
            return new_location;
        } else {
            return find_location(maps, end_prop, target_prop, new_location);
        }
    }
    return isize::MAX;
}

fn get_location(mappings: &Vec<(isize, isize, isize)>, current_location: isize) -> isize {
    for (to, from, length) in mappings {
        if current_location >= *from && current_location < from + length {
            let temp = current_location + (to - from);
            return temp;
        }
    }
    return current_location;
}
