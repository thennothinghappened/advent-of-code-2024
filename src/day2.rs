use std::{char::MAX, error::Error, f32::MIN, ops::Deref};

use crate::not_yet_implemented;

const MIN_DEVIATION: i32 = 1;
const MAX_DEVIATION: i32 = 3;

pub(crate) fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let reports = input
        .lines()
        .map(str::split_whitespace)
        .map(|line| {
            line.map(|num| num.parse::<i32>().expect("Invalid number in line!"))
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let num_reports_ok = reports
        .into_iter()
        .filter_map(|report| {
            let mut iter = report.iter();
            let mut deviation_dir: Option<i32> = None;

            let Some(mut prev) = iter.next() else {
                return Some(report);
            };

            for curr in iter {
                let diff = curr - prev;

                let dir = match deviation_dir {
                    Some(dir) => dir,
                    None => {
                        let dir = diff.signum();
                        deviation_dir = Some(dir);

                        dir
                    }
                };

                if diff.signum() != dir {
                    return None;
                }

                if diff.abs() < MIN_DEVIATION || diff.abs() > MAX_DEVIATION {
                    return None;
                }

                prev = curr;
            }

            Some(report)
        })
        .count();

    Ok(num_reports_ok.to_string())
}

fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(not_yet_implemented())
}
