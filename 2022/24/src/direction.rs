use crate::Position;

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    pub fn get_char(&self) -> char {
        match self {
            Direction::NORTH => '^',
            Direction::SOUTH => 'v',
            Direction::EAST => '>',
            Direction::WEST => '<',
        }
    }

    pub fn try_get_new_position(&self, position: &Position) -> Option<Position> {
        match self {
            Direction::NORTH => {
                if position.0 > 0 {
                    Some((position.0 - 1, position.1))
                } else {
                    None
                }
            }
            Direction::SOUTH => Some((position.0 + 1, position.1)),
            Direction::EAST => Some((position.0, position.1 + 1)),
            Direction::WEST => {
                if position.1 > 0 {
                    Some((position.0, position.1 - 1))
                } else {
                    None
                }
            }
        }
    }

    pub fn invert(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::SOUTH => Direction::NORTH,
            Direction::EAST => Direction::WEST,
            Direction::WEST => Direction::EAST,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(input: char) -> Result<Direction, Self::Error> {
        match input {
            '^' => return Ok(Direction::NORTH),
            'v' => return Ok(Direction::SOUTH),
            '>' => return Ok(Direction::EAST),
            '<' => return Ok(Direction::WEST),
            _ => Err(()),
        }
    }
}
