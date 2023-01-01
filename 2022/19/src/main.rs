#![allow(dead_code, unused_variables)]
mod utils;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref BIG_RE: Regex = Regex::new(r"Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();
}

fn main() {
    let contents = include_str!("../sample.txt");
    // let contents = include_str!("../input.txt");

    part_1(contents);
    // part_2(contents);
}

fn part_1(contents: &str) {
    let blueprints = utils::get_generic_groups(contents, parse_blueprint);
    dbg!(blueprints);
    // let max_blueprint = execute_blueprint(&blueprints[0]);
    // dbg!(&max_blueprint);
}

fn execute_blueprint(blueprint: &Blueprint) -> usize {
    let mut states: Vec<State> = vec![State::new(1)];

    for index in 0..24 {
        let mut new_states: Vec<State> = vec![]; // workhorse
        for state in &states {
            execute_possible_states(&mut new_states, &blueprint, state);
        }
        states = new_states;
    }

    return states
        .iter()
        .max_by_key(|state| state.num_geodes)
        .unwrap()
        .num_geodes;
}

fn execute_possible_states(new_states: &mut Vec<State>, blueprint: &Blueprint, state: &State) {
    for robot in &blueprint.robots {
        // if robot
        todo!();
    }
}

fn parse_blueprint(line: &str) -> Blueprint {
    let captures = BIG_RE.captures(line).unwrap();

    let id = get_numb(&captures, 1);

    let robots = [
        Robot::Ore(get_numb(&captures, 2)),
        Robot::Clay(get_numb(&captures, 3)),
        Robot::Obsidian(get_numb(&captures, 4), get_numb(&captures, 5)),
        Robot::Geode(get_numb(&captures, 6), get_numb(&captures, 7)),
    ];
    return Blueprint { id, robots };
}

fn get_numb(captures: &regex::Captures, arg: usize) -> usize {
    return captures
        .get(arg)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    robots: [Robot; 4],
}

#[derive(Debug)]
enum Robot {
    Ore(usize),
    Clay(usize),
    Obsidian(usize, usize),
    Geode(usize, usize),
}

#[derive(Debug, Default)]
struct State {
    ore: usize,
    ore_robots: usize,

    clay: usize,
    clay_robots: usize,

    obsidian_robots: usize,

    geodes: usize,
    num_geodes: usize,
}

impl State {
    pub fn new(ore_robots: usize) -> Self {
        let mut temp = Self::default();
        temp.ore_robots = ore_robots;
        return temp;
    }
}
