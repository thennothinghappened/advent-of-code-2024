use std::error::Error;

use itertools::Itertools;

use super::{DayResult, PartResult};
use crate::utils::not_yet_implemented;

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let sum: usize = input
        .lines()
        .filter_map(|line| {
            let mut split = line.split(": ");

            let result: usize = split.next().unwrap().parse().unwrap();
            let operands: Vec<usize> = split
                .next()
                .unwrap()
                .split(" ")
                .map(str::parse)
                .try_collect()
                .unwrap();

            let num_op_bits = operands.len();
            let num_combos = 2_usize.pow(num_op_bits as u32);

            println!("-------------");
            println!(
                "Attempt :: Desired Result = {} from operands {:?} (#possible combos = {})",
                result, operands, num_combos
            );

            for combo in 0..num_combos {
                let mut sum = operands[0];

                for op_index in 0..num_op_bits - 1 {
                    if sum > result {
                        break;
                    }

                    let rhs = operands[op_index + 1];
                    let op = Op::extract_from(combo, op_index);

                    sum = op.perform(sum, rhs);
                }

                if sum == result {
                    println!(
                        "Success! :: Using combination {:?}",
                        (0..num_op_bits - 1)
                            .map(|op_index| Op::extract_from(combo, op_index))
                            .collect_vec()
                    );
                    return Some(result);
                }
            }

            None
        })
        .sum();

    Ok(sum.to_string())
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn perform(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
        }
    }

    fn extract_from(bits: usize, index: usize) -> Self {
        Op::from((bits >> index) & 1)
    }
}

impl From<usize> for Op {
    fn from(value: usize) -> Self {
        match value {
            0 => Op::Add,
            1 => Op::Mul,
            _ => panic!("Expected 1-bit number for operator!"),
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Mul => write!(f, "*"),
        }
    }
}
