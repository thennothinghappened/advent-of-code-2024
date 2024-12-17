use itertools::Itertools;

use crate::utils::{
    boxdraw, not_yet_implemented,
    pos::{Index2d, Pos},
};

use super::{DayResult, PartResult};

// TODO: Get back to this one! This is hard!
pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let mut start_pos = Pos::from(0);
    let mut exit_pos = Pos::from(0);

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => Tile::Air,
                    '#' => Tile::Wall,
                    'S' => {
                        start_pos = Pos::new_from_usize_unchecked(x, y);
                        Tile::Air
                    }
                    'E' => {
                        exit_pos = Pos::new_from_usize_unchecked(x, y);
                        Tile::Air
                    }
                    _ => panic!("Invalid char in grid!"),
                })
                .collect_vec()
        })
        .collect_vec();

    println!(
        "{}",
        boxdraw::draw_shape(grid[0].len(), grid.len(), |pos| {
            match grid.get_2d_unchecked(pos) {
                Tile::Air => false,
                Tile::Wall => true,
            }
        })
    );

    not_yet_implemented()
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

enum Tile {
    Air,
    Wall,
}
