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

pub fn get_lines<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut rv = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.len() > 0 {
                    rv.push(line.parse::<T>().unwrap());
                }
            }
        }
    }
    return rv;
}

pub fn get_groups<T>(filename: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut rv = vec![];
    if let Ok(lines) = read_lines(filename) {
        let mut temp = vec![];
        for line in lines {
            if let Ok(line) = line {
                if line.len() > 0 {
                    temp.push(line.parse::<T>().unwrap());
                } else {
                    rv.push(temp);
                    temp = vec![];
                }
            }
        }

        if temp.len() > 0 {
            rv.push(temp);
        }
    }
    return rv;
}

pub fn get_trees(filename: &str) -> Vec<Vec<i32>> {
    let mut rv = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                rv.push(
                    line.chars()
                        .map(|c| c.to_string().parse::<i32>().unwrap())
                        .collect(),
                );
            }
        }
    }
    return rv;
}
