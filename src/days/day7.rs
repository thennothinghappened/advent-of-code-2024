use std::time::Instant;

use itertools::Itertools;
use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
    },
    str::ParallelString,
};

use super::{DayResult, PartResult};

/// A reasonable, though picked from mid-air, value for which we bother to cache op permutations up
/// to. If we go over this, the world explodes. Raise as needed.
const REASONABLE_MAX_CACHE_LEN: usize = 12;

pub(crate) fn solve(input: &str) -> DayResult {
    let equations: Vec<Equation> = input
        .par_lines()
        .map(|line| {
            let split = line.split_once(": ").unwrap();

            let result: usize = split.0.parse().unwrap();
            let operands: Vec<usize> = split.1.split(" ").map(str::parse).try_collect().unwrap();

            Equation { result, operands }
        })
        .collect();

    Ok((part1(&equations)?, part2(&equations)?))
}

fn part1(equations: &[Equation]) -> PartResult {
    const ALLOWED_OPS: &[Op] = &[Op::Add, Op::Mul];
    let allowed_ops_cache = make_ops_cache(ALLOWED_OPS);

    let sum: usize = equations
        .par_iter()
        .filter(|equation| can_solve(&equation, &allowed_ops_cache))
        .map(|equation| equation.result)
        .sum();

    Ok(sum.to_string())
}

fn part2(equations: &[Equation]) -> PartResult {
    const ALLOWED_OPS: &[Op] = &[Op::Add, Op::Mul, Op::Concat];
    let now = Instant::now();
    let allowed_ops_cache = make_ops_cache(ALLOWED_OPS);

    println!("Took {}µs to build cache", now.elapsed().as_micros());

    let sum: usize = equations
        .par_iter()
        .filter(|equation| can_solve(&equation, &allowed_ops_cache))
        .map(|equation| equation.result)
        .sum();

    Ok(sum.to_string())
}

fn can_solve(equation: &Equation, ops_cache: &[Vec<Vec<&Op>>]) -> bool {
    let num_operators = equation.operands.len() - 1;

    ops_cache[num_operators].iter().any(|combo| {
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

fn make_ops_cache(ops: &[Op]) -> Vec<Vec<Vec<&Op>>> {
    (0..REASONABLE_MAX_CACHE_LEN)
        .into_par_iter()
        .map(|num_ops| {
            (0..num_ops)
                .map(|_| ops)
                .multi_cartesian_product()
                .collect_vec()
        })
        .collect()
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
