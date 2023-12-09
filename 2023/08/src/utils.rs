use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_lines<T>(contents: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut rv = vec![];
    for line in contents.lines() {
        if line.len() > 0 {
            rv.push(line.parse::<T>().unwrap());
        }
    }
    return rv;
}

pub fn get_groups<T>(contents: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut rv = vec![];
    let mut temp = vec![];
    for line in contents.lines() {
        if line.len() > 0 {
            temp.push(line.parse::<T>().unwrap());
        } else {
            rv.push(temp);
            temp = vec![];
        }
    }

    if temp.len() > 0 {
        rv.push(temp);
    }
    return rv;
}

type ConvertLineFn<T> = fn(line: &str) -> T;
pub fn get_generic<T>(contents: &str, func: ConvertLineFn<T>) -> Vec<T> {
    let mut rv = vec![];
    for line in contents.lines() {
        rv.push(func(&line));
    }
    return rv;
}

#[derive(Debug, Clone)]
pub struct Node {
    pub label: String,
    pub left: String,
    pub right: String,
}

pub fn get_data(contents: &str) -> (String, HashMap<String, Node>) {
    let mut rv = HashMap::new();
    let mut directions = String::new();
    for line in contents.lines().filter(|l| l.len() > 0) {
        if directions.len() == 0 {
            directions = line.to_string();
        } else {
            let (label, line) = line.split_once(" = ").unwrap();
            let label = label.trim();
            let line = line.replace("(", "");
            let line = line.replace(")", "");
            let (left, right) = line.split_once(", ").unwrap();
            rv.insert(
                label.to_string(),
                Node {
                    label: label.to_string(),
                    left: left.to_string(),
                    right: right.to_string(),
                },
            );
        }
    }
    return (directions, rv);
}
