use itertools::Itertools;

use crate::utils::not_yet_implemented;

use super::{DayResult, PartResult};

pub(crate) fn solve(input: &str) -> DayResult {
    Ok((part1(input)?, part2(input)?))
}

fn part1(input: &str) -> PartResult {
    let mut lines = input.lines();

    let a: u32 = lines.next().unwrap()[12..].parse().unwrap();
    let b: u32 = lines.next().unwrap()[12..].parse().unwrap();
    let c: u32 = lines.next().unwrap()[12..].parse().unwrap();

    lines.next();

    let instructions = lines.next().unwrap()[9..]
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple::<(u8, u8)>().unwrap())
        .map(|chunk| (Op::from(chunk.0), chunk.1))
        .collect_vec();

    let mut vm = Vm {
        a,
        b,
        c,
        instructions,
        ..Default::default()
    };

    while vm.cycle() {
        println!("VM State: {:?}", vm);
    }

    println!("VM Output: {:?}", vm.output);

    not_yet_implemented()
}

#[derive(Debug, Default)]
struct Vm {
    a: u32,
    b: u32,
    c: u32,
    ip: usize,
    instructions: Vec<(Op, u8)>,
    output: Vec<u32>,
}

impl Vm {
    /// Perform the next instruction in the instruction list. Returns whether any instructions
    /// remain.
    fn cycle(&mut self) -> bool {
        let (op, operand) = self.instructions[self.ip];
        self.ip += 1;
        self.perform(op, operand as u32);

        self.ip < self.instructions.len()
    }

    fn perform(&mut self, op: Op, operand: u32) {
        match op {
            Op::Adv => {
                let numerator = self.a;
                let divisor = 2_u32.pow(self.combo(operand));

                self.a = numerator / divisor;
            }
            Op::Bxl => {
                self.b ^= operand;
            }
            Op::Bst => {
                self.b = operand % 8;
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
                self.output.push(self.combo(operand) % 8);
            }
            Op::Bdv => {
                let numerator = self.a;
                let divisor = 2_u32.pow(self.combo(operand));

                self.b = numerator / divisor;
            }
            Op::Cdv => {
                let numerator = self.a;
                let divisor = 2_u32.pow(self.combo(operand));

                self.c = numerator / divisor;
            }
        }
    }

    fn combo(&self, operand: u32) -> u32 {
        match operand {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => operand,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        match value {
            0 => Op::Adv,
            1 => Op::Bxl,
            2 => Op::Bst,
            3 => Op::Jnz,
            4 => Op::Bxc,
            5 => Op::Out,
            6 => Op::Bdv,
            7 => Op::Cdv,
            _ => panic!("Invalid OpCode {}!", value),
        }
    }
}

fn part2(input: &str) -> PartResult {
    not_yet_implemented()
}
