use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::result;

use num_enum::TryFromPrimitive;

mod intcode;

use intcode::{read_intcode, Intcode};

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
enum Color {
    Black = 0,
    White = 1,
}

#[derive(TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
enum Turn {
    Left = 0,
    Right = 1,
}

#[derive(TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match turn {
            Turn::Left => match self {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            },
            Turn::Right => match self {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

struct Robot {
    intcode: Intcode,
    position: Coordinate,
    direction: Direction,
}

impl Robot {
    fn new(intcode: Intcode) -> Robot {
        Robot {
            intcode: intcode.clone(),
            position: Coordinate { x: 0, y: 0 },
            direction: Direction::Up,
        }
    }

    fn turn_and_move(&mut self, turn: Turn) {
        self.direction = self.direction.turn(turn);
        match self.direction {
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
        }
    }
}

struct Hull {
    panels: HashMap<Coordinate, Color>,
}

impl Hull {
    fn new() -> Hull {
        Hull {
            panels: HashMap::new(),
        }
    }

    fn paint_registration(&mut self, intcode: Intcode, start_color: Color) -> Result<()> {
        let mut robot = Robot::new(intcode);
        let mut current_panel = start_color;
        while !robot.intcode.halted {
            let output = robot
                .intcode
                .execute(&[current_panel as i64])
                .expect("Failed to execute intcode");
            let color = Color::try_from(output[0] as u8)?;
            let turn = Turn::try_from(output[1] as u8)?;

            self.panels.insert(robot.position, color);
            robot.turn_and_move(turn);
            current_panel = *self.panels.get(&robot.position).unwrap_or(&Color::Black);
        }
        Ok(())
    }
}

impl fmt::Display for Hull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start_coord = Coordinate { x: 0, y: 0 };
        let up_left_corner = Coordinate {
            x: self
                .panels
                .keys()
                .min_by_key(|coord| coord.x)
                .unwrap_or(&start_coord)
                .x,
            y: self
                .panels
                .keys()
                .min_by_key(|coord| coord.y)
                .unwrap_or(&start_coord)
                .y,
        };
        let down_right_corner = Coordinate {
            x: self
                .panels
                .keys()
                .max_by_key(|coord| coord.x)
                .unwrap_or(&start_coord)
                .x,
            y: self
                .panels
                .keys()
                .max_by_key(|coord| coord.y)
                .unwrap_or(&start_coord)
                .y,
        };
        for y in up_left_corner.y..=down_right_corner.y {
            let mut row_string = String::new();
            for x in up_left_corner.x..=down_right_corner.x {
                row_string += match self
                    .panels
                    .get(&Coordinate { x, y })
                    .unwrap_or(&Color::Black)
                {
                    Color::Black => ".",
                    Color::White => "#",
                };
            }
            write!(f, "{}\n", row_string)?;
        }
        Ok(())
    }
}

fn solve_part1() -> Result<usize> {
    let intcode = read_intcode(INPUT)?;
    let mut hull = Hull::new();
    hull.paint_registration(intcode, Color::Black)?;
    Ok(hull.panels.len())
}

fn solve_part2() -> Result<String> {
    let intcode = read_intcode(INPUT)?;
    let mut hull = Hull::new();
    hull.paint_registration(intcode, Color::White)?;
    Ok(format!("\n{}", hull))
}

fn main() -> Result<()> {
    println!("Part 1: {}", solve_part1()?);
    println!("Part 2: {}", solve_part2()?);

    Ok(())
}
