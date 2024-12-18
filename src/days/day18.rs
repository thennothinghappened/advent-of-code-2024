use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::utils::{
    direction::DIRECTIONS,
    pos::{Index2d, Pos},
};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

const GRID_WIDTH: usize = 71;
const GRID_HEIGHT: usize = 71;

const START: Pos = Pos { x: 0, y: 0 };
const FINISH: Pos = Pos {
    x: GRID_WIDTH as i32 - 1,
    y: GRID_HEIGHT as i32 - 1,
};

fn part1(input: &str) -> PartResult {
    let mut passibility_grid = [[Some(u32::MAX); GRID_WIDTH]; GRID_HEIGHT];

    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple::<(i32, i32)>()
                .unwrap()
                .into()
        })
        .take(1024)
        .for_each(|pos: Pos| {
            *passibility_grid.get_2d_mut_unchecked(pos) = None;
        });

    // println!(
    //     "{}",
    //     boxdraw::draw_shape_outline(GRID_WIDTH, GRID_HEIGHT, |pos| {
    //         *passibility_grid.get_2d_unchecked(pos)
    //     })
    // );

    match path_find(&mut passibility_grid, START, FINISH) {
        Some(steps) => Ok(steps.to_string()),
        None => Err(anyhow::anyhow!("No valid path found!").into()),
    }
}

fn part2(input: &str) -> PartResult {
    let mut passibility_grid = [[Some(u32::MAX); GRID_WIDTH]; GRID_HEIGHT];

    for block in input.lines().map(|line| {
        Pos::from(
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple::<(i32, i32)>()
                .unwrap(),
        )
    }) {
        // Add the next block.
        *passibility_grid.get_2d_mut_unchecked(block) = None;

        // Try pathfinding.
        if path_find(&mut passibility_grid, START, FINISH).is_none() {
            return Ok(format!("{},{}", block.x, block.y));
        }

        // Reset distances.
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if let Some(dist) =
                    passibility_grid.get_2d_mut_unchecked(Pos::new_from_usize_unchecked(x, y))
                {
                    *dist = u32::MAX;
                }
            }
        }
    }

    Err(anyhow::anyhow!("Somehow the path is never blocked??").into())
}

fn path_find(
    grid: &mut [[Option<u32>; GRID_WIDTH]; GRID_HEIGHT],
    source: Pos,
    destination: Pos,
) -> Option<u32> {
    // Set of nodes we haven't yet visited.
    let mut unvisited = FxHashSet::<Pos>::default();

    for y in 0..GRID_HEIGHT as i32 {
        for x in 0..GRID_WIDTH as i32 {
            unvisited.insert(Pos::new(x, y));
        }
    }

    // The starting node is of course, a distance of 0 from itself!
    *grid.get_2d_mut_unchecked(source) = Some(0);

    while !unvisited.is_empty() {
        // Choose the closest node to evaluate from.
        let Some((&check_pos, check_pos_dist)) = unvisited
            .iter()
            .filter_map(|pos| grid.get_2d_unchecked(*pos).map(|dist| (pos, dist)))
            .filter(|&(_, dist)| dist < u32::MAX)
            .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
            .next()
        else {
            break;
        };

        // If we're checking the destination... well, we've arrived!
        if check_pos == destination {
            break;
        }

        for neighbour_pos in DIRECTIONS
            .iter()
            .map(|&direction| check_pos + direction)
            .filter(|pos| unvisited.contains(pos))
        {
            let dist_from_here = check_pos_dist + 1;

            let Some(dist_recorded_there) = grid.get_2d_mut_unchecked(neighbour_pos) else {
                continue;
            };

            if dist_from_here < *dist_recorded_there {
                *dist_recorded_there = dist_from_here;
            }
        }

        unvisited.remove(&check_pos);
    }

    grid.get_2d_unchecked(destination)
        .filter(|&dist| dist < u32::MAX)
}

// /// Trace the route from `source` to `destination` after [path_find] has been used to first find the
// /// cheapest path.
// ///
// /// Returns the list of positions along the path. If no valid path exists, [None] is returned.
// fn path_trace_route(
//     grid: [[Option<u32>; GRID_WIDTH]; GRID_HEIGHT],
//     source: Pos,
//     destination: Pos,
// ) -> Option<Vec<Pos>> {

//     let mut

//     todo!()
// }
