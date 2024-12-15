use itertools::Itertools;

use crate::utils::{
    boxdraw,
    direction::{self, Direction},
    not_yet_implemented,
    pos::{Index2d, Pos},
    wait_for_user,
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

    for move_dir in lines.join("").chars().map(|char| match char {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => panic!("Invalid direction to move!"),
    }) {
        let target = robot_pos + move_dir;

        // debug_show_state(&grid, robot_pos);
        // println!("Next Move: {:?}", move_dir);
        // wait_for_user();

        match grid.get_2d_unchecked(target) {
            Tile::Air => {
                robot_pos = target;
                continue;
            }
            Tile::Wall => {
                continue;
            }
            Tile::Box => (),
        }

        // 'ight, we've dealt with the easy paths, now we just care about the box moving logic.
        let Some(free_pos) = find_air_in_direction(&grid, target, move_dir) else {
            continue;
        };

        *grid.get_2d_mut_unchecked(free_pos) = Tile::Box;
        *grid.get_2d_mut_unchecked(target) = Tile::Air;

        robot_pos = target;
    }

    let gps_sum = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, tile)| match tile {
                    Tile::Air => None,
                    Tile::Wall => None,
                    Tile::Box => Some(x + (100 * y)),
                })
        })
        .sum::<usize>();

    Ok(gps_sum.to_string())
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

fn find_air_in_direction(
    grid: &Vec<Vec<Tile>>,
    start_pos: Pos,
    direction: Direction,
) -> Option<Pos> {
    for offset in 1..i32::MAX {
        let pos = start_pos + (Pos::from(direction) * offset);

        let Some(tile) = grid.get_2d(pos) else {
            break;
        };

        match tile {
            Tile::Air => return Some(pos),
            Tile::Wall => break,
            Tile::Box => continue,
        }
    }

    None
}

fn debug_show_state(grid: &[Vec<Tile>], robot_pos: Pos) {
    println!(
        "\n{}",
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
            .join("\n")
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
