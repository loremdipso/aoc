use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_lines(filename: &str) -> Vec<i64> {
    let mut rv = vec![];
    if let Ok(lines) = read_lines(filename) {
        let mut temp = 0;
        for line in lines {
            if let Ok(line) = line {
                if line.len() > 0 {
                    temp += line.parse::<i64>().unwrap();
                } else {
                    rv.push(temp);
                    temp = 0;
                }
            }
        }

        if temp > 0 {
            rv.push(temp);
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
