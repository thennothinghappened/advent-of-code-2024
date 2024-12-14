use itertools::Itertools;

use crate::utils::{not_yet_implemented, pos::Pos};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    const GRID_WIDTH: i32 = 101;
    const GRID_HEIGHT: i32 = 103;

    let robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|part| {
                    part[2..]
                        .split(",")
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .map(Pos::from)
                .collect_tuple::<(Pos, Pos)>()
                .unwrap()
                .into()
        })
        .collect_vec();

    println!("{:?}", robots);

    not_yet_implemented()
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Pos,
    velocity: Pos,
}

impl From<(Pos, Pos)> for Robot {
    fn from((pos, velocity): (Pos, Pos)) -> Self {
        Robot { pos, velocity }
    }
}
