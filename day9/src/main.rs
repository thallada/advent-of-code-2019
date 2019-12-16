use std::error::Error;
use std::result;

mod intcode;

use intcode::{Intcode, read_intcode};

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

fn solve_part1() -> Result<i64> {
    let mut intcode = read_intcode(INPUT)?;
    let output = intcode.execute(&[1]).expect("Failed to execute intcode");
    Ok(output[output.len() - 1])
}

fn solve_part2() -> Result<i64> {
    let mut intcode = read_intcode(INPUT)?;
    let output = intcode.execute(&[2]).expect("Failed to execute intcode");
    Ok(output[output.len() - 1])
}

fn main() -> Result<()> {
    println!("Part 1: {}", solve_part1()?);
    println!("Part 2: {}", solve_part2()?);

    Ok(())
}
