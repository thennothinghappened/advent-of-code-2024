use itertools::Itertools;

use crate::utils::{
    direction::DIRECTIONS,
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
    // Like part 1, but instead of perimeter area, we want *edges*.
    // uhhhhh........................................
    //
    // okay so part 1 has no sense of continuity right
    //
    // okay also
    // if we know we're starting at the top-most left-most position that matches the shape,
    // then we know that all of the members of the shape must be either to the right, or below
    // this one.
    //
    // we can also say that every row, in isolation, always has 4 sides.
    // the number of sides a row has is affected by the number of disjoints it has.
    // i.e., the number of times it sees a connected piece above or below, where there is not ALSO
    // a connected piece adjacent to that.

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

            // corners = edges!
            let mut corners = 0;
            let mut area = 0;

            define_region_p2(&mut grid, plant.species, pos, &mut corners, &mut area);

            sum += area * corners;
        }
    }

    Ok(sum.to_string())
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

fn define_region_p2(
    grid: &mut Vec<Vec<Plant>>,
    species: char,
    pos: Pos,
    corners: &mut u64,
    area: &mut u64,
) {
    const CORNER_DIRECTIONS: [Pos; 4] = [
        Pos { x: -1, y: -1 }, // Top-left.
        Pos { x: 1, y: -1 },  // Top-right.
        Pos { x: -1, y: 1 },  // Bottom-left.
        Pos { x: 1, y: 1 },   // Bottom-right.
    ];

    let Some(plant) = grid.get_2d_mut(pos) else {
        return;
    };

    if plant.seen || plant.species != species {
        return;
    }

    plant.seen = true;
    *area += 1;

    for corner in CORNER_DIRECTIONS {
        let vert_pos = pos + corner.vertical();
        let hor_pos = pos + corner.horizontal();

        let vert_missing = grid
            .get_2d(vert_pos)
            .map(|plant| plant.species != species)
            .unwrap_or(true);

        let hor_missing = grid
            .get_2d(hor_pos)
            .map(|plant| plant.species != species)
            .unwrap_or(true);

        match (vert_missing, hor_missing) {
            (true, true) => {
                // Convex corner.
                *corners += 1;
            }
            (false, false) => {
                // Concave corner.
                if grid
                    .get_2d(pos + corner)
                    .map(|plant| plant.species != species)
                    .unwrap_or(true)
                {
                    *corners += 1;
                }
            }
            _ => {}
        }

        define_region_p2(grid, species, vert_pos, corners, area);
        define_region_p2(grid, species, hor_pos, corners, area);
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
