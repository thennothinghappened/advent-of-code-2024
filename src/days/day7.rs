use itertools::Itertools;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

use super::{DayResult, PartResult};

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
        .filter(|equation| can_solve(&equation, &[Op::Add, Op::Mul]))
        .map(|equation| equation.result)
        .sum();

    Ok(sum.to_string())
}

fn part2(equations: &Vec<Equation>) -> PartResult {
    let sum: usize = equations
        .par_iter()
        .filter(|equation| can_solve(&equation, &[Op::Add, Op::Mul, Op::Concat]))
        .map(|equation| equation.result)
        .sum();

    Ok(sum.to_string())
}

fn can_solve(equation: &Equation, ops: &[Op]) -> bool {
    let num_operators = equation.operands.len() - 1;

    (0..num_operators)
        .map(|_| ops)
        .multi_cartesian_product()
        .any(|combo| {
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
                return true;
            }

            false
        })
}

struct Equation {
    result: usize,
    operands: Vec<usize>,
}

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
}
