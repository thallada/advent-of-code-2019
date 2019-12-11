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
}

impl Amplifier {
    fn new(intcode: Intcode) -> Amplifier {
        Amplifier {
            intcode: intcode,
        }
    }

    fn reset_intcode(&mut self, intcode: Intcode) {
        self.intcode = intcode;
    }

    fn execute(&mut self, input: i32) -> Result<Vec<i32>> {
        let output = self.intcode.execute(&[input])?;
        Ok(output)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct AmplificationCircuit {
    amplifiers: Vec<Amplifier>,
    intcode: Intcode,
}

impl AmplificationCircuit {
    fn new(
        intcode: Intcode,
        amplifier_count: usize,
    ) -> AmplificationCircuit {
        AmplificationCircuit {
            amplifiers: (0..amplifier_count)
                .map(|_| Amplifier::new(intcode.clone()))
                .collect(),
            intcode,
        }
    }

    fn set_phase_settings(&mut self, phase_settings: &[i32; 5]) -> Result<()> {
        for (index, phase_setting) in phase_settings.iter().enumerate() {
            self.amplifiers[index].execute(*phase_setting)?;
        }
        Ok(())
    }

    fn reset_circuit(&mut self) {
        for amplifier in self.amplifiers.iter_mut() {
            amplifier.reset_intcode(self.intcode.clone());
        }
    }

    fn execute_circuit(&mut self, input_signal: i32) -> Result<i32> {
        let mut input = input_signal;
        while !self.amplifiers[4].intcode.halted {
            for amplifier in self.amplifiers.iter_mut() {
                input = amplifier.execute(input)?[0];
            }
        }
        Ok(input)
    }

    fn find_max_output(
        &mut self,
        input_signal: i32,
        phase_setting_options: [i32; 5],
    ) -> Result<i32> {
        let mut phase_setting: [i32; 5] = phase_setting_options;
        let mut max_output = 0;
        let heap = Heap::new(&mut phase_setting);

        for permutation in heap {
            self.set_phase_settings(&permutation)?;

            let output = self.execute_circuit(input_signal)?;
            if output > max_output {
                max_output = output;
            }
            self.reset_circuit();
        }

        Ok(max_output)
    }
}

fn solve_part1() -> Result<i32> {
    let intcode = read_intcode(INPUT)?;
    let mut circuit = AmplificationCircuit::new(intcode, 5);
    Ok(circuit.find_max_output(0, [0, 1, 2, 3, 4])?)
}

fn solve_part2() -> Result<i32> {
    let intcode = read_intcode(INPUT)?;
    let mut circuit = AmplificationCircuit::new(intcode, 5);
    Ok(circuit.find_max_output(0, [5, 6, 7, 8, 9])?)
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
    const TEST_INPUT4: &str = "input/test4.txt";
    const TEST_INPUT5: &str = "input/test5.txt";

    #[test]
    fn executes_amplifier_circuits() {
        let intcode = read_intcode(TEST_INPUT1).unwrap();
        let mut circuit = AmplificationCircuit::new(intcode, 5);
        circuit.set_phase_settings(&[4, 3, 2, 1, 0]).unwrap();
        assert_eq!(circuit.execute_circuit(0).unwrap(), 43210);

        let intcode = read_intcode(TEST_INPUT2).unwrap();
        let mut circuit = AmplificationCircuit::new(intcode, 5);
        circuit.set_phase_settings(&[0, 1, 2, 3, 4]).unwrap();
        assert_eq!(circuit.execute_circuit(0).unwrap(), 54321);

        let intcode = read_intcode(TEST_INPUT3).unwrap();
        let mut circuit = AmplificationCircuit::new(intcode, 5);
        circuit.set_phase_settings(&[1, 0, 4, 3, 2]).unwrap();
        assert_eq!(circuit.execute_circuit(0).unwrap(), 65210);
    }

    #[test]
    fn finds_max_output_of_circuits() {
        let inputs = [TEST_INPUT1, TEST_INPUT2, TEST_INPUT3];
        let outputs = [43210, 54321, 65210];
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            let intcode = read_intcode(input).unwrap();
            let mut circuit = AmplificationCircuit::new(intcode, 5);
            assert_eq!(circuit.find_max_output(0, [0, 1, 2, 3, 4]).unwrap(), *output);
        }
    }

    #[test]
    fn executes_feedback_loop_amplifier_circuits() {
        let intcode = read_intcode(TEST_INPUT4).unwrap();
        let mut circuit = AmplificationCircuit::new(intcode, 5);
        circuit.set_phase_settings(&[9, 8, 7, 6, 5]).unwrap();
        assert_eq!(circuit.execute_circuit(0).unwrap(), 139629729);

        let intcode = read_intcode(TEST_INPUT5).unwrap();
        let mut circuit = AmplificationCircuit::new(intcode, 5);
        circuit.set_phase_settings(&[9, 7, 8, 5, 6]).unwrap();
        assert_eq!(circuit.execute_circuit(0).unwrap(), 18216);
    }

    #[test]
    fn finds_max_outputs_of_feedback_loop_circuits() {
        let inputs = [TEST_INPUT4, TEST_INPUT5];
        let outputs = [139629729, 18216];
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            let intcode = read_intcode(input).unwrap();
            let mut circuit = AmplificationCircuit::new(intcode, 5);
            assert_eq!(circuit.find_max_output(0, [5, 6, 7, 8, 9]).unwrap(), *output);
        }
    }
}
