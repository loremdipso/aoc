mod utils;
use core::num;
use std::collections::HashMap;

use utils::get_lines;

fn main() {
    let lines = get_lines::<i64>("input.txt");
    // let lines = get_lines::<i64>("sample.txt");
    // part_1(lines);
    part_2(lines);
}

fn part_1(mut fishes: Vec<i64>) {
    for day in (0..80) {
        simulate(&mut fishes);
    }
    dbg!(&fishes);
    dbg!(&fishes.len());
}

fn part_2(mut fishes: Vec<i64>) {
    let mut fish_groups: HashMap<i64, i64> = HashMap::new();
    for fish in fishes {
        add_fish(&mut fish_groups, fish, 1);
    }

    for day in (0..256) {
        simulate_fast(&mut fish_groups);
    }

    dbg!(&fish_groups);
    let mut total: i64 = 0;
    for (life, count) in fish_groups.iter() {
        total += count;
    }
    dbg!(total);
}

fn simulate(fishes: &mut Vec<i64>) {
    let mut num_to_add = 0;
    for fish in fishes.iter_mut() {
        if *fish == 0 {
            num_to_add += 1;
            *fish = 6;
        } else {
            *fish = *fish - 1;
        }
    }

    for i in (0..num_to_add) {
        fishes.push(8);
    }
}

fn simulate_fast(fish_groups: &mut HashMap<i64, i64>) {
    let mut num_to_add = 0;
    let clone = fish_groups.clone();
    fish_groups.clear();
    for (life, count) in clone.iter() {
        if *life == 0 {
            num_to_add = *count;
            add_fish(fish_groups, 6, *count)
        } else {
            add_fish(fish_groups, *life - 1, *count);
        }
    }

    // for group in fish_groups.iter_mut() {
    //     if group.life == 0 {
    //         num_to_add += group.count;
    //         group.life = 6;
    //     } else {
    //         group.life -= 1;
    //     }
    // }

    if num_to_add > 0 {
        add_fish(fish_groups, 8, num_to_add);
    }
}

fn add_fish(fish_groups: &mut HashMap<i64, i64>, life: i64, count: i64) {
    *fish_groups.entry(life).or_insert(0) += count;
}
