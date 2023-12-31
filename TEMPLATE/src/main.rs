#![allow(dead_code, unused_variables)]
mod utils;

fn main() {
    let filename = include_str!("../sample.txt");
    //let filename = include_str!("../input.txt");

    part_1(filename);
    // part_2(filename);
}

fn part_1(contents: &str) {
    let lines = utils::get_lines::<String>(contents);
    dbg!(&lines);
    utils::copy_to_clipboard("hi! :)");
}
