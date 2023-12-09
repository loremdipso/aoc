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

pub type Map = HashMap<String, HashMap<String, Vec<(isize, isize, isize)>>>;

pub fn get_maps(
    contents: &str,
) -> (
    Vec<isize>,
    // { start_property => { end_property => [Mappings] } }
    Map,
) {
    let mut seeds = Vec::new();
    let mut maps: Map = HashMap::new();

    let mut mappings: Vec<(isize, isize, isize)> = Vec::new();
    let mut from_property = String::new();
    let mut to_property = String::new();

    for line in contents.lines().filter(|line| line.len() > 0) {
        if line.starts_with("seeds:") {
            for piece in line.split(" ") {
                if let Ok(seed) = piece.parse::<isize>() {
                    seeds.push(seed);
                }
            }
        } else if line.chars().next().unwrap().is_digit(10) {
            let mut pieces = line.split(" ").map(|piece| piece.parse::<isize>().unwrap());
            mappings.push((
                pieces.next().unwrap(),
                pieces.next().unwrap(),
                pieces.next().unwrap(),
            ));
        } else {
            if from_property.len() > 0 {
                let outer_entry = maps.entry(from_property).or_default();
                let inner_entry = outer_entry.entry(to_property).or_default();
                inner_entry.extend(mappings.clone());
                mappings.clear();
            }

            let (new_map_name, _) = line.split_once(" ").unwrap();
            let pieces = new_map_name.split("-").collect::<Vec<&str>>();
            from_property = pieces[0].to_string();
            to_property = pieces[2].to_string();
        }
    }

    // final one
    let outer_entry = maps.entry(from_property).or_default();
    let inner_entry = outer_entry.entry(to_property).or_default();
    inner_entry.extend(mappings.clone());
    return (seeds, maps);
}
