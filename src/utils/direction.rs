use super::pos::Pos;

#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl std::ops::Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Pos::from(rhs)
    }
}

impl Direction {
    pub fn turned_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            Direction::Up => true,
            Direction::Right => false,
            Direction::Down => true,
            Direction::Left => false,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::Up => false,
            Direction::Right => true,
            Direction::Down => false,
            Direction::Left => true,
        }
    }
}

impl From<Direction> for Pos {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Pos { x: 0, y: -1 },
            Direction::Right => Pos { x: 1, y: 0 },
            Direction::Down => Pos { x: 0, y: 1 },
            Direction::Left => Pos { x: -1, y: 0 },
        }
    }
}
