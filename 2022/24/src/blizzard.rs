use crate::direction::Direction;
use crate::map::Map;
use crate::utils;
use crate::Position;

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Blizzard {
    pub direction: Direction,
    pub position: Position,
}

pub fn get_blizzards(contents: &str) -> (Map, Vec<Blizzard>, Position) {
    let lines = utils::get_lines::<String>(contents);
    let width = lines[0].len();
    let height = lines.len();

    let mut start_position: Position = (0, 0);
    let mut end_position: Position = (0, 0);
    let mut blizzards = Vec::new();
    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            let position = (line_index, char_index);
            if line_index == 0 && char == '.' {
                start_position = position;
            } else if line_index == height - 1 && char == '.' {
                end_position = position;
            } else {
                if let Ok(direction) = Direction::try_from(char) {
                    blizzards.push(Blizzard {
                        position,
                        direction,
                    });
                }
            }
        }
    }

    let map = Map {
        width,
        height,
        end_position,
        start_position,
    };

    return (map, blizzards, start_position);
}
