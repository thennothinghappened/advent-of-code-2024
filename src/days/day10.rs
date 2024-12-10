use std::vec;

use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::utils::{
    direction::DIRECTIONS,
    pos::{Index2d, Pos},
};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    Ok((part1(&grid)?, part2(&grid)?))
}

fn part1(grid: &Vec<Vec<u32>>) -> PartResult {
    let mut seen_peaks = FxHashSet::<Pos>::default();
    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, col)| **col == 0) {
            let start_pos = Pos::new_from_usize_unchecked(x, y);
            let peaks = find_peaks(grid, start_pos, 0);

            seen_peaks.clear();

            for peak in peaks {
                seen_peaks.insert(peak);
            }

            sum += seen_peaks.len();
        }
    }

    Ok(sum.to_string())
}

fn part2(grid: &Vec<Vec<u32>>) -> PartResult {
    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, col)| **col == 0) {
            let start_pos = Pos::new_from_usize_unchecked(x, y);
            sum += rate(grid, start_pos, 0);
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

fn rate(grid: &Vec<Vec<u32>>, from_pos: Pos, from_height: u32) -> usize {
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
        .count()
}
