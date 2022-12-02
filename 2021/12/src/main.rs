mod utils;
use std::collections::{HashMap, HashSet};

use utils::get_nodes;
mod node;
use node::Node;

fn main() {
    //let nodes = get_nodes("sample.txt");
    let nodes = get_nodes("input.txt");
    //dbg!(&nodes);
    //part_1(nodes);
    part_2(nodes);
}

fn part_1(nodes: HashMap<String, Node>) {
    let mut paths = vec![vec!["start".to_owned()]];
    let mut unique_paths: HashSet<String> = HashSet::new();
    loop {
        if let Some(path) = paths.pop() {
            let last = path.last().unwrap();
            if last == "end" {
                unique_paths.insert(path_to_string(&path));
            } else {
                //dbg!(&last);
                let node = nodes.get(last).unwrap();
                for child in &node.children {
                    let mut temp_path = path.clone();
                    temp_path.push(child.to_owned());
                    if valid(&temp_path) {
                        paths.push(temp_path);
                    }
                }
            }
        } else {
            break;
        };
    }

    dbg!(&unique_paths);
    dbg!(&unique_paths.len());
}

fn part_2(nodes: HashMap<String, Node>) {
    let mut paths = vec![vec!["start".to_owned()]];
    let mut unique_paths: HashSet<String> = HashSet::new();
    loop {
        if let Some(path) = paths.pop() {
            //dbg!(path_to_string(&path));
            let last = path.last().unwrap();
            if last == "end" {
                unique_paths.insert(path_to_string(&path));
            } else {
                //dbg!(&last);
                let node = nodes.get(last).unwrap();
                for child in &node.children {
                    let mut temp_path = path.clone();
                    temp_path.push(child.to_owned());
                    if valid2(&temp_path) {
                        paths.push(temp_path);
                    }
                }
            }
        } else {
            break;
        };
    }

    dbg!(&unique_paths);
    dbg!(&unique_paths.len());
}


fn valid(path: &Vec<String>) -> bool {
    for name in path.iter() {
        if Node::is_small(name) && path.iter().filter(|&elem| elem == name).count() > 1 {
            return false;
        }
    }
    return true;
}

fn valid2(path: &Vec<String>) -> bool {
    let mut double_smalls = HashSet::new();
    for name in path.iter() {
        if Node::is_small(name) {
            match path.iter().filter(|&elem| elem == name).count() {
                1 => {},
                2 => { double_smalls.insert(name.to_owned()); },
                _ => { return false; }
            };
        }
    }

    if path.iter().filter(|&elem| elem == "end").count() > 1 {
        return false;
    }

    if path.iter().filter(|&elem| elem == "start").count() > 1 {
        return false;
    }

    if double_smalls.len() > 1 {
        return false;
    }
    return true;
}

fn path_to_string(path: &Vec<String>) -> String {
    return String::from_iter(path.iter().map(|s| format!("{} -> ", s)));
}