use crate::utils::{
    direction::Direction,
    pos::{FlatIndex, FlatIndexMut, Pos},
};

use super::{DayResult, PartResult};
use enumflags2::BitFlags;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub(crate) fn solve(input: &str) -> DayResult {
    let mut grid = input
        .lines()
        .flat_map(|line| line.bytes())
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

const FLOOR: u8 = b'.';
const WALL: u8 = b'#';
const GUARD_INITIAL: u8 = b'^';

fn part1(
    grid: &[u8],
    grid_width: usize,
    grid_height: usize,
    initial_pos: Pos,
    initial_dir: Direction,
) -> PartResult {
    let mut visit_grid = vec![BitFlags::<Direction>::empty(); grid_width * grid_height];
    let visited = trace_path(
        grid,
        &mut visit_grid,
        grid_width,
        grid_height,
        initial_pos,
        initial_dir,
        None,
    )
    .expect("Somehow there was no valid exit!");

    Ok(visited.to_string())
}

fn part2(
    grid: &mut [u8],
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
        grid,
        &mut visit_grid,
        grid_width,
        grid_height,
        initial_pos,
        initial_dir,
        None,
    );

    let valid_targets = visit_grid
        .par_iter()
        .enumerate()
        .filter_map(|(i, col)| match col.is_empty() {
            true => None,
            false => Some(Pos::from_flat_index(grid_width, i)),
        })
        .filter(|&pos| pos != initial_pos)
        .filter(|&pos| {
            trace_path(
                grid,
                &mut vec![BitFlags::<Direction>::empty(); grid_width * grid_height],
                grid_width,
                grid_height,
                initial_pos,
                initial_dir,
                Some(pos),
            )
            .is_none()
        })
        .count();

    Ok(valid_targets.to_string())
}

/// Traces the path of the guard from `initial_pos` facing `initial_dir`, recording their path to
/// `visit_grid`. If the guard exited the map, the number of steps is returned. If they got stuck in
/// a loop however, [None](None) is returned.
fn trace_path(
    grid: &[u8],
    visit_grid: &mut [BitFlags<Direction>],
    grid_width: usize,
    grid_height: usize,
    initial_pos: Pos,
    initial_dir: Direction,
    additional_wall: Option<Pos>,
) -> Option<usize> {
    let mut pos = initial_pos;
    let mut dir = initial_dir;
    let mut visit_count = 1;

    loop {
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

        if *grid.flat_index(grid_width, next_pos) == WALL
            || additional_wall.is_some_and(|it| it == next_pos)
        {
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
