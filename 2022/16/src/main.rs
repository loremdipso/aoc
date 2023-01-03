#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate lazy_static;

use bit_set::BitSet;
use std::collections::{HashMap, HashSet, VecDeque};
mod utils;
use itertools::Itertools;
use regex::Regex;

lazy_static! {
    static ref BIG_RE: Regex =
        Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9]+); tunnel[s]* lead[s]* to valve[s]* (.*)")
            .unwrap();
}

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

fn part_1(filename: &str) {
    let (graph, name_map, start_pos) = get_graph(filename);
    let reachable_mapping = get_reachable_mapping(&graph);
    println!("Starting...");

    // Don't even consider nodes that have no flow rate
    let filtered_flow_rates = (0..graph.len())
        .filter(|i| graph[*i].flow_rate > 0)
        .collect();

    let state = State::new(start_pos, graph.len());
    let max = recur(
        &graph,
        &reachable_mapping,
        &filtered_flow_rates,
        state,
        0,
        30,
    );
    dbg!(max);
}

fn part_2(filename: &str) {
    let (graph, name_map, start_pos) = get_graph(filename);
    let reachable_mapping = get_reachable_mapping(&graph);
    println!("Starting...");

    // Don't even consider nodes that have no flow rate
    let filtered_flow_rates = (0..graph.len())
        .filter(|i| graph[*i].flow_rate > 0)
        .collect();

    let state = State::new(start_pos, graph.len());
    let mut maxes = HashMap::new();

    recur_2(
        &graph,
        &reachable_mapping,
        &filtered_flow_rates,
        &state,
        0,
        26,
        &mut maxes,
    );

    let max = maxes
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| human.0.is_disjoint(&elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap();

    dbg!(max);
}

fn recur(
    graph: &Vec<Node>,
    reachable_mapping: &ReachableMapping,
    filtered_flow_rates: &Vec<usize>,
    state: State,
    current_day: usize,
    num_days: usize,
) -> usize {
    if current_day > num_days {
        return state.total;
    }

    let mut max = state.total;

    let num_nodes = graph.len();
    let current_position = state.current_position;
    for target_position in filtered_flow_rates {
        if !state.open.contains(*target_position) {
            let cost = reachable_mapping[current_position * num_nodes + target_position];

            let node = &graph[*target_position];
            let new_day = current_day + cost + 1;
            if node.flow_rate > 0 && new_day < num_days {
                let mut state = state.clone();

                state.current_position = *target_position;
                state.open.insert(*target_position);
                state.total_flow_rate += node.flow_rate;
                // just add the rest of the days
                state.total += node.flow_rate * (num_days - new_day);

                let temp_total = recur(
                    graph,
                    reachable_mapping,
                    filtered_flow_rates,
                    state,
                    new_day,
                    num_days,
                );
                if temp_total > max {
                    max = temp_total;
                }
            }
        }
    }

    return max;
}
fn recur_2(
    graph: &Vec<Node>,
    reachable_mapping: &ReachableMapping,
    filtered_flow_rates: &Vec<usize>,
    state: &State,
    current_day: usize,
    num_days: usize,
    maxes: &mut HashMap<BitSet, usize>,
) -> usize {
    if current_day > num_days {
        return state.total;
    }

    let mut max = state.total;

    let num_nodes = graph.len();
    let current_position = state.current_position;
    for target_position in filtered_flow_rates {
        if !state.open.contains(*target_position) {
            let cost = reachable_mapping[current_position * num_nodes + target_position];

            let node = &graph[*target_position];
            let new_day = current_day + cost + 1;
            if node.flow_rate > 0 && new_day < num_days {
                let mut state = state.clone();

                state.current_position = *target_position;
                state.open.insert(*target_position);
                state.total_flow_rate += node.flow_rate;
                // just add the rest of the days
                state.total += node.flow_rate * (num_days - new_day);

                let temp_total = recur_2(
                    graph,
                    reachable_mapping,
                    filtered_flow_rates,
                    &state,
                    new_day,
                    num_days,
                    maxes,
                );
                if temp_total > max {
                    max = temp_total;
                }

                // TODO: this is slow (10s+). Only calculate this if we need to.
                let entry = maxes.entry(state.open.clone()).or_default();
                *entry = (*entry).max(state.total);
            }
        }
    }

    return max;
}

// { CurrentPosition: { OtherPosition: Distance } }
type ReachableMapping = Vec<usize>;
fn get_reachable_mapping(graph: &Vec<Node>) -> ReachableMapping {
    let mut mapping: ReachableMapping = Vec::new();
    for start in 0..graph.len() {
        for end in 0..graph.len() {
            let cost = bfs(graph, start, end);
            mapping.push(cost);
        }
    }
    return mapping;
}

fn bfs(graph: &Vec<Node>, start: usize, end: usize) -> usize {
    let mut stack = VecDeque::new();
    stack.push_back((0, start));
    loop {
        let (current_cost, position) = stack.pop_front().unwrap();
        if position == end {
            return current_cost;
        }

        for neighbor in &graph[position].edges {
            stack.push_back((current_cost + 1, *neighbor));
        }
    }
}

fn get_graph(filename: &str) -> (Vec<Node>, HashMap<usize, String>, usize) {
    let lines = utils::get_lines::<String>(filename);

    // get the basic mapping from string to usize
    let mut name_map: HashMap<String, usize> = HashMap::new();
    let mut reverse_map: HashMap<usize, String> = HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        let captures = BIG_RE.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str();
        name_map.insert(name.to_owned(), index);
        reverse_map.insert(index, name.to_owned());
    }

    // create nodes with this form
    let mut graph = Vec::new();
    for line in &lines {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let captures = BIG_RE.captures(&line).unwrap();

        let name = captures.get(1).unwrap().as_str();
        let name = name_map.get(name).unwrap();

        let flow_rate = captures.get(2).unwrap().as_str();

        let edges = captures.get(3).unwrap().as_str();
        let edges = edges
            .split(", ")
            .map(|e| *name_map.get(e).unwrap())
            .collect::<Vec<usize>>();

        let node = Node {
            name: *name,
            flow_rate: flow_rate.parse().unwrap(),
            edges,
        };
        graph.push(node);
    }

    return (graph, reverse_map, *name_map.get("AA").unwrap());
}

#[derive(Debug)]
struct Node {
    name: usize,
    flow_rate: usize,
    edges: Vec<usize>,
}

#[derive(Debug, Clone)]
struct State {
    open: BitSet,
    current_position: usize,
    total: usize,
    total_flow_rate: usize,
}

impl State {
    pub fn new(current_position: usize, capacity: usize) -> Self {
        Self {
            open: BitSet::with_capacity(capacity),
            current_position,
            total: 0,
            total_flow_rate: 0,
        }
    }
}
