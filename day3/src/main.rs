use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::result;
use std::str::FromStr;

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
struct CrossedWires {
    wires: Vec<Vec<Move>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    distance: i32,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl CrossedWires {
    fn find_intersections(&self) -> HashMap<Point, u32> {
        let mut intersections: HashMap<Point, u32> = HashMap::new();

        let mut occupied_points: HashMap<Point, u32> = HashMap::new();
        for (wire_index, wire) in self.wires.iter().enumerate() {
            let mut steps = 0;
            let mut end_point = Point { x: 0, y: 0 };
            for movement in wire.iter() {
                let mut point = end_point.clone();
                for _ in 0..movement.distance {
                    match movement.direction {
                        Direction::Up => point.y += 1,
                        Direction::Down => point.y -= 1,
                        Direction::Right => point.x += 1,
                        Direction::Left => point.x -= 1,
                    };
                    steps += 1;
                    if wire_index == 0 {
                        occupied_points.insert(point, steps);
                    } else {
                        if let Some(first_wire_steps) = occupied_points.get(&point) {
                            intersections.insert(point, first_wire_steps + steps);
                        }
                    }
                }
                end_point = point;
            }
        }

        intersections
    }
}

impl FromStr for CrossedWires {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<CrossedWires> {
        let mut wires = s.split("\n");
        let first_moves = wires.next().expect("First wire not found in input");
        let second_moves = wires.next().expect("Second wire not found in input");

        Ok(CrossedWires {
            wires: vec![
                get_moves_from_string(first_moves)?,
                get_moves_from_string(second_moves)?,
            ],
        })
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Could not parse direction: {}", c),
        }
    }
}

fn get_moves_from_string(moves_string: &str) -> Result<Vec<Move>> {
    let moves_strings = moves_string.split(",");
    let mut moves = vec![];

    for wire_move in moves_strings {
        let mut wire_move = wire_move.chars();
        let direction: Direction =
            Direction::from(wire_move.next().expect("Invalid empty wire move"));
        let distance: i32 = wire_move.collect::<String>().parse()?;

        moves.push(Move {
            direction,
            distance,
        });
    }
    Ok(moves)
}

fn read_wires(filename: &str) -> Result<CrossedWires> {
    let wires = fs::read_to_string(filename)?;
    Ok(wires.parse()?)
}

fn solve_part1() -> Result<i32> {
    let wires = read_wires(INPUT)?;
    let intersections = wires.find_intersections();
    let intersect_points = intersections.keys();
    let distances = intersect_points.map(|point| point.x.abs() + point.y.abs());
    Ok(distances.min().expect("No intersections found"))
}

fn solve_part2() -> Result<i32> {
    let wires = read_wires(INPUT)?;
    let intersections = wires.find_intersections();
    let min_intersection = intersections
        .iter()
        .min_by_key(|(_, steps)| steps.clone()).expect("No intersections found");
    Ok(*min_intersection.1 as i32)
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
    // const TEST_INPUT2: &str = "input/test2.txt";
    // const TEST_INPUT3: &str = "input/test3.txt";

    #[test]
    fn reads_wires() {
        assert_eq!(
            read_wires(TEST_INPUT1).unwrap(),
            CrossedWires {
                wires: vec![
                    vec![
                        Move {
                            direction: Direction::Right,
                            distance: 8
                        },
                        Move {
                            direction: Direction::Up,
                            distance: 5
                        },
                        Move {
                            direction: Direction::Left,
                            distance: 5
                        },
                        Move {
                            direction: Direction::Down,
                            distance: 3
                        },
                    ],
                    vec![
                        Move {
                            direction: Direction::Up,
                            distance: 7
                        },
                        Move {
                            direction: Direction::Right,
                            distance: 6
                        },
                        Move {
                            direction: Direction::Down,
                            distance: 4
                        },
                        Move {
                            direction: Direction::Left,
                            distance: 4
                        },
                    ],
                ],
            }
        );
    }
}
