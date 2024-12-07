use std::ops::{Index, IndexMut};

use enumflags2::BitFlags;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let initial_dir = Direction::Up;
    let initial_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &c)| match c {
                GUARD_INITIAL => Some(Pos {
                    x: x as i32,
                    y: y as i32,
                }),
                _ => None,
            })
        })
        .expect("Guard must have an initial position!");

    grid[initial_pos] = FLOOR;

    Ok((
        part1(&grid, initial_pos, initial_dir)?,
        part2(&mut grid, initial_pos, initial_dir)?,
    ))
}

const FLOOR: char = '.';
const WALL: char = '#';
const GUARD_INITIAL: char = '^';

fn part1(grid: &Vec<Vec<char>>, initial_pos: Pos, initial_dir: Direction) -> PartResult {
    let mut visit_grid = grid
        .iter()
        .map(|row| std::vec::from_elem(BitFlags::<Direction>::empty(), row.len()))
        .collect::<Vec<_>>();

    let visited = trace_path(&grid, &mut visit_grid, initial_pos, initial_dir)
        .expect("Somehow there was no valid exit!");

    Ok(visited.to_string())
}

fn part2(grid: &mut Vec<Vec<char>>, initial_pos: Pos, initial_dir: Direction) -> PartResult {
    // 1. Let's map out their path as in part 1.
    // 2. For each visited position, try placing a wall there.
    // 3. Record the pathfinding, rather than an X, store direction (bitwise mayhaps :P)
    // 4. If we've travelled the same position twice in the same direction we've made a loop.
    // 5. Output #loops.

    let mut visit_grid = grid
        .iter()
        .map(|row| std::vec::from_elem(BitFlags::<Direction>::empty(), row.len()))
        .collect::<Vec<_>>();

    trace_path(&grid, &mut visit_grid, initial_pos, initial_dir);

    let targets = visit_grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, col)| match col.is_empty() {
                    true => None,
                    false => Some(Pos {
                        x: x as i32,
                        y: y as i32,
                    }),
                })
                .filter(|&pos| pos != initial_pos)
        })
        .collect::<Vec<Pos>>();

    let mut valid_targets = 0;

    for target in targets {
        // Refresh the visit positions grid.
        visit_grid
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|col| *col = BitFlags::empty()));

        grid[target] = WALL;

        if let None = trace_path(&grid, &mut visit_grid, initial_pos, initial_dir) {
            valid_targets += 1;
        }

        grid[target] = FLOOR;
    }

    Ok(valid_targets.to_string())
}

/// Traces the path of the guard from `initial_pos` facing `initial_dir`, recording their path to
/// `visit_grid`. If the guard exited the map, the number of steps is returned. If they got stuck in
/// a loop however, [None](None) is returned.
fn trace_path(
    grid: &Vec<Vec<char>>,
    visit_grid: &mut Vec<Vec<BitFlags<Direction>>>,
    initial_pos: Pos,
    initial_dir: Direction,
) -> Option<usize> {
    let mut pos = initial_pos;
    let mut dir = initial_dir;
    let mut visit_count = 1;

    let grid_width = grid[pos.y as usize].len() as i32;
    let grid_height = grid.len() as i32;

    loop {
        if visit_grid[pos].contains(dir) {
            // We've been here before!
            return None;
        }

        let next_pos = pos + dir;
        visit_grid[pos] |= dir;

        if !next_pos.is_positive() || next_pos.y >= grid_height || next_pos.x >= grid_width {
            // Exiting the map.
            break;
        }

        if grid[next_pos] == WALL {
            dir = dir.turned_right();
            continue;
        }

        if visit_grid[next_pos].is_empty() {
            visit_count += 1;
        }

        pos = next_pos;
    }

    Some(visit_count)
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
        self + Pos::from(rhs)
    }
}

impl From<Pos> for (i32, i32) {
    fn from(value: Pos) -> (i32, i32) {
        (value.x, value.y)
    }
}

impl From<(i32, i32)> for Pos {
    fn from((x, y): (i32, i32)) -> Self {
        Pos { x, y }
    }
}

#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'V',
            Direction::Left => '<',
        }
    }
}

impl<T> Index<Pos> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Pos> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self[index.y as usize][index.x as usize]
    }
}
