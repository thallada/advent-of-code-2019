use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const INPUT: &str = "input/input.txt";

fn read_masses(filename: &str) -> io::Result<Vec<u32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|mass| mass.unwrap().parse().unwrap())
        .collect())
}

fn calculate_fuel_requirement(mass: u32) -> u32 {
    mass / 3 - 2
}

fn calculate_fuel_requirement_including_fuel_mass(mass: u32) -> u32 {
    let mut fuel: i32 = mass as i32 / 3 - 2;
    let mut total_requirement = 0;

    while fuel > 0 {
        total_requirement += fuel as u32;
        fuel = fuel / 3 - 2;
    }

    total_requirement
}

fn calculate_fuel_sum(masses: Vec<u32>) -> u32 {
    let fuel_requirements: Vec<u32> = masses
        .iter()
        .map(|mass| calculate_fuel_requirement(*mass))
        .collect();
    fuel_requirements.iter().sum()
}

fn calculate_fuel_sum_including_fuel_mass(masses: Vec<u32>) -> u32 {
    let fuel_requirements: Vec<u32> = masses
        .iter()
        .map(|mass| calculate_fuel_requirement_including_fuel_mass(*mass))
        .collect();
    fuel_requirements.iter().sum()
}

fn solve_part1() -> io::Result<u32> {
    Ok(calculate_fuel_sum(read_masses(INPUT)?))
}

fn solve_part2() -> io::Result<u32> {
    Ok(calculate_fuel_sum_including_fuel_mass(read_masses(INPUT)?))
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
    fn reads_masses() {
        assert_eq!(read_masses(TEST_INPUT).unwrap(), vec![12, 14, 1969, 100756]);
    }

    #[test]
    fn calculates_correct_fuel_requirements() {
        assert_eq!(calculate_fuel_requirement(12), 2);
        assert_eq!(calculate_fuel_requirement(14), 2);
        assert_eq!(calculate_fuel_requirement(1969), 654);
        assert_eq!(calculate_fuel_requirement(100756), 33583);
    }

    #[test]
    fn calculates_correct_fuel_requirements_including_fuel_mass() {
        assert_eq!(calculate_fuel_requirement_including_fuel_mass(14), 2);
        assert_eq!(calculate_fuel_requirement_including_fuel_mass(1969), 966);
        assert_eq!(calculate_fuel_requirement_including_fuel_mass(100756), 50346);
    }
}
