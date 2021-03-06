use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result;
use std::str::FromStr;

use num_enum::TryFromPrimitive;

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Intcode {
    pub integers: HashMap<usize, i64>,
    pub pointer: usize,
    pub halted: bool,
    pub relative_base: i64,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    parameter_modes: Vec<ParameterMode>,
}

impl TryFrom<i64> for Instruction {
    type Error = Box<dyn Error>;

    fn try_from(integer: i64) -> Result<Self> {
        let opcode: Opcode = Opcode::try_from((integer % 100) as u8)?;
        let modes_integer = integer / 100;
        let mut parameter_modes = vec![];
        for parameter_index in 0..opcode.parameter_count() {
            parameter_modes.push(ParameterMode::try_from(
                (modes_integer % (10_i64.pow(parameter_index + 1)) / 10_i64.pow(parameter_index))
                    as u8,
            )?)
        }
        Ok(Instruction {
            opcode,
            parameter_modes,
        })
    }
}

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    RelativeBaseOffset = 9,
    Halt = 99,
}

impl Opcode {
    pub fn parameter_count(&self) -> u32 {
        match self {
            Opcode::Add => 3,
            Opcode::Mult => 3,
            Opcode::Input => 1,
            Opcode::Output => 1,
            Opcode::JumpIfTrue => 2,
            Opcode::JumpIfFalse => 2,
            Opcode::LessThan => 3,
            Opcode::Equals => 3,
            Opcode::RelativeBaseOffset => 1,
            Opcode::Halt => 0,
        }
    }

    pub fn target_parameter_index(&self) -> Option<usize> {
        match self {
            Opcode::Add => Some(2),
            Opcode::Mult => Some(2),
            Opcode::Input => Some(0),
            Opcode::Output => None,
            Opcode::JumpIfTrue => None,
            Opcode::JumpIfFalse => None,
            Opcode::LessThan => Some(2),
            Opcode::Equals => Some(2),
            Opcode::RelativeBaseOffset => None,
            Opcode::Halt => None,
        }
    }
}

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl FromStr for Intcode {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Intcode> {
        let intcode_string = s.trim().to_string();
        let mut integers = HashMap::new();
        for (index, code) in intcode_string.split(',').enumerate() {
            integers.insert(index, code.parse().unwrap());
        }

        Ok(Intcode::new(integers))
    }
}

impl Intcode {
    fn new(integers: HashMap<usize, i64>) -> Intcode {
        Intcode {
            integers,
            pointer: 0,
            halted: false,
            relative_base: 0,
        }
    }

    fn load_parameters(&mut self, pointer: usize, instruction: &Instruction) -> Vec<i64> {
        (0..instruction.opcode.parameter_count() as usize)
            .map(|parameter_index| {
                let mut integer = *self
                    .integers
                    .entry(pointer + parameter_index + 1)
                    .or_insert(0);
                match instruction.parameter_modes[parameter_index] {
                    ParameterMode::Position => match instruction.opcode.target_parameter_index() {
                        Some(target_parameter_index)
                            if target_parameter_index == parameter_index => {}
                        _ => {
                            integer = *self.integers.entry(integer as usize).or_insert(0);
                        }
                    },
                    ParameterMode::Relative => match instruction.opcode.target_parameter_index() {
                        Some(target_parameter_index)
                            if target_parameter_index == parameter_index =>
                        {
                            integer += self.relative_base;
                        }
                        _ => {
                            integer = *self
                                .integers
                                .entry((self.relative_base + integer) as usize)
                                .or_insert(0);
                        }
                    },
                    _ => {}
                }
                integer
            })
            .collect()
    }

    pub fn execute(&mut self, inputs: &[i64]) -> Result<Vec<i64>> {
        let mut input_index = 0;
        let mut output = vec![];

        loop {
            let instruction =
                Instruction::try_from(*self.integers.entry(self.pointer).or_insert(0))?;
            let parameters = self.load_parameters(self.pointer, &instruction);
            let mut jump_pointer: Option<usize> = None;

            match instruction.opcode {
                Opcode::Add => {
                    self.integers
                        .insert(parameters[2] as usize, parameters[0] + parameters[1]);
                }
                Opcode::Mult => {
                    self.integers
                        .insert(parameters[2] as usize, parameters[0] * parameters[1]);
                }
                Opcode::Input => {
                    if input_index >= inputs.len() {
                        break; // pause execution to wait for more input
                    }
                    self.integers
                        .insert(parameters[0] as usize, inputs[input_index]);
                    input_index += 1;
                }
                Opcode::Output => {
                    output.push(parameters[0]);
                }
                Opcode::JumpIfTrue => {
                    if parameters[0] != 0 {
                        jump_pointer = Some(parameters[1] as usize);
                    }
                }
                Opcode::JumpIfFalse => {
                    if parameters[0] == 0 {
                        jump_pointer = Some(parameters[1] as usize);
                    }
                }
                Opcode::LessThan => {
                    if parameters[0] < parameters[1] {
                        self.integers.insert(parameters[2] as usize, 1);
                    } else {
                        self.integers.insert(parameters[2] as usize, 0);
                    }
                }
                Opcode::Equals => {
                    if parameters[0] == parameters[1] {
                        self.integers.insert(parameters[2] as usize, 1);
                    } else {
                        self.integers.insert(parameters[2] as usize, 0);
                    }
                }
                Opcode::RelativeBaseOffset => {
                    self.relative_base += parameters[0];
                }
                Opcode::Halt => {
                    self.halted = true;
                    break;
                }
            }

            match jump_pointer {
                Some(jump_pointer) => self.pointer = jump_pointer,
                None => self.pointer += 1 + instruction.opcode.parameter_count() as usize,
            }
        }

        Ok(output)
    }
}

