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
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(index, count)| {
            let block = match index % 2 {
                0 => Some(index / 2),
                _ => None,
            };

            vec![block; count as usize]
        })
        .collect();

    print_fs(&fs);

    while let Some(free_index) = fs.iter().position(|block| block.is_none()) {
        let Some(swap_block_index) = fs
            .iter()
            .enumerate()
            .rev()
            .find(|(_, block)| block.is_some())
            .map(|(index, _)| index)
        else {
            break;
        };

        if swap_block_index < free_index {
            break;
        }

        fs.swap(swap_block_index, free_index);
    }

    print_fs(&fs);

    Ok(fs
        .iter()
        .enumerate()
        .filter_map(|(index, block)| block.map(|id| index * id))
        .sum::<usize>()
        .to_string())
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

fn print_fs(fs: &[Option<usize>]) {
    println!(
        "{}",
        fs.iter()
            .map(|block| match block {
                Some(index) => index.to_string(),
                None => String::from("."),
            })
            .join("")
    );
}
