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

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Game {
    intcode: Intcode,
    tiles: HashMap<Coordinate, Tile>,
    ball: Option<Coordinate>,
    paddle: Option<Coordinate>,
    score: i64,
}

impl Game {
    fn new(intcode: Intcode) -> Game {
        Game {
            intcode,
            tiles: HashMap::new(),
            ball: None,
            paddle: None,
            score: 0,
        }
    }

    fn update(&mut self, output: Vec<i64>) -> Result<()> {
        for index in (0..output.len()).step_by(3) {
            if output[index] == -1 {
                self.score = output[index + 2];
            } else {
                let x = output[index];
                let y = output[index + 1];
                let coord = Coordinate { x, y };
                let tile = Tile::try_from(output[index + 2] as u8)?;

                if tile == Tile::Ball {
                    self.ball = Some(coord);
                } else if tile == Tile::HorizontalPaddle {
                    self.paddle = Some(coord);
                }

                self.tiles.insert(Coordinate { x, y }, tile);
            }
        }
        Ok(())
    }

    fn step(&mut self, input: Option<i64>) -> Result<()> {
        let output = self
            .intcode
            .execute(&[input.unwrap_or(0)])
            .expect("Failed to execute intcode");
        self.update(output)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Score: {}", self.score)?;
        let start_coord = Coordinate { x: 0, y: 0 };
        let up_left_corner = Coordinate {
            x: self
                .tiles
                .keys()
                .min_by_key(|coord| coord.x)
                .unwrap_or(&start_coord)
                .x,
            y: self
                .tiles
                .keys()
                .min_by_key(|coord| coord.y)
                .unwrap_or(&start_coord)
                .y,
        };
        let down_right_corner = Coordinate {
            x: self
                .tiles
                .keys()
                .max_by_key(|coord| coord.x)
                .unwrap_or(&start_coord)
                .x,
            y: self
                .tiles
                .keys()
                .max_by_key(|coord| coord.y)
                .unwrap_or(&start_coord)
                .y,
        };
        for y in up_left_corner.y..=down_right_corner.y {
            let mut row_string = String::new();
            for x in up_left_corner.x..=down_right_corner.x {
                row_string += match self.tiles.get(&Coordinate { x, y }).unwrap_or(&Tile::Empty) {
                    Tile::Empty => " ",
                    Tile::Wall => "|",
                    Tile::Block => "#",
                    Tile::HorizontalPaddle => "=",
                    Tile::Ball => "o",
                };
            }
            write!(f, "{}\n", row_string)?;
        }
        Ok(())
    }
}

fn solve_part1() -> Result<i64> {
    let intcode = read_intcode(INPUT)?;
    let mut game = Game::new(intcode);
    game.step(None)?;
    Ok(game.tiles.values().fold(0, |acc, tile| {
        if *tile == Tile::Block {
            return acc + 1;
        }
        acc
    }))
}

fn solve_part2() -> Result<i64> {
    let intcode = read_intcode(INPUT)?;
    let mut game = Game::new(intcode);
    let mut input;
    while !game.intcode.halted {
        input = 0;
        if let Some(ball_coord) = game.ball {
            if let Some(paddle_coord) = game.paddle {
                if ball_coord.x > paddle_coord.x {
                    input = 1;
                } else if ball_coord.x < paddle_coord.x {
                    input = -1;
                }
            }
        }

        game.step(Some(input))?;
    }
    Ok(game.score)
}

fn main() -> Result<()> {
    println!("Part 1: {}", solve_part1()?);
    println!("Part 2: {}", solve_part2()?);

    Ok(())
}