pub fn read_intcode(filename: &str) -> Result<Intcode> {
    let mut file = File::open(filename)?;
    let mut intcode_string = String::new();
    file.read_to_string(&mut intcode_string)?;

    Ok(intcode_string.parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "input/test1.txt";

    #[test]
    fn reads_intcode() {
        assert_eq!(
            read_intcode(TEST_INPUT).unwrap(),
            Intcode::new(
                vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]
                    .into_iter()
                    .enumerate()
                    .collect()
            ),
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
        let mut intcode = Intcode::new(vec![1, 0, 0, 0, 99].into_iter().enumerate().collect());
        intcode.execute(&[0]).unwrap();
        assert_eq!(
            intcode.integers,
            vec![2, 0, 0, 0, 99].into_iter().enumerate().collect()
        );

        let mut intcode = Intcode::new(vec![2, 3, 0, 3, 99].into_iter().enumerate().collect());
        intcode.execute(&[0]).unwrap();
        assert_eq!(
            intcode.integers,
            vec![2, 3, 0, 6, 99].into_iter().enumerate().collect()
        );

        let mut intcode = Intcode::new(vec![2, 4, 4, 5, 99, 0].into_iter().enumerate().collect());
        intcode.execute(&[0]).unwrap();
        assert_eq!(
            intcode.integers,
            vec![2, 4, 4, 5, 99, 9801].into_iter().enumerate().collect()
        );

        let mut intcode = Intcode::new(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99]
                .into_iter()
                .enumerate()
                .collect(),
        );
        intcode.execute(&[0]).unwrap();
        assert_eq!(
            intcode.integers,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
                .into_iter()
                .enumerate()
                .collect()
        );

        let mut intcode = Intcode::new(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
                .into_iter()
                .enumerate()
                .collect(),
        );
        intcode.execute(&[0]).unwrap();
        assert_eq!(
            intcode.integers,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
                .into_iter()
                .enumerate()
                .collect()
        );
    }

    #[test]
    fn less_and_equal_outputs() {
        let intcode = Intcode::new(
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]
                .into_iter()
                .enumerate()
                .collect(),
        );
        assert_eq!(intcode.clone().execute(&[8]).unwrap(), vec![1]);
        assert_eq!(intcode.clone().execute(&[0]).unwrap(), vec![0]);

        let intcode = Intcode::new(
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]
                .into_iter()
                .enumerate()
                .collect(),
        );
        assert_eq!(intcode.clone().execute(&[0]).unwrap(), vec![1]);
        assert_eq!(intcode.clone().execute(&[9]).unwrap(), vec![0]);

        let intcode = Intcode::new(
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]
                .into_iter()
                .enumerate()
                .collect(),
        );
        assert_eq!(intcode.clone().execute(&[8]).unwrap(), vec![1]);
        assert_eq!(intcode.clone().execute(&[0]).unwrap(), vec![0]);

        let intcode = Intcode::new(
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]
                .into_iter()
                .enumerate()
                .collect(),
        );
        assert_eq!(intcode.clone().execute(&[0]).unwrap(), vec![1]);
        assert_eq!(intcode.clone().execute(&[9]).unwrap(), vec![0]);
    }

    #[test]
    fn jump_outputs() {
        let intcode = Intcode::new(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
                .into_iter()
                .enumerate()
                .collect(),
        );
        assert_eq!(intcode.clone().execute(&[0]).unwrap(), vec![0]);
        assert_eq!(intcode.clone().execute(&[1]).unwrap(), vec![1]);

        let intcode = Intcode::new(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
                .into_iter()
                .enumerate()
                .collect(),
        );
        assert_eq!(intcode.clone().execute(&[0]).unwrap(), vec![0]);
        assert_eq!(intcode.clone().execute(&[1]).unwrap(), vec![1]);
    }

    #[test]
    fn larger_part2_intcode() {
        let intcode = Intcode::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]
            .into_iter()
            .enumerate()
            .collect(),
        );
        assert_eq!(intcode.clone().execute(&[0]).unwrap(), vec![999]);
        assert_eq!(intcode.clone().execute(&[8]).unwrap(), vec![1000]);
        assert_eq!(intcode.clone().execute(&[9]).unwrap(), vec![1001]);
    }

    #[test]
    fn multiple_input_intcode() {
        let intcode = Intcode::new(
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ]
            .into_iter()
            .enumerate()
            .collect(),
        );
        assert_eq!(intcode.clone().execute(&[1, 1]).unwrap(), vec![11]);
    }

    #[test]
    fn relative_base_offset_quine() {
        let code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let intcode = Intcode::new(code.clone().into_iter().enumerate().collect());
        assert_eq!(intcode.clone().execute(&[]).unwrap(), code);
    }

    #[test]
    fn sixteen_digit_output() {
        let code = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let intcode = Intcode::new(code.into_iter().enumerate().collect());
        assert_eq!(intcode.clone().execute(&[]).unwrap(), [1219070632396864]);
    }

    #[test]
    fn large_output() {
        let code = vec![104, 1125899906842624, 99];
        let intcode = Intcode::new(code.into_iter().enumerate().collect());
        assert_eq!(intcode.clone().execute(&[]).unwrap(), [1125899906842624]);
    }

    #[test]
    fn relative_target_parameters() {
        let code = vec![109, 1, 203, 2, 204, 2, 99];
        let intcode = Intcode::new(code.into_iter().enumerate().collect());
        assert_eq!(intcode.clone().execute(&[123]).unwrap(), [123]);
    }
}
