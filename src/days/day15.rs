use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::utils::{
    direction::Direction,
    pos::{Index2d, Pos},
};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    let mut robot_pos = Pos { x: 0, y: 0 };

    let mut lines = input.lines();
    let mut p1_grid = lines
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

    let mut p2_grid = p1_grid
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|tile| match tile {
                    Tile::Air => [DWTile::Air, DWTile::Air],
                    Tile::Wall => [DWTile::Wall, DWTile::Wall],
                    Tile::Box => [DWTile::BoxLeft, DWTile::BoxRight],
                })
                .collect_vec()
        })
        .collect_vec();

    let moves = lines
        .join("")
        .chars()
        .map(|char| match char {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Invalid direction to move!"),
        })
        .collect_vec();

    Ok((
        part1(&mut p1_grid, &moves, robot_pos)?,
        part2(&mut p2_grid, &moves, robot_pos)?,
    ))
}

fn part1(grid: &mut Vec<Vec<Tile>>, moves: &[Direction], robot_initial_pos: Pos) -> PartResult {
    let mut robot_pos = robot_initial_pos;

    for &move_dir in moves {
        let target = robot_pos + move_dir;

        // debug_show_state(grid, robot_pos);
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
        let Some(free_pos) = find_air_in_direction(grid, target, move_dir) else {
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

fn part2(grid: &mut Vec<Vec<DWTile>>, moves: &[Direction], robot_initial_pos: Pos) -> PartResult {
    let mut robot_pos = robot_initial_pos;

    'moves_loop: for &move_dir in moves {
        let target = robot_pos + move_dir;
        let target_tile = *grid.get_2d_unchecked(target);

        match target_tile {
            DWTile::Air => {
                robot_pos = target;
                continue;
            }
            DWTile::Wall => {
                continue;
            }
            _ => (),
        }

        if move_dir.is_horizontal() {
            // we've gotta actually move the whole row across this time!
            let Some(free) = p2_find_air_horizontal(grid, target, move_dir) else {
                continue;
            };

            *grid.get_2d_mut_unchecked(target) = DWTile::Air;
            *grid.get_2d_mut_unchecked(free) = if move_dir == Direction::Left {
                DWTile::BoxLeft
            } else {
                DWTile::BoxRight
            };

            for x in if free.x > target.x {
                (target.x + 1)..free.x
            } else {
                (free.x + 1)..target.x
            } {
                let edit_pos = Pos { x, y: target.y };
                *grid.get_2d_mut_unchecked(edit_pos) = match grid.get_2d_unchecked(edit_pos) {
                    DWTile::BoxLeft => DWTile::BoxRight,
                    DWTile::BoxRight => DWTile::BoxLeft,
                    _ => unreachable!(),
                };
            }

            robot_pos = target;
            continue;
        }

        // shenanigans afoot!

        // 1. Discover all the boxes in our way (a tree!)
        // 2. For the final lot on each branch, ensure we can move 'em.
        // 3. Starting from the finals and working backwards, move each box to its new position.

        // List of x-positions of the boxes to be moved. A new row is appended for each y
        // increment away from the target.
        let mut box_positions = Vec::<FxHashSet<i32>>::new();

        box_positions.push({
            let mut next: FxHashSet<i32> = FxHashSet::default();

            next.insert(target.x);
            next.insert(match target_tile {
                DWTile::BoxLeft => target.x + 1,
                DWTile::BoxRight => target.x - 1,
                _ => unreachable!(),
            });

            next
        });

        loop {
            let y = target.y + (Pos::from(move_dir).y * box_positions.len() as i32);
            let prev = box_positions.last().unwrap();
            let mut next: FxHashSet<i32> = FxHashSet::default();

            if y == 0 || y == grid.len() as i32 {
                continue 'moves_loop;
            }

            for &x in prev {
                let ahead = Pos { x, y };
                let ahead_tile = grid.get_2d_unchecked(ahead);

                match ahead_tile {
                    DWTile::Air => continue,
                    DWTile::Wall => continue 'moves_loop,
                    DWTile::BoxLeft => next.insert(x + 1),
                    DWTile::BoxRight => next.insert(x - 1),
                };

                next.insert(x);
            }

            if next.is_empty() {
                break;
            }

            box_positions.push(next);
        }

        for (y_offset, row) in box_positions.iter().enumerate().rev() {
            let offset_direction: i32 = Pos::from(move_dir).y;
            let y = target.y + (y_offset as i32) * offset_direction;

            for &x in row {
                let src_pos = Pos { x, y };
                let dest_pos = src_pos + move_dir;

                grid.swap_2d_unchecked(src_pos, dest_pos);
            }
        }

        robot_pos = target;
    }

    let gps_sum = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, tile)| match tile {
                    DWTile::BoxLeft => Some(x + (100 * y)),
                    _ => None,
                })
        })
        .sum::<usize>();

    Ok(gps_sum.to_string())
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

fn p2_find_air_horizontal(
    grid: &Vec<Vec<DWTile>>,
    start_pos: Pos,
    direction: Direction,
) -> Option<Pos> {
    assert!(direction.is_horizontal());

    for offset in 1..i32::MAX {
        let pos = start_pos + (Pos::from(direction) * offset);

        let Some(tile) = grid.get_2d(pos) else {
            break;
        };

        match tile {
            DWTile::Air => return Some(pos),
            DWTile::Wall => break,
            _ => continue,
        }
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Wall,
    Box,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DWTile {
    Air,
    Wall,
    BoxLeft,
    BoxRight,
}
