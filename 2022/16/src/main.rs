#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate lazy_static;

use bit_set::BitSet;
use std::collections::{HashMap, HashSet, VecDeque};
mod utils;
use regex::Regex;

lazy_static! {
    static ref BIG_RE: Regex =
        Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9]+); tunnel[s]* lead[s]* to valve[s]* (.*)")
            .unwrap();
}

fn main() {
    let filename = "sample.txt";
    // let filename = "input.txt";

    // 1401 is too low
    part_1(filename);
    // part_2(filename);
}

fn part_1(filename: &str) {
    let (name_map, graph) = get_graph(filename);
    dbg!(&graph);

    let reachable_mapping = get_reachable_mapping(&graph);
    dbg!(&name_map);
    dbg!(&reachable_mapping);

    let mut dead_states = vec![];
    let mut states = vec![State::new(0, graph.len())];

    let max = 20;
    let mut new_states = Vec::new();
    for day in 0..max {
        println!("{} / {}. States: {}", day, max, states.len());

        let num_states = states.len();
        new_states = Vec::with_capacity(num_states);

        for state in &mut states {
            // increment the current flow-rate
            state.total += state.total_flow_rate;
            let mut did_a_thing = false;

            let current_node = graph.get(state.current_position).unwrap();

            // Open the current node
            if !state.open.contains(state.current_position) {
                did_a_thing = true;
                let mut temp = state.clone();
                temp.seen_since_last_open.clear();
                temp.seen_since_last_open.insert(state.current_position);
                temp.open.insert(state.current_position.to_owned());
                temp.total_flow_rate += current_node.flow_rate;
                new_states.push(temp);
            }

            // look for potential moves at the current position
            for neighbor in &current_node.edges {
                // just move there
                if valid_move(state, *neighbor, &reachable_mapping) {
                    did_a_thing = true;
                    let mut temp = state.clone();
                    temp.current_position = *neighbor;
                    temp.seen_since_last_open.insert(neighbor.clone());
                    new_states.push(temp);
                }
            }

            if !did_a_thing {
                // If there's nothing left to do just finish the looping and save the state
                let mut state = state.clone();
                state.total += state.total_flow_rate * (max - (day + 1));
                dead_states.push(state);
            }
        }

        // swap it out
        states = new_states;
    }

    let max = states.iter().max_by_key(|e| e.total).unwrap();
    max.print("Max: ", &name_map);

    let max_dead = dead_states.iter().max_by_key(|e| e.total).unwrap();
    max_dead.print("Max dead: ", &name_map);
}

fn get_reachable_mapping(graph: &Vec<Node>) -> HashMap<(usize, usize), BitSet> {
    let mut mapping: HashMap<(usize, usize), BitSet> = HashMap::new();
    for current in 0..graph.len() {
        for neighbor in &graph[current].edges {
            let reachable = get_reachable(graph, current, *neighbor);
            mapping.insert((current, *neighbor), reachable);
        }
    }
    return mapping;
}

fn get_reachable(graph: &Vec<Node>, current: usize, next: usize) -> BitSet {
    let mut reachable = BitSet::with_capacity(graph.len());
    reachable.insert(next); // TODO: do we need this?

    let mut seen = HashSet::new();
    seen.insert(current);

    let mut stack = VecDeque::new();
    stack.push_front(next);

    loop {
        if stack.is_empty() {
            break;
        }

        let next = stack.pop_front().unwrap();
        for neighbor in &graph[next].edges {
            if !seen.contains(neighbor) {
                seen.insert(*neighbor);
                stack.push_back(*neighbor);
                reachable.insert(*neighbor);
            }
        }
    }

    return reachable;
}

fn valid_move(
    state: &mut State,
    neighbor: usize,
    reachable_mapping: &HashMap<(usize, usize), BitSet>
) -> bool {
    if state.seen_since_last_open.contains(neighbor) {
        return false;
    }

    let reachable = reachable_mapping.get(&(state.current_position, neighbor)).unwrap();
    return reachable.iter().any(|bit| !state.open.contains(bit));
}

fn get_graph(filename: &str) -> (HashMap<usize, String>, Vec<Node>) {
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

    return (reverse_map, graph);
}

#[derive(Debug)]
struct Node {
    name: usize,
    flow_rate: usize,
    edges: Vec<usize>,
}

#[derive(Debug, Clone)]
struct State {
    seen_since_last_open: BitSet,
    open: BitSet,
    current_position: usize,
    total: usize,
    total_flow_rate: usize,
}

impl State {
    pub fn new(current_position: usize, capacity: usize) -> Self {
        Self {
            seen_since_last_open: BitSet::with_capacity(capacity),
            open: BitSet::with_capacity(capacity),
            current_position,
            total: 0,
            total_flow_rate: 0,
        }
    }

    pub fn print(&self, prefix: &str, name_map: &HashMap<usize, String>) {
        println!(
            "{}\n\tCurrent:  {}\n\tTotal: {}\n\tFlow:  {}\n\tOpen:  {}",
            prefix,
            name_map.get(&self.current_position).unwrap(),
            self.total,
            self.total_flow_rate,
            Self::get_bitstring(name_map, &self.open)
        );
    }

    pub fn get_bitstring(name_map: &HashMap<usize, String>, bits: &BitSet) -> String {
        let mut rv = Vec::new();
        for index in 0..bits.capacity() {
            if bits.contains(index) {
                rv.push(name_map.get(&index).unwrap().to_owned());
            }
        }
        return rv.join(", ");
    }
}
