use std::{char::MAX, cmp, error::Error, f32::MIN, ops::Deref};

use crate::not_yet_implemented;

const MIN_DEVIATION: i32 = 1;
const MAX_DEVIATION: i32 = 3;

pub(crate) fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let num_reports_ok = input
        .lines()
        .map(str::split_whitespace)
        .map(|line| {
            line.map(|num| num.parse::<i32>().expect("Invalid number in line!"))
                .collect::<Vec<i32>>()
        })
        .into_iter()
        .filter(|report| evaluate_report(&report))
        .count();

    Ok(num_reports_ok.to_string())
}

fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let num_reports_ok = input
        .lines()
        .map(str::split_whitespace)
        .map(|line| {
            line.map(|num| num.parse::<i32>().expect("Invalid number in line!"))
                .collect::<Vec<i32>>()
        })
        .into_iter()
        .filter(|report| {
			
			if evaluate_report(&report) {
				return true;
			}
			
			// fuck it, brute force it, i'm quite lost lol.
			let mut mutable_report = report.clone();

			for i in 0..report.len() {

				let entry = mutable_report.remove(i);

				if evaluate_report(&mutable_report) {
					return true;
				}

				mutable_report.insert(i, entry);

			}

			return false;

        })
        .count();

    Ok(num_reports_ok.to_string())
}

fn evaluate_report(report: &Vec<i32>) -> bool {
    let mut dir: i32 = 0;

    for i in 1..report.len() {
        let prev = report[i - 1];
        let curr = report[i];

        let diff = curr - prev;
        let diff_sign = diff.signum();

        dir += diff_sign;
    }

    dir = dir.signum();

    for i in 1..report.len() {
        let prev = report[i - 1];
        let curr = report[i];

        let diff = curr - prev;
        let diff_sign = diff.signum();
        let diff_abs = diff.abs();

        if diff_sign != dir {
            return false;
        }

        if diff_abs < MIN_DEVIATION || diff_abs > MAX_DEVIATION {
            return false;
        }
    }

    true
}
