use itertools::Itertools;

use crate::utils::pos::FlatIndex;

use self::direction::{Direction, DIRECTIONS};

use super::{
    direction,
    pos::{Index2d, Pos},
};

const EMPTY: char = ' ';
const FILLED: char = '╳';

/// Draw the bounds of a shape within a grid, using the provided predicate to determine whether a
/// given position lies within the box, or not.
///
/// Unicode box-drawing characters are used for this task, and the result is a string that can be
/// printed directly.
///
/// # Safety
/// It is the responsibility of the caller to ensure that the given grid size matches the grid that
/// is being used as the input. If this is not true, the `within_box` predicate may be passed values
/// of positions exceeding the bounds of the underlying grid.
pub fn draw_shape_outline<F>(grid_width: usize, grid_height: usize, within_box: F) -> String
where
    F: Fn(Pos) -> bool,
{
    let not_within = |pos: Pos| -> bool {
        !pos.is_valid_grid_index(grid_width, grid_height) || !within_box(pos)
    };

    let mut outgrid = (0..grid_height * 3)
        .map(|_| (0..grid_width * 3).map(|_| EMPTY).collect_vec())
        .collect_vec();

    // Let's iterate forwards over the grid, until we find our first valid position. This will
    // represent a top-left corner.
    for y in 0..grid_height {
        for x in 0..grid_width {
            let pos = Pos::new_from_usize_unchecked(x, y);

            if !within_box(pos) {
                continue;
            }

            let outgrid_pos = pos * 3 + Pos::from(1).horizontal();
            *outgrid.get_2d_mut_unchecked(outgrid_pos) = FILLED;

            for direction in DIRECTIONS {
                let front_check_pos = pos + direction;
                let adjacent_check_pos = pos + direction.turned_right();
                let corner = Pos::from(direction) + direction.turned_right();

                *outgrid.get_2d_mut_unchecked(outgrid_pos + direction) =
                    match not_within(pos + direction) {
                        true => direction_edge_char(direction),
                        false => FILLED,
                    };

                let front_missing = not_within(front_check_pos);
                let adjacent_missing = not_within(adjacent_check_pos);

                *outgrid.get_2d_mut_unchecked(outgrid_pos + corner) =
                    match (front_missing, adjacent_missing) {
                        // Convex corner.
                        (true, true) => direction_corner_convex(direction),
                        // Concave corner.
                        (false, false) => {
                            if not_within(pos + corner) {
                                direction_corner_concave(direction)
                            } else {
                                FILLED
                            }
                        }
                        _ => {
                            *outgrid.get_2d_mut_unchecked(outgrid_pos + direction) =
                                match not_within(pos + direction) {
                                    true => direction_edge_char(direction),
                                    false => FILLED,
                                };
                            // TODO: fix this edge case. (heh)
                            match direction {
                                Direction::Up => '─',
                                Direction::Right => '─',
                                Direction::Down => '│',
                                Direction::Left => '│',
                            }
                        }
                    };
            }
        }
    }

    let header = "═".repeat(grid_width * 3);

    format!(
        "╔{}╗\n{}\n╚{}╝",
        header,
        outgrid
            .into_iter()
            .map(|row| format!("║{}║", row.into_iter().join("")))
            .join("\n"),
        header
    )
}

fn direction_edge_char(direction: Direction) -> char {
    match direction {
        Direction::Up => '─',
        Direction::Right => '│',
        Direction::Down => '─',
        Direction::Left => '│',
    }
}

fn direction_corner_convex(direction: Direction) -> char {
    match direction {
        Direction::Up => '┐',
        Direction::Right => '┘',
        Direction::Down => '└',
        Direction::Left => '┌',
    }
}

fn direction_corner_concave(direction: Direction) -> char {
    direction_corner_convex(direction.opposite())
}

#[test]
fn test_shape_outline() {
    #[rustfmt::skip]
    const GRID: &[u8] = &[
        0,0,0,0,0,
        0,1,1,1,0,
        0,0,1,0,1,
        0,1,1,1,1,
        0,0,1,1,1,
    ];

    const GRID_WIDTH: usize = 5;
    const GRID_HEIGHT: usize = 5;

    #[rustfmt::skip]
    const EXPECTED: &str = 
"...............
...............
...............
...┌───────┐...
...│       │...
...└──┐ ┌──┘...
";

    let output = draw_shape_outline(GRID_WIDTH, GRID_HEIGHT, |pos| {
        *GRID.flat_index(GRID_WIDTH, pos) == 1
    });

    println!("{}", output);

    // assert_eq!(output, EXPECTED);
}
