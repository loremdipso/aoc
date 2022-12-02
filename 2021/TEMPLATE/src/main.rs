mod utils;
use utils::get_lines;

fn main() {
    let lines = get_lines::<i64>("sample.txt");
    dbg!(&lines);
}
