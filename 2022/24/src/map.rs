use crate::blizzard::Blizzard;
use crate::direction::Direction;
use crate::Position;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub start_position: Position,
    pub end_position: Position,
}

impl Map {
    pub fn print(&self, blizzards: &Vec<Blizzard>, current_position: &Position) {
        let mut buffer = vec![vec!['.'; self.width]; self.height];

        for line_index in 0..self.height {
            let new_char = if line_index % 5 == 0 { '|' } else { '#' };
            buffer[line_index][0] = new_char;
            buffer[line_index][self.width - 1] = new_char;
        }
        for char_index in 0..self.width {
            let new_char = if char_index % 5 == 0 { '-' } else { '#' };
            buffer[0][char_index] = new_char;
            buffer[self.height - 1][char_index] = new_char;
        }

        buffer[self.end_position.0][self.end_position.1] = '.';

        let mut position_counts: HashMap<Position, usize> = HashMap::new();
        for blizzard in blizzards {
            let entry = position_counts.entry(blizzard.position).or_default();
            *entry += 1;
        }

        for blizzard in blizzards {
            let count = position_counts.get(&blizzard.position).unwrap();
            buffer[blizzard.position.0][blizzard.position.1] = match *count {
                1 => blizzard.direction.get_char(),
                2 => '2',
                3 => '3',
                4 => '4',
                _ => panic!(),
            };
        }

        // Sanity check
        match buffer[current_position.0][current_position.1] {
            '#' | '.' | '-' | '|' => {}
            _ => {
                panic!("What!?!??!");
            }
        };

        buffer[current_position.0][current_position.1] = 'E';

        for line in buffer.iter() {
            for char in line.iter() {
                print!("{char}");
            }
            println!();
        }
        println!();
    }

    pub fn get_all_blizzards(&self, blizzards: &mut Vec<Blizzard>) -> Vec<Vec<Blizzard>> {
        let mut seen_blizzards: HashSet<Vec<Blizzard>> = HashSet::new();
        let mut all_blizzards = Vec::new();
        all_blizzards.push(blizzards.clone());
        seen_blizzards.insert(blizzards.clone());
        loop {
            for blizzard in &mut *blizzards {
                self.move_blizzard(blizzard);
            }
            if !seen_blizzards.insert(blizzards.clone()) {
                break;
            }
            all_blizzards.push(blizzards.clone());
        }
        return all_blizzards;
    }

    pub fn move_blizzard(&self, blizzard: &mut Blizzard) {
        match blizzard.direction {
            Direction::NORTH => blizzard.position.0 -= 1,
            Direction::SOUTH => blizzard.position.0 += 1,
            Direction::EAST => blizzard.position.1 += 1,
            Direction::WEST => blizzard.position.1 -= 1,
        };

        if blizzard.position.0 == 0 {
            blizzard.position.0 = self.height - 2;
        } else if blizzard.position.0 == self.height - 1 {
            blizzard.position.0 = 1;
        } else if blizzard.position.1 == 0 {
            blizzard.position.1 = self.width - 2;
        } else if blizzard.position.1 == self.width - 1 {
            blizzard.position.1 = 1;
        }
    }

    pub fn get_legal_moves(
        &self,
        blizzards: &Vec<Blizzard>,
        current_position: &Position,
    ) -> Vec<Position> {
        let mut positions = Vec::new();

        let mut potential_positions = vec![current_position.clone()];
        'direction_loop: for direction in vec![
            Direction::NORTH,
            Direction::SOUTH,
            Direction::EAST,
            Direction::WEST,
        ] {
            // End position is legal
            if let Some(new_position) = direction.try_get_new_position(current_position) {
                // Special case: can't go through blizzard
                for blizzard in blizzards {
                    if blizzard.position == *current_position {
                        let mut previous_blizzard = blizzard.clone();
                        previous_blizzard.direction = previous_blizzard.direction.invert();
                        self.move_blizzard(&mut previous_blizzard);
                        if previous_blizzard.position == new_position {
                            // println!("This is bad"),
                            // dbg!(current_position, new_position);
                            continue 'direction_loop;
                        }
                    }
                }
                potential_positions.push(new_position);
            }
        }

        for new_position in potential_positions {
            if new_position == self.end_position || new_position == self.start_position {
                positions.push(new_position);
                continue;
            }

            // Stay in bounds
            if new_position.0 == 0
                || new_position.0 >= self.height - 1
                || new_position.1 == 0
                || new_position.1 >= self.width - 1
            {
                continue;
            }

            // Can't go into blizzard
            if blizzards
                .iter()
                .any(|blizzard| blizzard.position == new_position)
            {
                continue;
            }

            positions.push(new_position);
        }

        return positions;
    }
}
