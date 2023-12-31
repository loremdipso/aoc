use copypasta::{ClipboardContext, ClipboardProvider};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn copy_to_clipboard<T>(line: T)
where
    String: From<T>,
{
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(line.into()).unwrap();
    ctx.get_contents().unwrap();
}

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

pub fn get_grid(contents: &str) -> Vec<Vec<char>> {
    let mut rv = vec![];
    for line in contents.lines() {
        if line.len() > 0 {
            let mut temp = Vec::new();
            for char in line.chars() {
                temp.push(char);
            }
            rv.push(temp);
        }
    }
    return rv;
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        for char in line {
            print!("{char}");
        }
        println!();
    }
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
