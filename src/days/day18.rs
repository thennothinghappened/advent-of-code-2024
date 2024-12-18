use itertools::Itertools;

use crate::utils::{
    boxdraw, not_yet_implemented,
    pos::{Index2d, Pos},
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

    not_yet_implemented()
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}
