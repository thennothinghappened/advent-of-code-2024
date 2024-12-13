use itertools::Itertools;

use crate::utils::{not_yet_implemented, pos::Pos};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    let machines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(": ").unwrap().1.split_once(", ").unwrap())
        .tuples::<((&str, &str), (&str, &str), (&str, &str))>()
        .map(|(a, b, prize)| Machine {
            a: BigPos {
                x: a.0[2..].parse().unwrap(),
                y: a.1[2..].parse().unwrap(),
            },
            b: BigPos {
                x: b.0[2..].parse().unwrap(),
                y: b.1[2..].parse().unwrap(),
            },
            prize: BigPos {
                x: prize.0[2..].parse().unwrap(),
                y: prize.1[2..].parse().unwrap(),
            },
        })
        .collect_vec();

    Ok((part1(&machines)?, part2(&machines)?))
}

const A_COST: i64 = 3;
const B_COST: i64 = 1;

fn part1(machines: &[Machine]) -> PartResult {
    const MAX_PRESSES: i64 = 100;

    let spent_tokens = machines
        .iter()
        .filter_map(|machine| {
            calculate_ratios(
                machine.prize.x,
                machine.prize.y,
                machine.a.x,
                machine.a.y,
                machine.b.x,
                machine.b.y,
            )
        })
        .filter(|(a_times, b_times)| *a_times <= MAX_PRESSES && *b_times <= MAX_PRESSES)
        .map(|(a_times, b_times)| a_times * A_COST + b_times * B_COST)
        .sum::<i64>();

    Ok(spent_tokens.to_string())
}

fn calculate_ratios(px: i64, py: i64, ax: i64, ay: i64, bx: i64, by: i64) -> Option<(i64, i64)> {
    let a_dividend = (bx * py) - (by * px);
    let a_divisor = (ay * bx) - (ax * by);

    if a_dividend % a_divisor != 0 {
        return None;
    }

    let a_times = a_dividend / a_divisor;

    let b_dividend = px - (ax * a_times);
    let b_divisor = bx;

    if b_dividend % b_divisor != 0 {
        return None;
    }

    let b_times = b_dividend / b_divisor;
    Some((a_times, b_times))
}

fn part2(machines: &[Machine]) -> PartResult {
    let mut spent_tokens = 0;

    Ok(spent_tokens.to_string())
}

#[derive(Debug)]
struct Machine {
    a: BigPos,
    b: BigPos,
    prize: BigPos,
}

#[derive(Debug)]
struct BigPos {
    x: i64,
    y: i64,
}
