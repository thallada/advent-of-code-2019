use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result;

use num::integer::gcd;

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct AsteroidField {
    asteroids: Vec<Point>,
}

impl AsteroidField {
    fn find_monitoring_station(&self) -> (&Point, usize) {
        let mut asteroid_detect_scores = HashMap::new();
        for asteroid in self.asteroids.iter() {
            let mut lines_of_sight: HashMap<(i32, i32), Point> = HashMap::new();
            for other in self.asteroids.iter() {
                if asteroid != other {
                    let x_dist: i32 = other.x as i32 - asteroid.x as i32;
                    let y_dist: i32 = other.y as i32 - asteroid.y as i32;
                    let mut x_ratio: i32 = 0;
                    let mut y_ratio: i32 = 0;
                    if x_dist == 0 {
                        if y_dist > 0 {
                            y_ratio = 1;
                        } else {
                            y_ratio = -1;
                        }
                    } else if y_dist == 0 {
                        if x_dist > 0 {
                            x_ratio = 1;
                        } else {
                            x_ratio = -1;
                        }
                    } else {
                        let gcd = gcd(x_dist, y_dist);
                        x_ratio = x_dist / gcd;
                        y_ratio = y_dist / gcd;
                    }
                    lines_of_sight
                        .entry((x_ratio, y_ratio))
                        .and_modify(|current| {
                            if (current.x as i32 - asteroid.x as i32).abs() > x_dist.abs()
                                && (current.y as i32 - asteroid.y as i32).abs() > y_dist.abs()
                            {
                                current.x = other.x;
                                current.y = other.y;
                            }
                        })
                        .or_insert(other.clone());
                }
            }

            asteroid_detect_scores.insert(asteroid, lines_of_sight.len());
        }

        asteroid_detect_scores
            .into_iter()
            .max_by_key(|score| score.1)
            .expect("No asteroid detect scores")
    }
}

fn read_asteroid_field(filename: &str) -> Result<AsteroidField> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut asteroids = vec![];

    for (y, line) in reader.lines().enumerate() {
        for (x, contents) in line?.chars().enumerate() {
            if contents == '#' {
                asteroids.push(Point { x, y });
            }
        }
    }

    Ok(AsteroidField { asteroids })
}

fn solve_part1() -> Result<usize> {
    let asteroid_field = read_asteroid_field(INPUT)?;
    Ok(asteroid_field.find_monitoring_station().1)
}

fn solve_part2() -> Result<i64> {
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

    const TEST_INPUT1: &str = "input/test1.txt";
    const TEST_INPUT2: &str = "input/test2.txt";
    const TEST_INPUT3: &str = "input/test3.txt";
    const TEST_INPUT4: &str = "input/test4.txt";
    const TEST_INPUT5: &str = "input/test5.txt";

    #[test]
    fn reads_asteroid_field() {
        assert_eq!(
            read_asteroid_field(TEST_INPUT1).unwrap(),
            AsteroidField {
                asteroids: vec![
                    Point { x: 1, y: 0 },
                    Point { x: 4, y: 0 },
                    Point { x: 0, y: 2 },
                    Point { x: 1, y: 2 },
                    Point { x: 2, y: 2 },
                    Point { x: 3, y: 2 },
                    Point { x: 4, y: 2 },
                    Point { x: 4, y: 3 },
                    Point { x: 3, y: 4 },
                    Point { x: 4, y: 4 },
                ]
            },
        )
    }

    #[test]
    fn finds_monitoring_stations() {
        for (input, monitoring_point) in [
            (TEST_INPUT1, Point { x: 3, y: 4 }),
            (TEST_INPUT2, Point { x: 5, y: 8 }),
            (TEST_INPUT3, Point { x: 1, y: 2 }),
            (TEST_INPUT4, Point { x: 6, y: 3 }),
            (TEST_INPUT5, Point { x: 11, y: 13 }),
        ]
        .iter()
        {
            let asteroid_field = read_asteroid_field(input).unwrap();
            assert_eq!(asteroid_field.find_monitoring_station().0, monitoring_point);
        }
    }
}
