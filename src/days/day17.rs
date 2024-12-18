use std::u64;

use super::DayResult;
use itertools::Itertools;

pub(crate) fn solve(input: &str) -> DayResult {
    let mut lines = input.lines();

    let a: u64 = lines.next().unwrap()[12..].parse()?;
    let b: u64 = lines.next().unwrap()[12..].parse()?;
    let c: u64 = lines.next().unwrap()[12..].parse()?;

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
    let part2_output = part2(vm)?;

    Ok((part1_output, part2_output))
}

fn part1(mut vm: Vm) -> String {
    let mut output = Vec::new();

    while let Some(maybe_data) = vm.perform_next() {
        if let Some(data) = maybe_data {
            output.push(data);
        }
    }

    output.iter().join(",")
}

fn part2(mut vm: Vm) -> anyhow::Result<String> {
    let initial_b = vm.b;
    let initial_c = vm.c;
    let required_num_instructions = vm.instructions.len();
    let required_output_length = required_num_instructions * 2;

    // I'm working under a bunch of assumptions here.
    // TODO: write 'em down!

    // Assumption: There is exactly one ADV instruction, used to effectively decrement the loop
    // count.
    let adv_operand = vm
        .instructions
        .iter()
        .find(|(op, _)| *op == Op::Adv)
        .map(|(_, operand)| *operand)
        .ok_or(anyhow::anyhow!("There should be one instance of ADV!"))?;

    // Assumption: ADV is only passed constant operands, so the number of loops is known before
    // execution.
    assert!(adv_operand <= 3);

    // Assumption: JNZ is only ever used to continue the program loop from the start, or cease
    // execution if no more numbers should be printed.
    assert!(*vm.instructions.last().unwrap() == (Op::Jnz, 0));

    let a_divisor = 2u64.pow(adv_operand as u32);
    let max_possible_a = a_divisor.pow(required_output_length as u32);

    // Given the assumptions we've made, we know within these bounds that a valid A value, when
    // divided by `2 ^ adv_operand`, `required_output_length` times, equals 0.
    match find_a(&mut vm, 1, initial_b, initial_c, a_divisor) {
        Some(a) => Ok(a.to_string()),
        None => Err(anyhow::anyhow!("Failed to find the value of A!")),
    }
}

fn find_a(
    vm: &mut Vm,
    initial_search_a: u64,
    initial_b: u64,
    initial_c: u64,
    a_divisor: u64,
) -> Option<u64> {
    let required_num_instructions = vm.instructions.len();

    'mid_search: for n in 0..=a_divisor {
        let test_a = initial_search_a + n;

        vm.a = test_a;
        vm.b = initial_b;
        vm.c = initial_c;
        vm.ip = 0;

        println!("From initial {initial_search_a} :: {test_a} :: #{n}");

        for output_index in (0..required_num_instructions).rev() {
            let Some(op_result) = vm.perform_until_output().map(Op::try_from) else {
                // if output_index == (required_num_instructions - 1) {
                //     continue 'mid_search;
                // }

                // Partial match.
                if let Some(a) = find_a(vm, test_a * a_divisor, initial_b, initial_c, a_divisor) {
                    return Some(a);
                } else {
                    continue 'mid_search;
                }
            };

            let Ok(op) = op_result else {
                continue 'mid_search;
            };

            let Some(operand) = vm.perform_until_output() else {
                // if output_index == (required_num_instructions - 1) {
                //     continue 'mid_search;
                // }

                // Partial match.
                if let Some(a) = find_a(vm, test_a * a_divisor, initial_b, initial_c, a_divisor) {
                    return Some(a);
                } else {
                    continue 'mid_search;
                }
            };

            if vm.instructions[output_index] != (op, operand) {
                continue 'mid_search;
            }
        }

        return Some(test_a);
    }

    None
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
    /// Perform the next instruction in the instruction list. If the program has finished, `None`
    /// is returned.
    fn perform_next(&mut self) -> Option<Option<u8>> {
        if self.is_finished() {
            return None;
        }

        let (op, operand) = self.instructions[self.ip];

        self.ip += 1;
        Some(self.perform(op, operand as u32))
    }

    /// Execute the program until an output is made, or return `None` if the program ends without
    /// outputting any value.
    fn perform_until_output(&mut self) -> Option<u8> {
        while let Some(maybe_data) = self.perform_next() {
            if maybe_data.is_some() {
                return maybe_data;
            }
        }

        None
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

        while let Some(maybe_data) = vm.perform_next() {
            if let Some(data) = maybe_data {
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

        while let Some(maybe_data) = vm.perform_next() {
            if let Some(data) = maybe_data {
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
