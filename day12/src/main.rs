#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::AddAssign;
use std::ops::Index;
use std::result;
use std::str::FromStr;

use num::integer::lcm;
use regex::Regex;

type Result<T> = result::Result<T, Box<dyn Error>>;

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Body {
    position: Vector,
    velocity: Vector,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct NBody {
    bodies: Vec<Body>,
}

impl FromStr for Body {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Body> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
        }

        let captures = match RE.captures(s) {
            None => {
                return Err(From::from("Malformed scan, no positions could be found"));
            }
            Some(captures) => captures,
        };

        Ok(Body {
            position: Vector {
                x: captures["x"].parse()?,
                y: captures["y"].parse()?,
                z: captures["z"].parse()?,
            },
            velocity: Vector::new(),
        })
    }
}

impl Vector {
    fn new() -> Vector {
        Vector { x: 0, y: 0, z: 0 }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Index<&str> for Vector {
    type Output = i64;

    fn index(&self, index: &str) -> &i64 {
        match index {
            "x" => &self.x,
            "y" => &self.y,
            "z" => &self.z,
            _ => panic!("unknown field: {}", index),
        }
    }
}

impl Body {
    fn add_gravity(&self, gravity: &mut Vector, other: &Self) {
        if self.position.x > other.position.x {
            gravity.x -= 1;
        } else if self.position.x < other.position.x {
            gravity.x += 1;
        }

        if self.position.y > other.position.y {
            gravity.y -= 1;
        } else if self.position.y < other.position.y {
            gravity.y += 1;
        }

        if self.position.z > other.position.z {
            gravity.z -= 1;
        } else if self.position.z < other.position.z {
            gravity.z += 1;
        }
    }
}

impl NBody {
    fn run_step(&mut self) {
        let mut gravities = Vec::new();
        for body in self.bodies.iter() {
            let mut gravity = Vector::new();
            for other_body in self.bodies.iter() {
                body.add_gravity(&mut gravity, other_body);
            }
            gravities.push(gravity);
        }

        for (index, gravity) in gravities.into_iter().enumerate() {
            self.bodies[index].velocity += gravity;
            let velocity = self.bodies[index].velocity;
            self.bodies[index].position += velocity;
        }
    }

    fn total_energy(&self) -> i64 {
        let mut total_energy = 0;
        for body in self.bodies.iter() {
            let potential_energy =
                body.position.x.abs() + body.position.y.abs() + body.position.z.abs();
            let kinetic_energy =
                body.velocity.x.abs() + body.velocity.y.abs() + body.velocity.z.abs();
            total_energy += potential_energy * kinetic_energy;
        }
        total_energy
    }

    fn state(&self, component: &str) -> [(i64, i64); 4] {
        [
            (
                self.bodies[0].position[component],
                self.bodies[0].velocity[component],
            ),
            (
                self.bodies[1].position[component],
                self.bodies[1].velocity[component],
            ),
            (
                self.bodies[2].position[component],
                self.bodies[2].velocity[component],
            ),
            (
                self.bodies[3].position[component],
                self.bodies[3].velocity[component],
            ),
        ]
    }
}

fn read_moon_scan(filename: &str) -> Result<NBody> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut moons = vec![];

    for line in reader.lines() {
        moons.push(line?.parse()?);
    }

    Ok(NBody { bodies: moons })
}

fn solve_part1(filename: &str) -> Result<i64> {
    let mut nbody = read_moon_scan(filename)?;
    for _ in 0..1000 {
        nbody.run_step();
    }
    Ok(nbody.total_energy())
}

fn solve_part2(filename: &str) -> Result<u64> {
    let mut step_count = 0;
    let mut x_states: HashSet<[(i64, i64); 4]> = HashSet::new();
    let mut y_states: HashSet<[(i64, i64); 4]> = HashSet::new();
    let mut z_states: HashSet<[(i64, i64); 4]> = HashSet::new();
    let mut x_repeated_step_count = None;
    let mut y_repeated_step_count = None;
    let mut z_repeated_step_count = None;
    let mut nbody = read_moon_scan(filename)?;
    while x_repeated_step_count == None
        || y_repeated_step_count == None
        || z_repeated_step_count == None
    {
        if x_repeated_step_count == None {
            let x_state = nbody.state("x");
            if x_states.contains(&x_state) {
                x_repeated_step_count = Some(step_count);
            } else {
                x_states.insert(x_state);
            }
        }

        if y_repeated_step_count == None {
            let y_state = nbody.state("y");
            if y_states.contains(&y_state) {
                y_repeated_step_count = Some(step_count);
            } else {
                y_states.insert(y_state);
            }
        }

        if z_repeated_step_count == None {
            let z_state = nbody.state("z");
            if z_states.contains(&z_state) {
                z_repeated_step_count = Some(step_count);
            } else {
                z_states.insert(z_state);
            }
        }

        nbody.run_step();
        step_count += 1;
    }

    Ok(lcm(
        x_repeated_step_count.unwrap(),
        lcm(
            y_repeated_step_count.unwrap(),
            z_repeated_step_count.unwrap(),
        ),
    ))
}

fn main() -> Result<()> {
    println!("Part 1: {}", solve_part1(INPUT)?);
    println!("Part 2: {}", solve_part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "input/test1.txt";
    const TEST_INPUT2: &str = "input/test2.txt";
    fn nbody_1() -> NBody {
        NBody {
            bodies: vec![
                Body {
                    position: Vector { x: -1, y: 0, z: 2 },
                    velocity: Vector { x: 0, y: 0, z: 0 },
                },
                Body {
                    position: Vector {
                        x: 2,
                        y: -10,
                        z: -7,
                    },
                    velocity: Vector { x: 0, y: 0, z: 0 },
                },
                Body {
                    position: Vector { x: 4, y: -8, z: 8 },
                    velocity: Vector { x: 0, y: 0, z: 0 },
                },
                Body {
                    position: Vector { x: 3, y: 5, z: -1 },
                    velocity: Vector { x: 0, y: 0, z: 0 },
                },
            ],
        }
    }
    fn nbody_1_after_10_steps() -> NBody {
        NBody {
            bodies: vec![
                Body {
                    position: Vector { x: 2, y: 1, z: -3 },
                    velocity: Vector { x: -3, y: -2, z: 1 },
                },
                Body {
                    position: Vector { x: 1, y: -8, z: 0 },
                    velocity: Vector { x: -1, y: 1, z: 3 },
                },
                Body {
                    position: Vector { x: 3, y: -6, z: 1 },
                    velocity: Vector { x: 3, y: 2, z: -3 },
                },
                Body {
                    position: Vector { x: 2, y: 0, z: 4 },
                    velocity: Vector { x: 1, y: -1, z: -1 },
                },
            ],
        }
    }
    fn nbody_2() -> NBody {
        NBody {
            bodies: vec![
                Body {
                    position: Vector {
                        x: -8,
                        y: -10,
                        z: 0,
                    },
                    velocity: Vector { x: 0, y: 0, z: 0 },
                },
                Body {
                    position: Vector { x: 5, y: 5, z: 10 },
                    velocity: Vector { x: 0, y: 0, z: 0 },
                },
                Body {
                    position: Vector { x: 2, y: -7, z: 3 },
                    velocity: Vector { x: 0, y: 0, z: 0 },
                },
                Body {
                    position: Vector { x: 9, y: -8, z: -3 },
                    velocity: Vector { x: 0, y: 0, z: 0 },
                },
            ],
        }
    }
    fn nbody_2_after_100_steps() -> NBody {
        NBody {
            bodies: vec![
                Body {
                    position: Vector {
                        x: 8,
                        y: -12,
                        z: -9,
                    },
                    velocity: Vector { x: -7, y: 3, z: 0 },
                },
                Body {
                    position: Vector {
                        x: 13,
                        y: 16,
                        z: -3,
                    },
                    velocity: Vector {
                        x: 3,
                        y: -11,
                        z: -5,
                    },
                },
                Body {
                    position: Vector {
                        x: -29,
                        y: -11,
                        z: -1,
                    },
                    velocity: Vector { x: -3, y: 7, z: 4 },
                },
                Body {
                    position: Vector {
                        x: 16,
                        y: -13,
                        z: 23,
                    },
                    velocity: Vector { x: 7, y: 1, z: 1 },
                },
            ],
        }
    }

    #[test]
    fn reads_moon_scan_file() {
        assert_eq!(read_moon_scan(TEST_INPUT1).unwrap(), nbody_1());
        assert_eq!(read_moon_scan(TEST_INPUT2).unwrap(), nbody_2());
    }

    #[test]
    fn runs_10_steps() {
        let mut nbody = read_moon_scan(TEST_INPUT1).unwrap();
        for _ in 0..10 {
            nbody.run_step();
        }
        assert_eq!(nbody, nbody_1_after_10_steps());
    }

    #[test]
    fn runs_100_steps() {
        let mut nbody = read_moon_scan(TEST_INPUT2).unwrap();
        for _ in 0..100 {
            nbody.run_step();
        }
        assert_eq!(nbody, nbody_2_after_100_steps());
    }

    #[test]
    fn calculates_total_energy_after_10_steps() {
        let mut nbody = read_moon_scan(TEST_INPUT1).unwrap();
        for _ in 0..10 {
            nbody.run_step();
        }
        assert_eq!(nbody.total_energy(), 179);
    }

    #[test]
    fn calculates_total_energy_after_100_steps() {
        let mut nbody = read_moon_scan(TEST_INPUT2).unwrap();
        for _ in 0..100 {
            nbody.run_step();
        }
        assert_eq!(nbody.total_energy(), 1940);
    }

    #[test]
    fn finds_repeated_states() {
        assert_eq!(solve_part2(TEST_INPUT1).unwrap(), 2772);
        assert_eq!(solve_part2(TEST_INPUT2).unwrap(), 4686774924);
    }
}
