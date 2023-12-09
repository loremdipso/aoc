use crate::Position;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct State {
    pub position: Position,
    pub iteration: usize,
    pub steps: Vec<Position>,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct SimpleState {
    pub position: Position,
    pub iteration: usize,
}

impl State {
    pub fn simple_state(&self) -> SimpleState {
        SimpleState {
            position: self.position.clone(),
            iteration: self.iteration,
        }
    }
}

impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        (self.position == other.position) && (self.iteration == other.iteration)
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.iteration.cmp(&self.iteration)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
