use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result;

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
struct Image {
    layers: Vec<Layer>,
}

impl Image {
    fn final_layer(&self) -> Layer {
        let mut layer_iter = self.layers.iter();
        let mut final_layer = (*layer_iter.next().expect("No layers in image")).clone();
        for layer in layer_iter {
            for (row_index, row) in layer.rows.iter().enumerate() {
                for (col_index, pixel) in row.iter().enumerate() {
                    if final_layer.rows[row_index][col_index] == 2 {
                        final_layer.rows[row_index][col_index] = *pixel;
                    }
                }
            }
        }
        final_layer
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for layer in self.layers.iter() {
            write!(f, "{}\n", layer)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Layer {
    rows: Vec<Vec<u8>>,
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter() {
            let row_string = row
                .iter()
                .map(|pixel| pixel.to_string())
                .collect::<Vec<String>>()
                .join("");
            write!(f, "{}\n", row_string)?;
        }
        Ok(())
    }
}

impl Layer {
    fn count_pixels(&self, pixel: u8) -> u32 {
        self.rows
            .iter()
            .flatten()
            .fold(0, |acc, p| if *p == pixel { acc + 1 } else { acc })
    }
}

fn read_image_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut image_string = String::new();
    file.read_to_string(&mut image_string)?;

    Ok(image_string.trim().to_string())
}

fn parse_image(image_string: String, width: usize, height: usize) -> Result<Image> {
    let mut layers = vec![];
    let mut layer = vec![];
    let mut row: Vec<u8> = vec![];
    for pixel in image_string.chars() {
        row.push(pixel.to_digit(10).expect("Invalid pixel character") as u8);
        if row.len() == width {
            layer.push(row);
            row = vec![];
        }
        if layer.len() == height {
            layers.push(Layer { rows: layer });
            layer = vec![];
        }
    }
    Ok(Image { layers })
}

fn solve_part1() -> Result<u32> {
    let image_string = read_image_file(INPUT)?;
    let image = parse_image(image_string, 25, 6)?;
    let fewest_zero_layer = image
        .layers
        .iter()
        .min_by_key(|layer| layer.count_pixels(0))
        .expect("No image layers created");
    Ok(fewest_zero_layer.count_pixels(1) * fewest_zero_layer.count_pixels(2))
}

fn solve_part2() -> Result<String> {
    let image_string = read_image_file(INPUT)?;
    let image = parse_image(image_string, 25, 6)?;
    Ok(format!("{}", image.final_layer()))
}

fn main() -> Result<()> {
    println!("Part 1: {}", solve_part1()?);
    println!("Part 2:\n{}", solve_part2()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "input/test.txt";

    #[test]
    fn reads_image() {
        let image_string = read_image_file(TEST_INPUT).unwrap();
        assert_eq!(
            parse_image(image_string, 3, 2).unwrap(),
            Image {
                layers: vec![Layer {
                    rows: vec![vec![1, 2, 3], vec![4, 5, 6]],
                }],
            }
        )
    }
}
