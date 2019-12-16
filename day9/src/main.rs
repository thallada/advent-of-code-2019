use std::error::Error;
use std::result;

mod intcode;

use intcode::{Intcode, read_intcode};

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

fn solve_part1() -> Result<u32> {
    Ok(1)
}

fn solve_part2() -> Result<u32> {
    Ok(1)
}

fn main() -> Result<()> {
    println!("Part 1: {}", solve_part1()?);
    println!("Part 2: {}", solve_part2()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
