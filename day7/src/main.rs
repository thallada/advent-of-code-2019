use std::error::Error;
use std::result;

use permutohedron::Heap;

mod intcode;

use intcode::{read_intcode, Intcode};

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, PartialEq)]
struct Amplifier {
    intcode: Intcode,
    phase_setting: i32,
}

impl Amplifier {
    fn new(intcode: Intcode, phase_setting: i32) -> Amplifier {
        Amplifier {
            intcode: intcode,
            phase_setting: phase_setting,
        }
    }

    fn execute(&self, input: i32) -> Result<i32> {
        let mut intcode = self.intcode.clone();
        let output = intcode.execute(&[self.phase_setting, input])?;
        dbg!(&output);
        Ok(output[0])
    }
}

#[derive(Debug, Clone, PartialEq)]
struct AmplificationCircuit {
    amplifiers: Vec<Amplifier>,
}

impl AmplificationCircuit {
    fn new(
        intcode: Intcode,
        amplifier_count: usize,
        initial_phase_settings: &[i32],
    ) -> AmplificationCircuit {
        AmplificationCircuit {
            amplifiers: (0..amplifier_count)
                .map(|index| Amplifier::new(intcode.clone(), initial_phase_settings[index]))
                .collect(),
        }
    }

    fn set_phase_settings(&mut self, phase_settings: &[i32; 5]) {
        for (index, phase_setting) in phase_settings.iter().enumerate() {
            self.amplifiers[index].phase_setting = *phase_setting;
        }
    }

    fn execute_circuit(&self, input_signal: i32) -> Result<i32> {
        let mut input = input_signal;
        for amplifier in self.amplifiers.iter() {
            input = amplifier.execute(input)?;
        }
        Ok(input)
    }

    fn find_max_output(&mut self, input_signal: i32) -> Result<i32> {
        let mut phase_setting: [i32; 5] = [0, 1, 2, 3, 4];
        let mut max_output = 0;
        let heap = Heap::new(&mut phase_setting);

        for permutation in heap {
            self.set_phase_settings(&permutation);

            let output = self.execute_circuit(input_signal)?;
            if output > max_output {
                max_output = output;
            }
        }

        Ok(max_output)
    }
}

fn solve_part1() -> Result<i32> {
    let intcode = read_intcode(INPUT)?;
    let mut circuit = AmplificationCircuit::new(intcode, 5, &[0, 0, 0, 0, 0]);
    Ok(circuit.find_max_output(0)?)
}

fn solve_part2() -> Result<i32> {
    Ok(0)
}

fn main() -> Result<()> {
    println!("Part 1: {}", solve_part1()?);
    println!("Part 2: {}", solve_part2()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "input/test1.txt";
    const TEST_INPUT2: &str = "input/test2.txt";
    const TEST_INPUT3: &str = "input/test3.txt";

    #[test]
    fn executes_amplifier_circuits() {
        let intcode = read_intcode(TEST_INPUT1).unwrap();
        let circuit = AmplificationCircuit::new(intcode, 5, &[4, 3, 2, 1, 0]);
        assert_eq!(circuit.execute_circuit(0).unwrap(), 43210);

        let intcode = read_intcode(TEST_INPUT2).unwrap();
        let circuit = AmplificationCircuit::new(intcode, 5, &[0, 1, 2, 3, 4]);
        assert_eq!(circuit.execute_circuit(0).unwrap(), 54321);

        let intcode = read_intcode(TEST_INPUT3).unwrap();
        let circuit = AmplificationCircuit::new(intcode, 5, &[1, 0, 4, 3, 2]);
        assert_eq!(circuit.execute_circuit(0).unwrap(), 65210);
    }

    #[test]
    fn finds_max_output_of_circuits() {
        let inputs = [TEST_INPUT1, TEST_INPUT2, TEST_INPUT3];
        let outputs = [43210, 54321, 65210];
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            let intcode = read_intcode(input).unwrap();
            let mut circuit = AmplificationCircuit::new(intcode, 5, &[0, 0, 0, 0, 0]);
            assert_eq!(circuit.find_max_output(0).unwrap(), *output);
        }
    }
}
