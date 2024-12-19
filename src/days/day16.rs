use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::utils::{
    boxdraw,
    direction::{Direction, DIRECTIONS},
    not_yet_implemented,
    pos::{Index2d, Pos},
    wait_for_user,
};

use super::{DayResult, PartResult};

// TODO: Get back to this one! This is hard!
pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let mut start_pos = Pos::from(0);
    let mut exit_pos = Pos::from(0);

    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => Some(None),
                    '#' => None,
                    'S' => {
                        start_pos = Pos::new_from_usize_unchecked(x, y);
                        Some(None)
                    }
                    'E' => {
                        exit_pos = Pos::new_from_usize_unchecked(x, y);
                        Some(None)
                    }
                    _ => panic!("Invalid char in grid!"),
                })
                .collect_vec()
        })
        .collect_vec();

    let result = path_find(&mut grid, start_pos, Direction::Right, exit_pos)
        .ok_or(anyhow::anyhow!("").into())
        .map(|dist| dist.to_string());

    result
}

fn part2(_input: &str) -> PartResult {
    not_yet_implemented()
}

fn path_find(
    grid: &mut Vec<Vec<Option<Option<(Direction, u32)>>>>,
    source: Pos,
    start_dir: Direction,
    destination: Pos,
) -> Option<u32> {
    const TURN_COST: u32 = 1000;

    // Set of nodes we haven't yet visited.
    let mut unvisited = FxHashSet::<Pos>::default();

    for y in 0..grid.height() as i32 {
        for x in 0..grid.width() as i32 {
            unvisited.insert(Pos::new(x, y));
        }
    }

    // The starting node is of course, a distance of 0 from itself!
    *grid.get_2d_mut_unchecked(source) = Some(Some((start_dir, 0)));

    while !unvisited.is_empty() {
        // Choose the closest node to evaluate from.
        let Some((&check_pos, (check_pos_facing, check_pos_dist))) = unvisited
            .iter()
            .filter_map(|pos| {
                grid.get_2d_unchecked(*pos)
                    .map(|maybe_dist| maybe_dist.map(|dist| (pos, dist)))
                    .flatten()
            })
            .sorted_by(|a, b| Ord::cmp(&a.1 .1, &b.1 .1))
            .next()
        else {
            break;
        };

        // If we're checking the destination... well, we've arrived!
        if check_pos == destination {
            break;
        }

        for (neighbour_pos, facing, dist_from_here) in [
            (check_pos_facing.turned_left(), TURN_COST + 1),
            (check_pos_facing, 1),
            (check_pos_facing.turned_right(), TURN_COST + 1),
        ]
        .iter()
        .map(|&(facing, cost)| (check_pos + facing, facing, check_pos_dist + cost))
        .filter(|(pos, _, _)| unvisited.contains(pos))
        {
            let Some(recorded) = grid.get_2d_mut_unchecked(neighbour_pos) else {
                continue;
            };

            if let Some((_, dist_recorded_there)) = recorded {
                if *dist_recorded_there <= dist_from_here {
                    continue;
                }
            }

            *recorded = Some((facing, dist_from_here));
        }

        // println!(
        //     "{}",
        //     boxdraw::draw_grid(grid.width(), grid.height(), |pos| {
        //         match grid.get_2d_unchecked(pos) {
        //             Some(Some((dir, _))) => char::from(*dir),
        //             Some(None) => '@',
        //             None => '░',
        //         }
        //     })
        // );
        // wait_for_user();

        unvisited.remove(&check_pos);
    }

    grid.get_2d_unchecked(destination)
        .flatten()
        .map(|(_, dist)| dist)
}
