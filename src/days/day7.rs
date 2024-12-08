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
            let split = line.split_once(": ").unwrap();

            let result: usize = split.0.parse().unwrap();
            let operands: Vec<usize> = split.1.split(" ").map(str::parse).try_collect().unwrap();

            Equation { result, operands }
        })
        .collect();

    Ok((part1(&equations)?, part2(&equations)?))
}

fn part1(equations: &[Equation]) -> PartResult {
    let sum: usize = equations
        .par_iter()
        .filter(|equation| is_possible(equation.result, &equation.operands, &[Op::Add, Op::Mul]))
        .map(|equation| equation.result)
        .sum();

    Ok(sum.to_string())
}

fn part2(equations: &[Equation]) -> PartResult {
    let sum: usize = equations
        .par_iter()
        .filter(|equation| {
            is_possible(
                equation.result,
                &equation.operands,
                &[Op::Add, Op::Mul, Op::Concat],
            )
        })
        .map(|equation| equation.result)
        .sum();

    Ok(sum.to_string())
}

fn is_possible(result: usize, operands: &[usize], ops: &[Op]) -> bool {
    // 1. Iterate backwards from the last value.
    // 2. For each step, we're looking to see if it is possible, using the operations we have, to
    //    step backwards from the current value (starting at `result`), towards 0.
    // 3. Discard impossibilities.
    // 4. We should have reached 0 by the end.

    match operands {
        [remaining @ .., last] => ops.iter().any(|op| {
            match op {
                // For addition, `x + last == result`, and there's no subtraction.
                // Thus, `result - last == x`, and `x >= 0` must be true.
                Op::Add => (*last <= result) && is_possible(result - last, remaining, ops),

                // For multiplication, `x * last == result`, thus `result / last == x`.
                // `x` must be a valid integer (`result % last == 0`)
                Op::Mul => (result % last == 0) && is_possible(result / last, remaining, ops),

                // Concatenation means that `result` ends in `last`, and `x` is the rest of
                // `result`'s digits.
                Op::Concat => {
                    // 1. Take the number of digits in `last`.
                    // 2. `(result - last) / 10^digits_last == x`.
                    // Therefore `(result - last) % 10^digits_last == 0`.

                    if *last > result {
                        return false;
                    }

                    let divisor = 10_usize.pow(last.ilog10() + 1);
                    let result_removed_last = result - last;

                    (result_removed_last % divisor == 0)
                        && is_possible(result_removed_last / divisor, remaining, ops)
                }
            }
        }),
        [] => result == 0,
    }
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
