use itertools::Itertools;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

use super::{DayResult, PartResult};
use crate::utils::not_yet_implemented;

pub(crate) fn solve(input: &str) -> DayResult {
    let equations: Vec<Equation> = input
        .par_lines()
        .map(|line| {
            let mut split = line.split(": ");

            let result: usize = split.next().unwrap().parse().unwrap();
            let operands: Vec<usize> = split
                .next()
                .unwrap()
                .split(" ")
                .map(str::parse)
                .try_collect()
                .unwrap();

            Equation { result, operands }
        })
        .collect();

    Ok((part1(&equations)?, part2(&equations)?))
}

fn part1(equations: &Vec<Equation>) -> PartResult {
    let sum: usize = equations
        .par_iter()
        .filter_map(|equation| {
            let num_op_bits = equation.operands.len();
            let num_combos = 2_usize.pow((num_op_bits - 1) as u32);

            // println!("-------------");
            // println!(
            //     "Attempt :: Desired Result = {} from operands {:?} (#possible combos = {})",
            //     equation.result, equation.operands, num_combos
            // );

            for combo in 0..num_combos {
                let mut sum = equation.operands[0];

                for op_index in 0..num_op_bits - 1 {
                    if sum > equation.result {
                        break;
                    }

                    let rhs = equation.operands[op_index + 1];
                    let op = Op::extract_from(combo, op_index);

                    sum = op.perform(sum, rhs);
                }

                if sum == equation.result {
                    // println!(
                    //     "Success! :: Using combination {:?}",
                    //     (0..num_op_bits - 1)
                    //         .map(|op_index| Op::extract_from(combo, op_index))
                    //         .collect_vec()
                    // );
                    return Some(equation.result);
                }
            }

            None
        })
        .sum();

    Ok(sum.to_string())
}

fn part2(equations: &Vec<Equation>) -> PartResult {
    let sum: usize = equations
        .iter()
        .filter_map(|equation| {
            let num_operators = equation.operands.len() - 1;
            let num_combos = 2_usize.pow(num_operators as u32);

            println!("-------------");
            println!(
                "Attempt :: Desired Result = {} from operands {:?} (#possible combos = {})",
                equation.result, equation.operands, num_combos
            );

            for combo in (0..num_operators)
                .map(|_| [Op::Add, Op::Mul, Op::Concat])
                .multi_cartesian_product()
            {
                let mut sum = equation.operands[0];

                for op_index in 0..num_operators {
                    if sum > equation.result {
                        break;
                    }

                    let rhs = equation.operands[op_index + 1];
                    let op = combo[op_index];

                    sum = op.perform(sum, rhs);
                }

                if sum == equation.result {
                    println!("Success! :: Using combination {:?}", combo);
                    return Some(equation.result);
                }
            }

            None
        })
        .sum();

    Ok(sum.to_string())
}

struct Equation {
    result: usize,
    operands: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn perform(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
            Op::Concat => {
                let num_digits_rhs = rhs.checked_ilog10().unwrap_or(0) + 1;
                lhs * 10_usize.pow(num_digits_rhs) + rhs
            }
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
