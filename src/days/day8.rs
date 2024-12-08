use std::collections::HashMap;

use itertools::Itertools;
use rustc_hash::FxHashSet;

use super::{DayResult, PartResult};
use crate::utils::pos::Pos;

pub(crate) fn solve(input: &str) -> DayResult {
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

    Ok((
        part1(&antenna_types, grid_width, grid_height)?,
        part2(&antenna_types, grid_width, grid_height)?,
    ))
}

fn part1(
    antenna_types: &HashMap<char, Vec<Pos>>,
    grid_width: usize,
    grid_height: usize,
) -> PartResult {
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

    Ok(antinodes.len().to_string())
}

fn part2(
    antenna_types: &HashMap<char, Vec<Pos>>,
    grid_width: usize,
    grid_height: usize,
) -> PartResult {
    let mut antinodes = FxHashSet::<Pos>::default();

    for (char, antennas) in antenna_types.iter() {
        for i in 0..antennas.len() {
            let src_antenna = antennas[i];
            for dest_antenna in (0..antennas.len()).filter(|j| *j != i).map(|j| antennas[j]) {
                let mut offset = 1;

                loop {
                    let antinode = src_antenna + (dest_antenna - src_antenna) * offset.into();

                    if !antinode.is_positive()
                        || (antinode.x as usize) >= grid_width
                        || (antinode.y as usize) >= grid_height
                    {
                        break;
                    }

                    antinodes.insert(antinode);
                    offset += 1;
                }
            }
        }
    }

    Ok(antinodes.len().to_string())
}
