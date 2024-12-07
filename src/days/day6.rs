use std::{
    ops::{Index, IndexMut},
    path::Display,
};

use enumflags2::BitFlags;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    let mut grid = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<_>>();

    let grid_width = input
        .lines()
        .next()
        .expect("Grid shouldn't be empty...")
        .len();

    let grid_height = grid.len() / grid_width;

    let initial_dir = Direction::Up;
    let initial_pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, &col)| match col {
            GUARD_INITIAL => Some(Pos::from_flat_index(grid_width, i)),
            _ => None,
        })
        .expect("Guard must have an initial position!");

    *grid.flat_index_mut(grid_width, initial_pos) = FLOOR;

    Ok((
        part1(&grid, grid_width, grid_height, initial_pos, initial_dir)?,
        part2(&mut grid, grid_width, grid_height, initial_pos, initial_dir)?,
    ))
}

const FLOOR: char = '.';
const WALL: char = '#';
const GUARD_INITIAL: char = '^';

fn part1(
    grid: &Vec<char>,
    grid_width: usize,
    grid_height: usize,
    initial_pos: Pos,
    initial_dir: Direction,
) -> PartResult {
    let mut visit_grid = vec![BitFlags::<Direction>::empty(); grid_width * grid_height];
    let visited = trace_path(
        &grid,
        grid_width,
        grid_height,
        &mut visit_grid,
        initial_pos,
        initial_dir,
    )
    .expect("Somehow there was no valid exit!");

    Ok(visited.to_string())
}

fn part2(
    grid: &mut Vec<char>,
    grid_width: usize,
    grid_height: usize,
    initial_pos: Pos,
    initial_dir: Direction,
) -> PartResult {
    // 1. Let's map out their path as in part 1.
    // 2. For each visited position, try placing a wall there.
    // 3. Record the pathfinding, rather than an X, store direction (bitwise mayhaps :P)
    // 4. If we've travelled the same position twice in the same direction we've made a loop.
    // 5. Output #loops.

    let mut visit_grid = vec![BitFlags::<Direction>::empty(); grid_width * grid_height];
    trace_path(
        &grid,
        grid_width,
        grid_height,
        &mut visit_grid,
        initial_pos,
        initial_dir,
    );

    let targets = visit_grid
        .iter()
        .enumerate()
        .filter_map(|(i, col)| match col.is_empty() {
            true => None,
            false => Some(Pos::from_flat_index(grid_width, i)),
        })
        .filter(|&pos| pos != initial_pos)
        .collect::<Vec<Pos>>();

    let mut valid_targets = 0;

    for target in targets {
        // Refresh the visit positions grid.
        visit_grid
            .iter_mut()
            .for_each(|col| *col = BitFlags::empty());

        *grid.flat_index_mut(grid_width, target) = WALL;

        if let None = trace_path(
            &grid,
            grid_width,
            grid_height,
            &mut visit_grid,
            initial_pos,
            initial_dir,
        ) {
            valid_targets += 1;
        }

        *grid.flat_index_mut(grid_width, target) = FLOOR;
    }

    Ok(valid_targets.to_string())
}

/// Traces the path of the guard from `initial_pos` facing `initial_dir`, recording their path to
/// `visit_grid`. If the guard exited the map, the number of steps is returned. If they got stuck in
/// a loop however, [None](None) is returned.
fn trace_path(
    grid: &Vec<char>,
    grid_width: usize,
    grid_height: usize,
    visit_grid: &mut Vec<BitFlags<Direction>>,
    initial_pos: Pos,
    initial_dir: Direction,
) -> Option<usize> {
    let mut pos = initial_pos;
    let mut dir = initial_dir;
    let mut visit_count = 1;

    loop {
        // println!(
        //     "{}\nGuard :: (x: {}, y: {}) :: facing {:?} :: Spot contains {}\n",
        //     grid.chunks(grid_width)
        //         .zip(visit_grid.chunks(grid_width))
        //         .map(|(map_row, visits_row)| {
        //             let mut str = String::new();

        //             for i in 0..grid_width {
        //                 let visits_col = visits_row[i];

        //                 if !visits_col.is_empty() {
        //                     str.push('X');
        //                 } else {
        //                     str.push(map_row[i]);
        //                 }
        //             }

        //             str
        //         })
        //         .collect::<Vec<String>>()
        //         .join("\n"),
        //     pos.x,
        //     pos.y,
        //     dir,
        //     visit_grid.flat_index(grid_width, pos)
        // );

        if visit_grid.flat_index(grid_width, pos).contains(dir) {
            // We've been here before!
            return None;
        }

        *visit_grid.flat_index_mut(grid_width, pos) |= dir;
        let next_pos = pos + dir;

        if !next_pos.is_positive()
            || next_pos.y as usize >= grid_height
            || next_pos.x as usize >= grid_width
        {
            // Exiting the map.
            break;
        }

        if *grid.flat_index(grid_width, next_pos) == WALL {
            dir = dir.turned_right();
            continue;
        }

        if visit_grid.flat_index(grid_width, next_pos).is_empty() {
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

    fn from_flat_index(grid_width: usize, index: usize) -> Self {
        let x = index % grid_width;
        let y = (index - x) / grid_width;

        Pos {
            x: x as i32,
            y: y as i32,
        }
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

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
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

trait FlatIndex<T> {
    type Output: ?Sized;
    fn flat_index(&self, width: usize, index: T) -> &Self::Output;
}

trait FlatIndexMut<T>: FlatIndex<T> {
    fn flat_index_mut(&mut self, width: usize, index: T) -> &mut Self::Output;
}

impl<T> FlatIndex<Pos> for Vec<T> {
    type Output = T;

    fn flat_index(&self, width: usize, index: Pos) -> &Self::Output {
        &self[index.x as usize + (index.y as usize) * width]
    }
}

impl<T> FlatIndexMut<Pos> for Vec<T> {
    fn flat_index_mut(&mut self, width: usize, index: Pos) -> &mut Self::Output {
        &mut self[index.x as usize + (index.y as usize) * width]
    }
}
