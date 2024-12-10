use std::{iter, vec};

use itertools::Itertools;
use rayon::iter::empty;
use rustc_hash::FxHashSet;

use crate::utils::{
    direction::DIRECTIONS,
    not_yet_implemented,
    pos::{Index2d, Pos},
};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut seen_peaks = FxHashSet::<Pos>::default();
    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, col)| **col == 0) {
            let start_pos = Pos::new_from_usize_unchecked(x, y);
            let peaks = find_peaks(&grid, start_pos, 0);

            seen_peaks.clear();

            for peak in peaks {
                seen_peaks.insert(peak);
            }

            sum += seen_peaks.len();
        }
    }

    Ok(sum.to_string())
}

fn find_peaks(grid: &Vec<Vec<u32>>, from_pos: Pos, from_height: u32) -> Vec<Pos> {
    DIRECTIONS
        .iter()
        .filter_map(move |&direction| {
            let pos = from_pos + direction;
            let height = grid.get_2d(pos)?;

            if *height - 1 != from_height {
                return None;
            }

            if *height == 9 {
                return Some(vec![pos]);
            }

            Some(find_peaks(grid, pos, *height))
        })
        .flatten()
        .collect()
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}
