#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::result;
use std::str::FromStr;

use regex::Regex;

type Result<T> = result::Result<T, Box<dyn Error>>;

const INPUT: &str = "input/input.txt";

#[derive(Debug, PartialEq)]
struct Reactions {
    reactions: HashMap<String, Reaction>,
}

#[derive(Debug, PartialEq)]
struct Reaction {
    output: ChemicalAmount,
    inputs: Vec<ChemicalAmount>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct ChemicalAmount {
    chemical: String,
    amount: u32,
}

impl FromStr for Reactions {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Reactions> {
        lazy_static! {
            static ref SOURCE_CHEMICALS: Regex =
                Regex::new(concat!(r"(?P<input_amount>\d+) (?P<input_chemical>\w+),? ",)).unwrap();
            static ref OUTPUT_CHEMICAL: Regex = Regex::new(concat!(
                r"=> (?P<output_amount>\d+) (?P<output_chemical>\w+)"
            ))
            .unwrap();
        }

        let mut reactions = HashMap::new();
        for line in s.trim().split('\n') {
            let mut inputs: Vec<ChemicalAmount> = vec![];
            for captures in SOURCE_CHEMICALS.captures_iter(line) {
                inputs.push(ChemicalAmount {
                    chemical: captures["input_chemical"].to_string(),
                    amount: captures["input_amount"].parse()?,
                });
            }
            match OUTPUT_CHEMICAL.captures(line) {
                None => {
                    return Err(From::from(
                        "Malformed reactions, no output chemical could be found",
                    ));
                }
                Some(captures) => {
                    let output = ChemicalAmount {
                        chemical: captures["output_chemical"].to_string(),
                        amount: captures["output_amount"].parse()?,
                    };
                    reactions.insert(output.chemical.clone(), Reaction { inputs, output });
                }
            };
        }

        Ok(Reactions { reactions })
    }
}

impl Reactions {
    fn new() -> Reactions {
        Reactions {
            reactions: HashMap::new(),
        }
    }
}

fn calculate_ore_required(
    reactions: &Reactions,
    produced_chemical: &ChemicalAmount,
    left_overs: &mut HashMap<String, u32>,
) -> u32 {
    let reaction = &reactions.reactions[&produced_chemical.chemical];
    let mut needed_amount = produced_chemical.amount;
    let mut left_over = 0;
    if let Some(left_over_amount) = left_overs.get(&produced_chemical.chemical) {
        left_over = *left_over_amount;
    }

    if left_over > 0 {
        if left_over >= needed_amount {
            left_overs.insert(
                produced_chemical.chemical.clone(),
                left_over - needed_amount,
            );
            return 0;
        } else {
            left_overs.insert(produced_chemical.chemical.clone(), 0);
            needed_amount -= left_over;
        }
    }

    let ratio: f32 = needed_amount as f32 / reaction.output.amount as f32;
    let production_count = ratio.ceil() as u32;
    left_overs.insert(
        produced_chemical.chemical.clone(),
        (reaction.output.amount * production_count) - needed_amount,
    );

    if reaction.inputs.len() == 1 && reaction.inputs[0].chemical == "ORE" {
        return reaction.inputs[0].amount * production_count;
    } else {
        return reaction
            .inputs
            .iter()
            .map(|input| {
                calculate_ore_required(
                    reactions,
                    &ChemicalAmount {
                        chemical: input.chemical.clone(),
                        amount: input.amount * production_count,
                    },
                    left_overs,
                )
            })
            .sum();
    }
}

fn read_reactions(filename: &str) -> Result<Reactions> {
    let reactions = read_to_string(filename)?.parse()?;
    Ok(reactions)
}

fn solve_part1(filename: &str) -> Result<u32> {
    let reactions = read_reactions(filename)?;
    let mut left_overs = HashMap::new();
    Ok(calculate_ore_required(
        &reactions,
        &ChemicalAmount {
            chemical: "FUEL".to_string(),
            amount: 1,
        },
        &mut left_overs,
    ))
}

fn solve_part2(filename: &str) -> Result<u64> {
    Ok(1)
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
    const TEST_INPUT3: &str = "input/test3.txt";
    const TEST_INPUT4: &str = "input/test4.txt";
    const TEST_INPUT5: &str = "input/test5.txt";

    fn reactions_1() -> Reactions {
        Reactions {
            reactions: vec![
                (
                    "E".to_string(),
                    Reaction {
                        output: ChemicalAmount {
                            chemical: "E".to_string(),
                            amount: 1,
                        },
                        inputs: vec![
                            ChemicalAmount {
                                chemical: "A".to_string(),
                                amount: 7,
                            },
                            ChemicalAmount {
                                chemical: "D".to_string(),
                                amount: 1,
                            },
                        ],
                    },
                ),
                (
                    "A".to_string(),
                    Reaction {
                        output: ChemicalAmount {
                            chemical: "A".to_string(),
                            amount: 10,
                        },
                        inputs: vec![ChemicalAmount {
                            chemical: "ORE".to_string(),
                            amount: 10,
                        }],
                    },
                ),
                (
                    "D".to_string(),
                    Reaction {
                        output: ChemicalAmount {
                            chemical: "D".to_string(),
                            amount: 1,
                        },
                        inputs: vec![
                            ChemicalAmount {
                                chemical: "A".to_string(),
                                amount: 7,
                            },
                            ChemicalAmount {
                                chemical: "C".to_string(),
                                amount: 1,
                            },
                        ],
                    },
                ),
                (
                    "FUEL".to_string(),
                    Reaction {
                        output: ChemicalAmount {
                            chemical: "FUEL".to_string(),
                            amount: 1,
                        },
                        inputs: vec![
                            ChemicalAmount {
                                chemical: "A".to_string(),
                                amount: 7,
                            },
                            ChemicalAmount {
                                chemical: "E".to_string(),
                                amount: 1,
                            },
                        ],
                    },
                ),
                (
                    "B".to_string(),
                    Reaction {
                        output: ChemicalAmount {
                            chemical: "B".to_string(),
                            amount: 1,
                        },
                        inputs: vec![ChemicalAmount {
                            chemical: "ORE".to_string(),
                            amount: 1,
                        }],
                    },
                ),
                (
                    "C".to_string(),
                    Reaction {
                        output: ChemicalAmount {
                            chemical: "C".to_string(),
                            amount: 1,
                        },
                        inputs: vec![
                            ChemicalAmount {
                                chemical: "A".to_string(),
                                amount: 7,
                            },
                            ChemicalAmount {
                                chemical: "B".to_string(),
                                amount: 1,
                            },
                        ],
                    },
                ),
            ]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn reads_reactions() {
        assert_eq!(read_reactions(TEST_INPUT1).unwrap(), reactions_1());
    }

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(TEST_INPUT1).unwrap(), 31);
        assert_eq!(solve_part1(TEST_INPUT2).unwrap(), 165);
        assert_eq!(solve_part1(TEST_INPUT3).unwrap(), 13312);
        assert_eq!(solve_part1(TEST_INPUT4).unwrap(), 180697);
        assert_eq!(solve_part1(TEST_INPUT5).unwrap(), 2210736);
    }
}
