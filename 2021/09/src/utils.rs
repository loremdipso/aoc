use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_lines(filename: &str) -> Vec<Vec<i64>> {
    let mut rv = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                if line.len() > 0 {
                    let mut v = vec![];
                    for char in line.chars() {
                        v.push(char.to_string().parse::<i64>().unwrap());
                    }
                    rv.push(v);
                }
            }
        }
    }
    return rv;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
