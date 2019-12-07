use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result;
use std::str::FromStr;

use num_enum::TryFromPrimitive;

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
struct Intcode {
    integers: Vec<i32>,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: Opcode,
    parameter_modes: Vec<ParameterMode>,
}

impl TryFrom<i32> for Instruction {
    type Error = Box<dyn Error>;

    fn try_from(integer: i32) -> Result<Self> {
        let opcode: Opcode = Opcode::try_from((integer % 100) as u8)?;
        let modes_integer = integer / 100;
        let mut parameter_modes = vec![];
        for parameter_index in 0..opcode.parameter_count() {
            parameter_modes.push(match opcode.target_parameter_index() {
                Some(target_parameter_index)
                    if target_parameter_index == parameter_index as usize =>
                {
                    ParameterMode::Position
                }
                _ => ParameterMode::try_from(
                    (modes_integer % (10_i32.pow(parameter_index + 1))
                        / 10_i32.pow(parameter_index)) as u8,
                )?,
            })
        }
        Ok(Instruction {
            opcode,
            parameter_modes,
        })
    }
}

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Opcode {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    Halt = 99,
}

impl Opcode {
    fn parameter_count(&self) -> u32 {
        match self {
            Opcode::Add => 3,
            Opcode::Mult => 3,
            Opcode::Input => 1,
            Opcode::Output => 1,
            Opcode::Halt => 0,
        }
    }

    fn target_parameter_index(&self) -> Option<usize> {
        match self {
            Opcode::Add => Some(2),
            Opcode::Mult => Some(2),
            Opcode::Input => Some(0),
            Opcode::Output => None,
            Opcode::Halt => None,
        }
    }
}

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl FromStr for Intcode {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Intcode> {
        let intcode_string = s.trim().to_string();

        Ok(Intcode {
            integers: intcode_string
                .split(',')
                .map(|code| code.parse().unwrap())
                .collect(),
        })
    }
}

impl Intcode {
    fn load_parameters(&self, pointer: usize, instruction: &Instruction) -> Vec<i32> {
        (0..instruction.opcode.parameter_count() as usize)
            .map(|parameter_index| {
                let mut integer = self.integers[pointer + parameter_index + 1];
                if let ParameterMode::Position = instruction.parameter_modes[parameter_index] {
                    match instruction.opcode.target_parameter_index() {
                        Some(target_parameter_index)
                            if target_parameter_index == parameter_index => {}
                        _ => {
                            integer = self.integers[integer as usize];
                        }
                    }
                }
                integer
            })
            .collect()
    }

    fn execute(&mut self, input: i32) -> Result<Vec<i32>> {
        let mut pointer = 0;
        let mut output = vec![];

        loop {
            let instruction = Instruction::try_from(self.integers[pointer])?;
            let parameters = self.load_parameters(pointer, &instruction);

            match instruction.opcode {
                Opcode::Add => {
                    self.integers[parameters[2] as usize] = parameters[0] + parameters[1];
                }
                Opcode::Mult => {
                    self.integers[parameters[2] as usize] = parameters[0] * parameters[1];
                }
                Opcode::Input => {
                    self.integers[parameters[0] as usize] = input;
                }
                Opcode::Output => {
                    output.push(parameters[0]);
                }
                Opcode::Halt => {
                    break;
                }
            }

            pointer += 1 + instruction.opcode.parameter_count() as usize;
        }

        Ok(output)
    }
}

fn read_intcode(filename: &str) -> Result<Intcode> {
    let mut file = File::open(filename)?;
    let mut intcode_string = String::new();
    file.read_to_string(&mut intcode_string)?;

    Ok(intcode_string.parse()?)
}

fn solve_part1() -> Result<i32> {
    let mut intcode = read_intcode(INPUT)?;
    Ok(intcode.execute(1)?.into_iter().last().ok_or("No output")?)
}

// fn solve_part2() -> io::Result<i32> {
// }

fn main() -> Result<()> {
    println!("Part 1: {:?}", solve_part1()?);
    // println!("Part 2: {}", solve_part2()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "input/test.txt";

    #[test]
    fn reads_intcode() {
        assert_eq!(
            read_intcode(TEST_INPUT).unwrap(),
            Intcode {
                integers: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
            },
        );
    }

    #[test]
    fn converts_integer_to_instruction() {
        assert_eq!(
            Instruction::try_from(1002).unwrap(),
            Instruction {
                opcode: Opcode::Mult,
                parameter_modes: vec![
                    ParameterMode::Position,
                    ParameterMode::Immediate,
                    ParameterMode::Position
                ],
            }
        );

        assert_eq!(
            Instruction::try_from(101).unwrap(),
            Instruction {
                opcode: Opcode::Add,
                parameter_modes: vec![
                    ParameterMode::Immediate,
                    ParameterMode::Position,
                    ParameterMode::Position
                ],
            }
        );
    }

    #[test]
    fn executes_intcodes() {
        let mut intcode = Intcode {
            integers: vec![1, 0, 0, 0, 99],
        };
        intcode.execute(0).unwrap();
        assert_eq!(intcode.integers, vec![2, 0, 0, 0, 99]);

        let mut intcode = Intcode {
            integers: vec![2, 3, 0, 3, 99],
        };
        intcode.execute(0).unwrap();
        assert_eq!(intcode.integers, vec![2, 3, 0, 6, 99]);

        let mut intcode = Intcode {
            integers: vec![2, 4, 4, 5, 99, 0],
        };
        intcode.execute(0).unwrap();
        assert_eq!(intcode.integers, vec![2, 4, 4, 5, 99, 9801]);

        let mut intcode = Intcode {
            integers: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        };
        intcode.execute(0).unwrap();
        assert_eq!(intcode.integers, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);

        let mut intcode = Intcode {
            integers: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        };
        intcode.execute(0).unwrap();
        assert_eq!(
            intcode.integers,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
}
