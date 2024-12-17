use std::u64;

use anyhow::anyhow;
use itertools::Itertools;

use crate::utils::not_yet_implemented;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    let mut lines = input.lines();

    let a: u64 = lines.next().unwrap()[12..].parse().unwrap();
    let b: u64 = lines.next().unwrap()[12..].parse().unwrap();
    let c: u64 = lines.next().unwrap()[12..].parse().unwrap();

    lines.next();

    let instructions = lines.next().unwrap()[9..]
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple::<(u8, u8)>().unwrap())
        .map(|chunk| (chunk.0.try_into().unwrap(), chunk.1))
        .collect_vec();

    let vm = Vm {
        a,
        b,
        c,
        instructions: &instructions,
        ..Default::default()
    };

    let part1_output = part1(vm.clone());
    let part2_output = part2(vm);

    Ok((part1_output, part2_output))
}

fn part1(mut vm: Vm) -> String {
    let mut output = Vec::new();

    while !vm.is_finished() {
        if let Some(data) = vm.perform_next() {
            output.push(data);
        }
    }

    output.iter().join(",")
}

fn part2(mut vm: Vm) -> String {
    let initial_b = vm.b;
    let initial_c = vm.c;
    let required_output_count = vm.instructions.len() * 2;

    'attempts: for a in 0..u64::MAX {
        vm.a = a;
        vm.b = initial_b;
        vm.c = initial_c;
        vm.ip = 0;

        let mut output_index = 0;

        if a % 1000000 == 0 {
            println!("Trying A = {}", a);
        }

        while output_index < required_output_count && !vm.is_finished() {
            if let Some(data) = vm.perform_next() {
                let (expected_op, expected_operand) = vm.instructions[output_index / 2];

                if output_index % 2 == 0 {
                    let Ok(op) = Op::try_from(data) else {
                        continue 'attempts;
                    };

                    if op != expected_op {
                        continue 'attempts;
                    }
                } else if data != expected_operand {
                    continue 'attempts;
                }

                output_index += 1;
            }
        }

        if output_index < required_output_count {
            // println!(
            //     "Expected {} output numbers, got {}.",
            //     expected_output_count, output_index
            // );
            continue;
        }

        return a.to_string();
    }

    panic!("There should be a solution!!!");
}

#[derive(Debug, Default, Clone)]
struct Vm<'a> {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    instructions: &'a [(Op, u8)],
}

impl Vm<'_> {
    /// Perform the next instruction in the instruction list. Optionally returns outputted data.
    fn perform_next(&mut self) -> Option<u8> {
        let (op, operand) = self.instructions[self.ip];

        self.ip += 1;
        self.perform(op, operand as u32)
    }

    fn is_finished(&self) -> bool {
        self.ip >= self.instructions.len()
    }

    fn perform(&mut self, op: Op, operand: u32) -> Option<u8> {
        match op {
            Op::Adv => {
                let numerator = self.a;
                let divisor = 2_u64.pow(self.combo(operand) as u32);

                self.a = numerator / divisor;
            }
            Op::Bxl => {
                self.b ^= operand as u64;
            }
            Op::Bst => {
                self.b = self.combo(operand) % 8;
            }
            Op::Jnz => {
                if self.a != 0 {
                    self.ip = (operand / 2) as usize;
                }
            }
            Op::Bxc => {
                self.b ^= self.c;
            }
            Op::Out => {
                return Some((self.combo(operand) % 8) as u8);
            }
            Op::Bdv => {
                let numerator = self.a;
                let divisor = 2_u64.pow(self.combo(operand) as u32);

                self.b = numerator / divisor;
            }
            Op::Cdv => {
                let numerator = self.a;
                let divisor = 2_u64.pow(self.combo(operand) as u32);

                self.c = numerator / divisor;
            }
        }

        None
    }

    fn combo(&self, operand: u32) -> u64 {
        match operand {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => operand as u64,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u8> for Op {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Op::Adv),
            1 => Ok(Op::Bxl),
            2 => Ok(Op::Bst),
            3 => Ok(Op::Jnz),
            4 => Ok(Op::Bxc),
            5 => Ok(Op::Out),
            6 => Ok(Op::Bdv),
            7 => Ok(Op::Cdv),
            _ => Err(anyhow::anyhow!("Invalid OpCode {}!", value)),
        }
    }
}

#[test]
fn test_instructions() {
    {
        let mut vm = Vm {
            c: 9,
            instructions: &[(2.try_into().unwrap(), 6)],
            ..Default::default()
        };

        let mut count = 0;

        while !vm.is_finished() {
            vm.perform_next();
            count += 1;
        }

        assert_eq!(count, 1);
        assert_eq!(vm.b, 1);
    }

    {
        let mut vm = Vm {
            a: 10,
            instructions: &[
                (5.try_into().unwrap(), 0),
                (5.try_into().unwrap(), 1),
                (5.try_into().unwrap(), 4),
            ],
            ..Default::default()
        };

        let mut count = 0;
        let mut output = Vec::new();

        while !vm.is_finished() {
            if let Some(data) = vm.perform_next() {
                output.push(data);
            }
            count += 1;
        }

        assert_eq!(count, 3);
        assert_eq!(output.iter().join(","), "0,1,2");
    }

    {
        let mut vm = Vm {
            a: 2024,
            instructions: &[
                (0.try_into().unwrap(), 1),
                (5.try_into().unwrap(), 4),
                (3.try_into().unwrap(), 0),
            ],
            ..Default::default()
        };

        let mut output = Vec::new();

        while !vm.is_finished() {
            if let Some(data) = vm.perform_next() {
                output.push(data);
            }
        }

        assert_eq!(vm.a, 0);
        assert_eq!(output.iter().join(","), "4,2,5,6,7,7,7,7,3,1,0");
    }

    {
        let mut vm = Vm {
            b: 29,
            instructions: &[(1.try_into().unwrap(), 7)],
            ..Default::default()
        };

        while !vm.is_finished() {
            vm.perform_next();
        }

        assert_eq!(vm.b, 26);
    }

    {
        let mut vm = Vm {
            b: 2024,
            c: 43690,
            instructions: &[(4.try_into().unwrap(), 0)],
            ..Default::default()
        };

        while !vm.is_finished() {
            vm.perform_next();
        }

        assert_eq!(vm.b, 44354);
    }
}
