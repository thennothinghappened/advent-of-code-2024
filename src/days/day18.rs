use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::utils::{
    boxdraw,
    direction::DIRECTIONS,
    not_yet_implemented,
    pos::{self, Index2d, Pos},
};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

const GRID_WIDTH: usize = 7;
const GRID_HEIGHT: usize = 7;

const START: Pos = Pos { x: 0, y: 0 };
const FINISH: Pos = Pos {
    x: GRID_WIDTH as i32 - 1,
    y: GRID_HEIGHT as i32 - 1,
};

fn part1(input: &str) -> PartResult {
    let barricade_positions_thru_time: Vec<Pos> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple::<(i32, i32)>()
                .unwrap()
                .into()
        })
        .take(12)
        .collect_vec();

    let passibility_grid = (0..GRID_WIDTH)
        .map(|y| {
            (0..GRID_WIDTH)
                .map(|x| {
                    !barricade_positions_thru_time.contains(&Pos::new_from_usize_unchecked(x, y))
                })
                .collect_vec()
        })
        .collect_vec();

    println!(
        "{}",
        boxdraw::draw_shape_outline(GRID_WIDTH, GRID_HEIGHT, |pos| {
            *passibility_grid.get_2d_unchecked(pos)
        })
    );

    // Set of all nodes, and their associated distances from the starting node.
    let mut nodes = FxHashMap::<Pos, u64>::default();

    // Set of nodes we haven't yet visited.
    let mut unvisited = FxHashSet::<Pos>::default();

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let pos = Pos::new_from_usize_unchecked(x, y);

            if *passibility_grid.get_2d_unchecked(pos) {
                nodes.insert(pos, u64::MAX);
                unvisited.insert(pos);
            }
        }
    }

    // The starting node is of course, a distance of 0 from itself!
    nodes.entry(START).and_modify(|distance| *distance = 0);

    while !unvisited.is_empty() {
        // Choose the closest node to evaluate from.
        let Some((&check_pos, &check_pos_dist)) = unvisited
            .iter()
            .map(|pos| nodes.get_key_value(pos).unwrap())
            .filter(|&(_, dist)| *dist < u64::MAX)
            .sorted_by(|a, b| Ord::cmp(a.1, b.1))
            .next()
        else {
            break;
        };

        for neighbour_pos in DIRECTIONS
            .iter()
            .map(|&direction| check_pos + direction)
            .filter(|pos| unvisited.contains(pos))
        {
            let dist_from_here = check_pos_dist + 1;

            nodes.entry(neighbour_pos).and_modify(|dist| {
                if *dist > dist_from_here {
                    *dist = dist_from_here;
                }
            });
        }

        unvisited.remove(&check_pos);
    }

    println!(
        "{:?}\n\nCost to reach exit: {:?}",
        nodes,
        nodes.get(&FINISH).unwrap()
    );

    not_yet_implemented()
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}
