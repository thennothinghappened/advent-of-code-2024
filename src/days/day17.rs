use itertools::Itertools;

use crate::utils::not_yet_implemented;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let mut lines = input.lines();

    let mut a: i32 = lines.next().unwrap()[12..].parse().unwrap();
    let mut b: i32 = lines.next().unwrap()[12..].parse().unwrap();
    let mut c: i32 = lines.next().unwrap()[12..].parse().unwrap();

    lines.next();

    for (opcode, operand) in lines.next().unwrap()[9..]
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple().unwrap())
    {
        println!("Opcode {} with operand {}", opcode, operand);
    }

    not_yet_implemented()
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}
