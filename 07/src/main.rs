mod utils;
use utils::get_lines;

fn main() {
    // let crabs = get_lines::<i64>("sample.txt");
    let crabs = get_lines::<i64>("input.txt");
    // part_1(crabs);
    part_2(crabs);
}

fn part_1(mut crabs: Vec<i64>) {
    crabs.sort();
    let min = crabs[0];
    let max = crabs[crabs.len() - 1];
    let mut min_location = min;
    let mut min_cost: i64 = get_cost_1(&crabs, min);
    for location in (min - 1)..(max + 1) {
        let cost = get_cost_1(&crabs, location);
        if cost < min_cost {
            min_cost = cost;
            min_location = location;
        }
    }
    dbg!(&crabs, min_location, min_cost);
}

fn get_cost_1(crabs: &Vec<i64>, location: i64) -> i64 {
    let mut cost = 0;
    for crab in crabs {
        cost += (location - crab).abs();
    }
    return cost;
}

fn part_2(mut crabs: Vec<i64>) {
    crabs.sort();
    let min = crabs[0];
    let max = crabs[crabs.len() - 1];
    let mut min_location = min;
    let mut min_cost: i64 = get_cost_2(&crabs, min);
    for location in (min - 1)..(max + 1) {
        let cost = get_cost_2(&crabs, location);
        if cost < min_cost {
            min_cost = cost;
            min_location = location;
        }
    }
    dbg!(&crabs, min_location, min_cost);
    dbg!(calculate_cost(100));
}

fn get_cost_2(crabs: &Vec<i64>, location: i64) -> i64 {
    let mut cost = 0;
    for crab in crabs {
        cost += calculate_cost((location - crab).abs());
    }
    return cost;
}

fn calculate_cost(distance: i64) -> i64 {
    // Thank you, math!
    let total_cost = distance * (distance + 1) / 2;
    return total_cost;
}
