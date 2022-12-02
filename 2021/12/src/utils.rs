use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::node::Node;

pub fn get_nodes(filename: &str) -> HashMap<String, Node>
{
    let mut nodes = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.len() > 0 {
                    let mut pieces = line.split("-");
                    let a_name = pieces.next().unwrap();
                    let b_name = pieces.next().unwrap();
                    insert_node(&mut nodes, a_name, b_name);
                }
            }
        }
    }
    return nodes;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn insert_node(nodes: &mut HashMap<String, Node>, a_name: &str, b_name: &str) {
    nodes.entry(a_name.to_owned())
        .or_insert_with(||
            Node {
                name: a_name.to_owned(),
                children: HashSet::new()
            }
        ).children.insert(b_name.to_owned());

    nodes.entry(b_name.to_owned())
        .or_insert_with(||
            Node {
                name: b_name.to_owned(),
                children: HashSet::new()
            }
        ).children.insert(a_name.to_owned());
}