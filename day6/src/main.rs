use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result;

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

fn read_orbit_map(filename: &str) -> Result<HashMap<String, Option<String>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut objects: HashMap<String, Option<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(")");
        let mass_name = parts
            .next()
            .expect("Invalid line, no mass part.")
            .to_string();
        let orbiter_name = parts
            .next()
            .expect("Invalid line, no orbiter part.")
            .to_string();

        objects.entry(mass_name.clone()).or_insert(None);
        objects.insert(orbiter_name.clone(), Some(mass_name));
    }

    Ok(objects)
}

fn get_orbit_count(orbit_map: &HashMap<String, Option<String>>, orbiter: String) -> u32 {
    match orbit_map.get(&orbiter) {
        None => panic!("Incomplete orbit map"),
        Some(mass) => match mass {
            None => return 0,
            Some(mass) => return 1 + get_orbit_count(orbit_map, mass.to_string()),
        },
    }
}

fn get_orbit_count_checksum(orbit_map: &HashMap<String, Option<String>>) -> u32 {
    let mut checksum = 0;

    for orbiter in orbit_map.keys() {
        checksum += get_orbit_count(&orbit_map, orbiter.to_string());
    }

    checksum
}

fn solve_part1() -> Result<u32> {
    let orbit_map = read_orbit_map(INPUT)?;
    Ok(get_orbit_count_checksum(&orbit_map))
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

    const TEST_INPUT: &str = "input/test.txt";

    #[test]
    fn reads_orbit_map() {
        assert_eq!(
            read_orbit_map(TEST_INPUT).unwrap(),
            [
                ("COM".to_string(), None),
                ("B".to_string(), Some("COM".to_string())),
                ("C".to_string(), Some("B".to_string())),
                ("D".to_string(), Some("C".to_string())),
                ("E".to_string(), Some("D".to_string())),
                ("F".to_string(), Some("E".to_string())),
                ("G".to_string(), Some("B".to_string())),
                ("H".to_string(), Some("G".to_string())),
                ("I".to_string(), Some("D".to_string())),
                ("J".to_string(), Some("E".to_string())),
                ("K".to_string(), Some("J".to_string())),
                ("L".to_string(), Some("K".to_string())),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn gets_orbit_count_checksum() {
        let orbit_map = read_orbit_map(TEST_INPUT).unwrap();
        assert_eq!(get_orbit_count_checksum(&orbit_map), 42)
    }
}
