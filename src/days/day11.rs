use super::{DayResult, PartResult};
use rustc_hash::FxHashMap;

pub(crate) fn solve(input: &str) -> DayResult {
    let mut stones = FxHashMap::<u64, u64>::default();

    input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .for_each(|stone| {
            stones
                .entry(stone)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    let result_part1 = part1(&mut stones)?;
    let result_part2 = part2(&mut stones)?;

    Ok((result_part1, result_part2))
}

fn part1(stones: &mut FxHashMap<u64, u64>) -> PartResult {
    for _ in 1..=25 {
        blink(stones);
    }

    Ok(stones.values().sum::<u64>().to_string())
}

fn part2(stones: &mut FxHashMap<u64, u64>) -> PartResult {
    for _ in 1..=50 {
        blink(stones);
    }

    Ok(stones.values().sum::<u64>().to_string())
}

fn split_halfway(stone: u64) -> (u64, u64) {
    let num_digits = stone.ilog10() + 1;
    let halfway = num_digits / 2;
    let divisor = 10_u64.pow(halfway);
    let high = stone / divisor;
    let low = stone - high * 10_u64.pow(halfway);

    (high, low)
}

fn blink(stones: &mut FxHashMap<u64, u64>) {
    // New method based on https://www.reddit.com/r/adventofcode/comments/1hbm0al/comment/m1hmola/
    // as it turns out to actually be cheaper to take the hit of these repeated allocations
    // than to have a continually growing cache (lots of reallocs!).
    //
    // Effectively we ditch recursion, because the order of the stones *does not matter*, so we can
    // bulk-apply each operation to `n` stones with the same outcome.

    let clone = stones.clone();
    stones.clear();

    for (stone, src_count) in clone.into_iter().filter(|(_, count)| *count > 0) {
        if stone == 0 {
            stones
                .entry(1)
                .and_modify(|count| *count += src_count)
                .or_insert(src_count);
            continue;
        }

        let num_digits = stone.ilog10() + 1;

        if num_digits % 2 != 0 {
            stones
                .entry(stone * 2024)
                .and_modify(|count| *count += src_count)
                .or_insert(src_count);
            continue;
        }

        let (high, low) = split_halfway(stone);

        stones
            .entry(high)
            .and_modify(|count| *count += src_count)
            .or_insert(src_count);
        stones
            .entry(low)
            .and_modify(|count| *count += src_count)
            .or_insert(src_count);
    }
}
