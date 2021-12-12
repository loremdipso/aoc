mod utils;
use std::collections::HashMap;

use utils::get_nodes;
mod node;
use node::Node;

fn main() {
    let nodes = get_nodes("sample.txt");
    part_1(nodes);
}

fn part_1(nodes: HashMap<String, Node>) {
    dbg!(&nodes);
}
