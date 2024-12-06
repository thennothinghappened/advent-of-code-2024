use crate::utils::not_yet_implemented;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut guard_pos = Pos {
        x: i32::MAX,
        y: i32::MAX,
    };
    let mut guard_dir = Direction::Up;
    let mut visited = 1;

    'find_guard: for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                guard_pos = Pos {
                    x: x as i32,
                    y: y as i32,
                };

                break 'find_guard;
            }
        }
    }

    debug_assert_ne!(
        guard_pos,
        Pos {
            x: i32::MAX,
            y: i32::MAX
        }
    );

    loop {
        // We don't account for the guard getting stuck, as the instructions don't mention this case
        // so presumably it does not occur. :)

        grid[guard_pos.y as usize][guard_pos.x as usize] = 'X';

        #[cfg(debug_assertions)]
        println!(
            "{}\nGuard :: (x: {}, y: {}) :: facing {:?}\n",
            grid.iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n"),
            guard_pos.x,
            guard_pos.y,
            guard_dir
        );

        let next_pos = guard_pos + guard_dir;

        if !next_pos.is_positive()
            || next_pos.y as usize >= grid.len()
            || next_pos.x as usize >= grid[next_pos.y as usize].len()
        {
            break;
        }

        let next_char = grid[next_pos.y as usize][next_pos.x as usize];

        if next_char == '#' {
            guard_dir = guard_dir.turned_right();
            continue;
        }

        if next_char != 'X' {
            visited += 1;
        }

        guard_pos = next_pos;
    }

    Ok(visited.to_string())
}

fn part2(input: &str) -> PartResult {
    // 1. Let's map out their path as in part 1.
    // 2. For each X, try placing a # there.
    // 3. Record the pathfinding, rather than an X, store direction (bitwise mayhaps :P)
    // 4. If we've travelled the same position twice in the same direction we've made a loop.
    // 5. Output #loops.

    not_yet_implemented()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn is_positive(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Into::<Pos>::into(rhs)
    }
}

impl Into<(i32, i32)> for Pos {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<(i32, i32)> for Pos {
    fn from((x, y): (i32, i32)) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turned_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Into<Pos> for Direction {
    fn into(self) -> Pos {
        match self {
            Direction::Up => Pos { x: 0, y: -1 },
            Direction::Right => Pos { x: 1, y: 0 },
            Direction::Down => Pos { x: 0, y: 1 },
            Direction::Left => Pos { x: -1, y: 0 },
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'V',
            Direction::Left => '<',
        }
    }
}
