use std::iter::{once, Once};

use itertools::{Either, Itertools};

use crate::utils::not_yet_implemented;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    // LMFAO I'll sort this out later.
    let num_stones = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .flat_map(|stone| blink(stone).into_iter())
        .count();

    Ok(num_stones.to_string())
}

fn blink(stone: u64) -> Either<Once<u64>, [u64; 2]> {
    if stone == 0 {
        return Either::Left(once(1));
    }

    let num_digits = stone.ilog10() + 1;

    if num_digits % 2 != 0 {
        return Either::Left(once(stone * 2024));
    }

    Either::Right(split_halfway(stone))
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

fn split_halfway(stone: u64) -> [u64; 2] {
    debug_assert_eq!(stone % 2, 0);

    let num_digits = stone.ilog10() + 1;
    let halfway = num_digits / 2;
    let divisor = 10_u64.pow(halfway);
    let high = stone / divisor;
    let low = stone - high * 10_u64.pow(halfway);

    [high, low]
}

#[test]
fn blink_works() {
    {
        let stone = 0;
        assert_eq!(blink(stone).unwrap_left().next().unwrap(), 1);
    }

    {
        let stone = 123;
        assert_eq!(blink(stone).unwrap_left().next().unwrap(), 123 * 2024);
    }

    {
        let stone = 2048;
        let [high, low] = blink(stone).unwrap_right();

        assert_eq!(high, 20);
        assert_eq!(low, 48);
    }
}

#[test]
fn try_splitting_halfway() {
    let stone = 2048;
    let [high, low] = split_halfway(stone);

    assert_eq!(high, 20);
    assert_eq!(low, 48);
}
