use std::fs::File;
use std::io::{self, prelude::*};

const INPUT: &str = "input/input.txt";

fn read_intcode(filename: &str) -> io::Result<Vec<i32>> {
    let mut file = File::open(filename)?;
    let mut intcode_string = String::new();
    file.read_to_string(&mut intcode_string)?;
    let intcode_string = intcode_string.trim().to_string();

    Ok(intcode_string
        .split(',')
        .map(|code| code.parse().unwrap())
        .collect())
}

fn run_intcode(intcode: &mut Vec<i32>) {
    let mut pointer = 0;

    loop {
        match intcode[pointer] {
            1 => {
                let a = intcode[intcode[pointer + 1] as usize];
                let b = intcode[intcode[pointer + 2] as usize];
                let target = intcode[pointer + 3] as usize;
                intcode[target] = a + b;
            }
            2 => {
                let a = intcode[intcode[pointer + 1] as usize];
                let b = intcode[intcode[pointer + 2] as usize];
                let target = intcode[pointer + 3] as usize;
                intcode[target] = a * b;
            }
            99 => {
                break;
            }
            invalid => panic!("Invalid opcode: {}", invalid),
        }

        pointer += 4;
    }
}

fn solve_part1() -> io::Result<i32> {
    let mut intcode = read_intcode(INPUT)?;
    intcode[1] = 12;
    intcode[2] = 2;
    run_intcode(&mut intcode);
    Ok(intcode[0])
}

fn solve_part2() -> io::Result<i32> {
    let original_intcode = read_intcode(INPUT)?;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut intcode = original_intcode.clone();
            intcode[1] = noun;
            intcode[2] = verb;
            run_intcode(&mut intcode);
            if intcode[0] == 19690720 {
                return Ok(100 * noun + verb)
            }
        }
    }
    panic!("Could not find a noun and verb that produced the target value")
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", solve_part1()?);
    println!("Part 2: {}", solve_part2()?);

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
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn runs_intcodes() {
        let mut intcode = vec![1, 0, 0, 0, 99];
        run_intcode(&mut intcode);
        assert_eq!(intcode, vec![2, 0, 0, 0, 99]);

        let mut intcode = vec![2, 3, 0, 3, 99];
        run_intcode(&mut intcode);
        assert_eq!(intcode, vec![2, 3, 0, 6, 99]);

        let mut intcode = vec![2, 4, 4, 5, 99, 0];
        run_intcode(&mut intcode);
        assert_eq!(intcode, vec![2, 4, 4, 5, 99, 9801]);

        let mut intcode = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_intcode(&mut intcode);
        assert_eq!(intcode, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);

        let mut intcode = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run_intcode(&mut intcode);
        assert_eq!(intcode, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}
