#![allow(dead_code, unused_mut, unused_variables)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};
mod utils;

fn main() {
    // let filename = "sample.txt";
    let filename = "input.txt";

    // part_1(filename);
    part_2(filename);
}

#[derive(Debug)]
struct Entry {
    size: usize,
    name: String,
    parent: Option<Rc<RefCell<Entry>>>,
    children: HashMap<String, Rc<RefCell<Entry>>>,
    is_dir: bool,
}

impl Entry {
    pub fn new_folder(name: &str, size: usize, parent: Option<Rc<RefCell<Entry>>>) -> Self {
        Entry {
            children: HashMap::new(),
            size,
            name: name.to_string(),
            parent,
            is_dir: true,
        }
    }

    pub fn new_file(name: &str, size: usize, parent: Option<Rc<RefCell<Entry>>>) -> Self {
        Entry {
            children: HashMap::new(),
            size,
            name: name.to_string(),
            parent,
            is_dir: false,
        }
    }
}

fn part_1(filename: &str) {
    let root = get_root(filename);

    print_entry(&root, "");
    let total = get_sum_dirs_of_size_at_most(&root, 100000);
    dbg!(total);
}

fn part_2(filename: &str) {
    let root = get_root(filename);

    let goal = 30000000 - (70000000 - root.borrow().size);
    dbg!(goal);

    print_entry(&root, "");
    let size = get_dir_sizes_at_least(&root, goal);
    dbg!(size);
}

fn get_root(filename: &str) -> Rc<RefCell<Entry>> {
    let mut root = Rc::new(RefCell::new(Entry::new_folder("/", 0, None)));
    let mut current_dir = root.clone();

    let lines = utils::get_lines::<String>(filename);
    for line in &lines {
        let temp = current_dir.clone();
        let mut temp = temp.borrow_mut();

        if line.starts_with("$") {
            // execute command
            let pieces: Vec<&str> = line.split(" ").collect();
            match pieces[1] {
                "cd" => {
                    match pieces[2] {
                        "/" => {
                            current_dir = root.clone();
                        }

                        // ruh-roh >_<
                        ".." => {
                            if let Some(parent) = &temp.parent {
                                current_dir = parent.clone();
                                continue;
                            } else {
                                // I guess this was fine
                                // panic!();
                            }
                        }

                        dir_name => {
                            // switch to existing
                            if temp.children.contains_key(dir_name) {
                                current_dir = temp.children.get(dir_name).unwrap().clone();
                                continue;
                            }

                            // create new
                            let new_entry = Rc::new(RefCell::new(Entry::new_folder(
                                dir_name,
                                0,
                                Some(current_dir.clone()),
                            )));

                            temp.children
                                .insert(dir_name.to_string(), new_entry.clone());
                            current_dir = new_entry.clone();
                        }
                    }
                }

                "ls" => {
                    // do nothing, I guess
                }

                _ => panic!(),
            };
        } else {
            // parse output
            let pieces: Vec<&str> = line.split(" ").collect();

            match pieces[0] {
                "dir" => {
                    // println!("Skipping dirs in ls for now...");
                }

                size => {
                    let size = size.parse::<usize>().unwrap();
                    let name = pieces[1];

                    // check if we already have this bad boy
                    if temp.children.contains_key(name) {
                        continue;
                    }

                    // create new
                    let new_entry = Rc::new(RefCell::new(Entry::new_file(
                        name,
                        size,
                        Some(current_dir.clone()),
                    )));

                    temp.children.insert(name.to_string(), new_entry.clone());
                }
            }
        }
    }

    fix_sizes(&root, "");
    return root;
}

fn fix_sizes(entry: &RefCell<Entry>, indent: &str) -> usize {
    let mut entry = entry.borrow_mut();
    let indent = indent.to_string() + "   ";
    let mut total = 0;
    for (name, child) in entry.children.iter() {
        let size = fix_sizes(child, &indent);
        total += size;
    }

    if total > 0 {
        entry.size += total;
    }

    return entry.size;
}

fn print_entry(entry: &RefCell<Entry>, indent: &str) {
    let mut entry = entry.borrow_mut();
    println!("{}{} ({})", indent, entry.name, entry.size);
    let indent = indent.to_string() + "   ";
    for (name, child) in entry.children.iter() {
        print_entry(child, &indent);
    }
}

fn get_sum_dirs_of_size_at_most(entry: &RefCell<Entry>, max: usize) -> usize {
    let mut total: usize = 0;

    let mut entry = entry.borrow_mut();
    if entry.is_dir && entry.size < max {
        total += entry.size;
    }

    // what about this?
    for (name, child) in entry.children.iter() {
        total += get_sum_dirs_of_size_at_most(child, max);
    }

    return total;
}

fn get_dir_sizes_at_least(entry: &RefCell<Entry>, min: usize) -> usize {
    let mut smallest = usize::MAX;

    let mut entry = entry.borrow_mut();
    if entry.is_dir && entry.size >= min {
        smallest = entry.size;
        println!("Option: {}", smallest);
    }

    // what about this?
    for (name, child) in entry.children.iter() {
        let temp = get_dir_sizes_at_least(child, min);
        if temp < smallest {
            smallest = temp;
        }
    }

    return smallest;
}
