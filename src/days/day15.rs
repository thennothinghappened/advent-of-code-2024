use itertools::Itertools;

use crate::utils::{
    boxdraw,
    direction::Direction,
    not_yet_implemented,
    pos::{Index2d, Pos},
};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let mut robot_pos = Pos { x: 0, y: 0 };

    let mut lines = input.lines();
    let mut grid = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => Tile::Air,
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => {
                        robot_pos = Pos::new_from_usize_unchecked(x, y);
                        Tile::Air
                    }
                    _ => panic!("Invalid char in grid!"),
                })
                .collect_vec()
        })
        .collect_vec();

    let moves = lines
        .next()
        .unwrap()
        .chars()
        .map(|char| match char {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Invalid direction to move!"),
        })
        .collect_vec();

    debug_show_state(&grid, &moves, robot_pos);

    not_yet_implemented()
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

fn debug_show_state(grid: &[Vec<Tile>], moves: &[Direction], robot_pos: Pos) {
    println!(
        "{}\nMoves: {:?}\n",
        grid.iter()
            .enumerate()
            .map(|(y, row)| row
                .iter()
                .enumerate()
                .map(
                    move |(x, tile)| match Pos::new_from_usize_unchecked(x, y) == robot_pos {
                        true => '@',
                        false => char::from(*tile),
                    }
                )
                .join(""))
            .join("\n"),
        moves
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Wall,
    Box,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Air => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
        }
    }
}
