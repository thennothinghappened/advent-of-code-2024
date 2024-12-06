use std::collections::HashMap;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    // 1. Create two arrays.
    // 2. Iterate over each line of input, putting LHS of whitespace into arr1, RHS into arr2 (converted to numbers.)
    // 3. Get the smallest of both arrays, += the absolute difference to output.
    // 4. Print output.

    let mut arr1 = Vec::<i32>::new();
    let mut arr2 = Vec::<i32>::new();
    let mut sum = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let lhs = split.next().unwrap().parse::<i32>()?;
        let rhs = split.next().unwrap().parse::<i32>()?;

        arr1.push(lhs);
        arr2.push(rhs);
    }

    arr1.sort();
    arr2.sort();

    for (index, lhs) in arr1.iter().enumerate() {
        let rhs = arr2[index];
        sum += lhs.abs_diff(rhs);
    }

    Ok(sum.to_string())
}

fn part2(input: &str) -> PartResult {
    let mut left_occurrences: HashMap<usize, usize> = HashMap::new();
    let mut right_occurrences: HashMap<usize, usize> = HashMap::new();
    let (left, right): (Vec<usize>, Vec<usize>) = input.lines().map(parse_line).unzip();

    left.iter().for_each(|value| {
        left_occurrences
            .entry(*value)
            .and_modify(|value| *value += 1)
            .or_insert(1);
        right_occurrences.insert(*value, 0);
    });

    right.iter().for_each(|value| {
        right_occurrences.entry(*value).and_modify(|occurrences| {
            *occurrences += 1;
        });
    });

    let mut sum: usize = 0;

    right_occurrences
        .into_iter()
        .map(|(value, occurrences)| (value, occurrences * left_occurrences.get(&value).unwrap()))
        .for_each(|(value, occurrences)| {
            sum += value * occurrences;
        });

    Ok(sum.to_string())
}

fn parse_line(line: &str) -> (usize, usize) {
    let mut split = line.split_ascii_whitespace();
    let lhs = split
        .next()
        .expect("Line should have two members.")
        .parse()
        .expect("Line members should be numerical.");
    let rhs = split
        .next()
        .expect("Line should have two members.")
        .parse()
        .expect("Line members should be numerical.");

    (lhs, rhs)
}
