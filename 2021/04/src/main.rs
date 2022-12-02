mod utils;
use utils::get_pieces;

fn main() {
    let mut pieces = get_pieces();
    let moves = pieces.remove(0);
    let boards = pieces.drain(..).map(|s| Board::new(s)).collect();
    dbg!(&moves);
    // dbg!(&boards);

    part_1(&moves[0], boards);
    // part_2(&moves[0], boards);
}

fn part_1(moves: &String, mut boards: Vec<Board>) {
    for number in moves.split(',') {
        for board in &mut boards {
            board.mark(number);
            if board.is_complete() {
                println!("Winner!");
                board.print();
                dbg!(board.score(number.parse::<u64>().unwrap()));
                println!();
                return;
            }
        }
    }
}

fn part_2(moves: &String, mut boards: Vec<Board>) {
    for number in moves.split(',') {
        dbg!(&number);
        dbg!(&boards.len());
        for board in &mut boards {
            board.mark(number);
        }

        if boards.len() == 1 {
            let board = &boards[0];
            if board.is_complete() {
                board.print();
                println!("Score: {}", board.score(number.parse::<u64>().unwrap()));
                println!();
                return;
            }
        }

        boards.retain(|board| !board.is_complete());
    }
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<String>>,
}

impl Board {
    pub fn new(rows: Vec<String>) -> Board {
        let rows = rows
            .iter()
            .map(|row| {
                row.split(' ')
                    .map(|row| row.trim().to_owned())
                    .filter(|row| row.len() > 0)
                    .collect()
            })
            .collect();
        return Board { rows };
    }

    pub fn mark(&mut self, number: &str) {
        for row in &mut self.rows {
            for column in row {
                if column == number {
                    *column = "".to_owned();
                }
            }
        }
    }

    pub fn score(&self, last_number: u64) -> u64 {
        let mut total = 0;
        for row in &self.rows {
            for column in row {
                if column.len() > 0 {
                    total += column.parse::<u64>().unwrap();
                }
            }
        }
        return total * last_number;
    }

    pub fn print(&self) {
        for row in &self.rows {
            for column in row {
                if column.len() == 0 {
                    print!("{:>3}", "x");
                } else {
                    print!("{:>3}", column);
                }
            }
            println!();
        }
    }

    pub fn is_complete(&self) -> bool {
        for row in &self.rows {
            if row.iter().all(|row| row.len() == 0) {
                return true;
            }
        }

        for column in self.columns() {
            if column.iter().all(|column| column.len() == 0) {
                return true;
            }
        }

        return false;
    }

    pub fn columns<'a>(&'a self) -> impl std::iter::Iterator<Item = Vec<&String>> + 'a {
        let count = self.rows[0].len();
        let mut index = 0;
        std::iter::from_fn(move || {
            if index >= count {
                return None;
            }

            let mut column = vec![];
            for row in &self.rows {
                column.push(&row[index]);
            }
            index += 1;
            return Some(column);
        })
    }
}
