#![allow(dead_code, unused_variables)]
mod blizzard;
mod direction;
mod map;
mod state;
mod utils;
use crate::state::{SimpleState, State};
use std::collections::{BinaryHeap, HashSet};

pub type Position = (usize, usize);

fn main() {
    let filename = include_str!("../sample.txt");
    // let filename = include_str!("../input.txt");

    part_1(filename);
    // part_2(filename);
}

fn part_1(contents: &str) {
    let (map, mut blizzards, start_position) = blizzard::get_blizzards(contents);

    let all_blizzards = map.get_all_blizzards(&mut blizzards);
    // At this point we have the maze. Let's just do a simple dijkstra and solve
    // this boyo.
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut seen_states: HashSet<SimpleState> = HashSet::new();
    heap.push(State {
        position: start_position,
        iteration: 0,
        steps: vec![start_position],
    });

    let mut closest = isize::MAX;
    for _ in 0..100_000 {
        let mut state = heap.pop().unwrap();

        if state.position == map.end_position {
            dbg!("Found it!");
            for i in 0..=state.iteration {
                dbg!(i);
                map.print(&all_blizzards[i % all_blizzards.len()], &state.steps[i]);
            }
            dbg!(state.iteration);
            return;
        }

        state.iteration += 1;
        // map.print(
        //     &all_blizzards[(state.iteration - 1) % all_blizzards.len()],
        //     &state.position,
        // );
        let delta = ((state.position.0 as isize) - (map.end_position.0 as isize)).abs()
            + ((state.position.1 as isize) - (map.end_position.1 as isize)).abs();
        if delta < closest {
            closest = delta;
            dbg!(state.position);
            map.print(
                &all_blizzards[(state.iteration - 1) % all_blizzards.len()],
                &state.position,
            );
            dbg!(delta);
        }

        // map.print(
        //     &all_blizzards[(state.iteration) % all_blizzards.len()],
        //     &(0, 0),
        // );

        for new_position in map.get_legal_moves(
            &all_blizzards[state.iteration % all_blizzards.len()],
            &state.position,
        ) {
            let mut state = state.clone();
            state.position = new_position;
            if seen_states.insert(state.simple_state()) {
                state.steps.push(new_position);
                heap.push(state);
            }
        }
    }

    dbg!(heap);
    // 254 is too low
    println!("ugg");
}
