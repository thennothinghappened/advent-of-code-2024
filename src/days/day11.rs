use super::{DayResult, PartResult};
use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap};

pub(crate) fn solve(input: &str) -> DayResult {
    let mut cache = FxHashMap::<(u64, u64), u64>::with_capacity_and_hasher(120_000, FxBuildHasher);
    let stones = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect_vec();

    let result_part1 = part1(&stones, &mut cache)?;
    let result_part2 = part2(&stones, &mut cache)?;

    Ok((result_part1, result_part2))
}

fn part1(stones: &[u64], cache: &mut FxHashMap<(u64, u64), u64>) -> PartResult {
    let num_stones = stones
        .iter()
        .map(|&stone| blink(cache, stone, 25))
        .sum::<u64>();

    Ok(num_stones.to_string())
}

fn part2(stones: &[u64], cache: &mut FxHashMap<(u64, u64), u64>) -> PartResult {
    let num_stones = stones
        .iter()
        .map(|&stone| blink(cache, stone, 75))
        .sum::<u64>();

    Ok(num_stones.to_string())
}

fn split_halfway(stone: u64) -> (u64, u64) {
    let num_digits = stone.ilog10() + 1;
    let halfway = num_digits / 2;
    let divisor = 10_u64.pow(halfway);
    let high = stone / divisor;
    let low = stone - high * 10_u64.pow(halfway);

    (high, low)
}

fn blink(cache: &mut FxHashMap<(u64, u64), u64>, stone: u64, iterations: u64) -> u64 {
    let cache_key = (stone, iterations);

    if let Some(cached_entry) = cache.get(&cache_key) {
        return *cached_entry;
    }

    if stone == 0 {
        if iterations == 1 {
            return 1;
        }

        let count = blink(cache, 1, iterations - 1);
        cache.insert(cache_key, count);

        return count;
    }

    let num_digits = stone.ilog10() + 1;

    if num_digits % 2 != 0 {
        if iterations == 1 {
            return 1;
        }

        let count = blink(cache, stone * 2024, iterations - 1);
        cache.insert(cache_key, count);

        return count;
    }

    if iterations == 1 {
        return 2;
    }

    let (high, low) = split_halfway(stone);
    let count = blink(cache, high, iterations - 1) + blink(cache, low, iterations - 1);

    cache.insert(cache_key, count);
    count
}
