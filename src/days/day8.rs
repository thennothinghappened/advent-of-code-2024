use std::{collections::HashMap, ops::DerefMut};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::utils::{not_yet_implemented, pos::Pos};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let grid_width = input.lines().next().unwrap().len();
    let grid_height = input.lines().count();

    let antenna_types = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, chr)| {
                if chr == '.' {
                    return None;
                }

                Some((
                    chr,
                    Pos {
                        x: x as i32,
                        y: y as i32,
                    },
                ))
            })
        })
        .into_group_map();

    println!(
        "{:#?}\n\nGrid :: {}x{}",
        antenna_types, grid_width, grid_height
    );

    let mut antinodes = FxHashSet::<Pos>::default();

    for (char, antennas) in antenna_types.iter() {
        for i in 0..antennas.len() {
            let src_antenna = antennas[i];
            for dest_antenna in (0..antennas.len()).filter(|j| *j != i).map(|j| antennas[j]) {
                let antinode = src_antenna + (dest_antenna - src_antenna) * 2.into();

                if antinode.is_positive()
                    && (antinode.x as usize) < grid_width
                    && (antinode.y as usize) < grid_height
                {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    println!(
        "{}",
        (0..grid_height)
            .map(|y| (0..grid_width)
                .map(|x| {
                    let pos = Pos {
                        x: x as i32,
                        y: y as i32,
                    };

                    if antinodes.contains(&pos) {
                        return &'#';
                    }

                    for (char, antennas) in antenna_types.iter() {
                        if antennas.contains(&pos) {
                            return &char;
                        }
                    }

                    &'.'
                })
                .join(""))
            .join("\n")
    );

    Ok(antinodes.len().to_string())
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}
