use itertools::Itertools;

use crate::utils::{not_yet_implemented, pos::Pos};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

const A_COST: u64 = 3;
const B_COST: u64 = 1;

fn part1(input: &str) -> PartResult {
    let machines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(": ").unwrap().1.split_once(", ").unwrap())
        .tuples::<((&str, &str), (&str, &str), (&str, &str))>()
        .map(|(a, b, prize)| Machine {
            a: Pos {
                x: a.0[2..].parse().unwrap(),
                y: a.1[2..].parse().unwrap(),
            },
            b: Pos {
                x: b.0[2..].parse().unwrap(),
                y: b.1[2..].parse().unwrap(),
            },
            prize: Pos {
                x: prize.0[2..].parse().unwrap(),
                y: prize.1[2..].parse().unwrap(),
            },
        });

    not_yet_implemented()
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

#[derive(Debug)]
struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}
