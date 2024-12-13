use itertools::Itertools;

use crate::utils::{not_yet_implemented, pos::Pos};

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

const A_COST: i32 = 3;
const B_COST: i32 = 1;
const MAX_PRESSES: i32 = 100;

fn part1(input: &str) -> PartResult {
    let machines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(": ").unwrap().1.split_once(", ").unwrap())
        .tuples::<((&str, &str), (&str, &str), (&str, &str))>()
        .map(|(a, b, prize)| Machine {
            a: Pos {
                x: a.0[2..].parse().unwrap(),
                y: a.1[2..].parse().unwrap(),
            },
            b: Pos {
                x: b.0[2..].parse().unwrap(),
                y: b.1[2..].parse().unwrap(),
            },
            prize: Pos {
                x: prize.0[2..].parse().unwrap(),
                y: prize.1[2..].parse().unwrap(),
            },
        });

    let mut spent_tokens = 0;

    for machine in machines {
        let mut cheapest = (i32::MAX, i32::MAX);

        for a_presses in 0..=MAX_PRESSES {
            for b_presses in 0..=MAX_PRESSES {
                if (machine.a * a_presses + machine.b * b_presses) == machine.prize {
                    cheapest.0 = a_presses;
                    cheapest.1 = b_presses;
                    break;
                }
            }
        }

        if cheapest.0 <= MAX_PRESSES && cheapest.1 <= MAX_PRESSES {
            spent_tokens += cheapest.0 * A_COST + cheapest.1 * B_COST;
        //     println!(
        //         "{:?} :: Cheapest solution costs {} tokens.",
        //         machine,
        //         cheapest.0 * A_COST + cheapest.1 * B_COST
        //     );
        } else {
            //     println!("{:?} :: No solutions found.", machine);
        }
    }

    Ok(spent_tokens.to_string())
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

#[derive(Debug)]
struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}
