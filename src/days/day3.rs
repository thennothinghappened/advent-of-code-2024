use std::error::Error;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let mut rem_input = input;
    let mut sum = 0;

    loop {
        let Some(next_match) = rem_input.find("mul(") else {
            break;
        };

        rem_input = &rem_input[next_match + 4..];

        let Some(comma) = rem_input.find(",") else {
            continue;
        };

        let Some(closing) = rem_input.find(")") else {
            continue;
        };

        if comma >= closing {
            continue;
        }

        let Ok(lhs) = rem_input[..comma].parse::<i32>() else {
            continue;
        };

        let Ok(rhs) = rem_input[comma + 1..closing].parse::<i32>() else {
            continue;
        };

        sum += lhs * rhs;
    }

    Ok(sum.to_string())
}

fn part2(input: &str) -> PartResult {
    let mut rem_input = input;
    let mut bother = true;
    let mut sum = 0;

    loop {
        let Some(next_match) = rem_input.find("mul(") else {
            break;
        };

        match bother {
            true => {
                if let Some(next_dont) = rem_input.find("don't()") {
                    if next_dont < next_match {
                        bother = false;
                    }
                }
            }
            false => {
                if let Some(next_do) = rem_input.find("do()") {
                    if next_do < next_match {
                        bother = true;
                    }
                }
            }
        }

        rem_input = &rem_input[next_match + 4..];

        if !bother {
            continue;
        }

        let Some(comma) = rem_input.find(",") else {
            continue;
        };

        let Some(closing) = rem_input.find(")") else {
            continue;
        };

        if comma >= closing {
            continue;
        }

        let Ok(lhs) = rem_input[..comma].parse::<i32>() else {
            continue;
        };

        let Ok(rhs) = rem_input[comma + 1..closing].parse::<i32>() else {
            continue;
        };

        sum += lhs * rhs;
    }

    Ok(sum.to_string())
}
