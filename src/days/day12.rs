use itertools::Itertools;

use crate::utils::{
    direction::{Direction, DIRECTIONS},
    not_yet_implemented,
    pos::{Index2d, Pos},
};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(Plant::new).collect_vec())
        .collect_vec();

    let mut sum: u64 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let pos = Pos::new_from_usize_unchecked(x, y);
            let plant = *grid.get_2d_unchecked(pos);

            if plant.seen {
                continue;
            }

            let mut perimeter = 0;
            let mut area = 0;

            define_region(&mut grid, plant.species, pos, &mut perimeter, &mut area);

            sum += area * perimeter;
        }
    }

    Ok(sum.to_string())
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

fn define_region(
    grid: &mut Vec<Vec<Plant>>,
    species: char,
    pos: Pos,
    perimeter: &mut u64,
    area: &mut u64,
) {
    let Some(plant) = grid.get_2d_mut(pos) else {
        *perimeter += 1;
        return;
    };

    if plant.species != species {
        *perimeter += 1;
        return;
    }

    if plant.seen {
        return;
    }

    *area += 1;
    plant.seen = true;

    for direction in DIRECTIONS {
        define_region(grid, species, pos + direction, perimeter, area);
    }
}

#[derive(Clone, Copy)]
struct Plant {
    species: char,
    seen: bool,
}

impl Plant {
    fn new(species: char) -> Self {
        Plant {
            species,
            seen: false,
        }
    }
}
