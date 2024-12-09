use itertools::Itertools;

use crate::utils::not_yet_implemented;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    // 1. Parse input into mut Vec<Option<usize>>, where the usize represents the ID (incrementing).
    // 2. Determine the first free space.
    // 3. Iterate backwards, moving each block to the free space, and finding the next free space.
    // 4. Checksum is given by sum of enumerating each block * their ID.

    let mut fs: Vec<Option<usize>> = input
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(index, count)| {
            let block_type = match index % 2 {
                0 => Some(index / 2),
                _ => None,
            };

            vec![block_type; count as usize]
        })
        .collect();

    println!(
        "{}",
        fs.iter()
            .map(|block| match block {
                Some(index) => index.to_string(),
                None => String::from("."),
            })
            .join("")
    );

    not_yet_implemented()
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}
