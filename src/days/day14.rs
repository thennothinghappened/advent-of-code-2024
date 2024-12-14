use itertools::Itertools;

use crate::utils::{
    direction::Direction,
    not_yet_implemented,
    pos::{self, Pos},
};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

const GRID_DIMENSIONS: Pos = Pos { x: 101, y: 103 };

fn part1(input: &str) -> PartResult {
    const STEPS: i32 = 100;

    let robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|part| {
                    part[2..]
                        .split(",")
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .map(Pos::from)
                .collect_tuple::<(Pos, Pos)>()
                .unwrap()
                .into()
        })
        .collect_vec();

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in robots {
        let forecast_pos = robot.move_steps(STEPS, GRID_DIMENSIONS);

        let Some(quadrant) = Quadrant::of_pos(forecast_pos, GRID_DIMENSIONS) else {
            continue;
        };

        *match quadrant {
            Quadrant::TopLeft => &mut top_left,
            Quadrant::TopRight => &mut top_right,
            Quadrant::BottomLeft => &mut bottom_left,
            Quadrant::BottomRight => &mut bottom_right,
        } += 1;
    }

    let safety_factor = top_left * top_right * bottom_left * bottom_right;
    Ok(safety_factor.to_string())
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Pos,
    velocity: Pos,
}

impl Robot {
    fn move_steps(&self, steps: i32, grid_dimensions: Pos) -> Pos {
        let raw_displacement = self.velocity * steps;

        // I am going to detonate whoever decided that `%` should not be the wrapping version.
        // This cost me literal hours. A use of `%` instead of `.rem_euclid()`.
        Pos {
            x: (self.pos.x + raw_displacement.x).rem_euclid(grid_dimensions.x),
            y: (self.pos.y + raw_displacement.y).rem_euclid(grid_dimensions.y),
        }
    }
}

#[test]
fn test_robot_move_forecast() {
    let grid_dimensions = Pos { x: 11, y: 7 };

    let robot = Robot {
        pos: Pos { x: 2, y: 4 },
        velocity: Pos { x: 2, y: -3 },
    };

    assert_eq!(robot.move_steps(0, grid_dimensions), Pos { x: 2, y: 4 });
    assert_eq!(robot.move_steps(1, grid_dimensions), Pos { x: 4, y: 1 });
    assert_eq!(robot.move_steps(2, grid_dimensions), Pos { x: 6, y: 5 });
    assert_eq!(robot.move_steps(3, grid_dimensions), Pos { x: 8, y: 2 });
    assert_eq!(robot.move_steps(4, grid_dimensions), Pos { x: 10, y: 6 });
    assert_eq!(robot.move_steps(5, grid_dimensions), Pos { x: 1, y: 3 });
}

impl From<(Pos, Pos)> for Robot {
    fn from((pos, velocity): (Pos, Pos)) -> Self {
        Robot { pos, velocity }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    fn of_pos(pos: Pos, grid_dimensions: Pos) -> Option<Self> {
        let halfway = grid_dimensions / 2;

        let is_left: bool = match pos.x.cmp(&halfway.x) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => {
                if grid_dimensions.x % 2 == 0 {
                    false
                } else {
                    return None;
                }
            }
            std::cmp::Ordering::Greater => false,
        };

        let is_top: bool = match pos.y.cmp(&halfway.y) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => {
                if grid_dimensions.y % 2 == 0 {
                    false
                } else {
                    return None;
                }
            }
            std::cmp::Ordering::Greater => false,
        };

        Some(match (is_left, is_top) {
            (true, true) => Quadrant::TopLeft,
            (true, false) => Quadrant::BottomLeft,
            (false, true) => Quadrant::TopRight,
            (false, false) => Quadrant::BottomRight,
        })
    }
}

#[test]
fn test_quadrant_sorting() {
    let grid_3x3 = Pos { x: 3, y: 3 };
    let grid_4x4 = Pos { x: 4, y: 4 };

    assert_eq!(
        Quadrant::of_pos(Pos { x: 0, y: 0 }, grid_3x3),
        Some(Quadrant::TopLeft)
    );

    assert_eq!(
        Quadrant::of_pos(Pos { x: 2, y: 2 }, grid_3x3),
        Some(Quadrant::BottomRight)
    );

    assert_eq!(
        Quadrant::of_pos(Pos { x: 2, y: 0 }, grid_3x3),
        Some(Quadrant::TopRight)
    );

    assert_eq!(
        Quadrant::of_pos(Pos { x: 0, y: 2 }, grid_3x3),
        Some(Quadrant::BottomLeft)
    );

    assert_eq!(Quadrant::of_pos(Pos { x: 1, y: 1 }, grid_3x3), None);

    assert_eq!(
        Quadrant::of_pos(Pos { x: 1, y: 1 }, grid_4x4),
        Some(Quadrant::TopLeft)
    );
}
