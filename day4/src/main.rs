const INPUT_MIN: u32 = 245318;
const INPUT_MAX: u32 = 765747;

fn solve_part1() -> u32 {
    let mut counter = 0;
    for num in INPUT_MIN..=INPUT_MAX {
        let num_string = num.to_string();
        let mut previous = None;
        let mut has_double = false;
        let mut decreasing = false;
        for c in num_string.chars() {
            match previous {
                None => previous = Some(c),
                Some(p) => {
                    if p == c {
                        has_double = true;
                    }
                    if p.to_digit(10) > c.to_digit(10) {
                        decreasing = true;
                        break;
                    }
                    previous = Some(c);
                }
            }
        }
        if has_double && !decreasing {
            counter += 1;
        }
    }
    counter
}

fn solve_part2() -> u32 {
    // too lazy to DRY it up
    let mut counter = 0;
    for num in INPUT_MIN..=INPUT_MAX {
        let num_string = num.to_string();
        let mut previous = None;
        let mut has_double = false;
        let mut matching_group_count = 1;
        let mut decreasing = false;
        for c in num_string.chars() {
            match previous {
                None => previous = Some(c),
                Some(p) => {
                    if p == c {
                        matching_group_count += 1;
                    } else {
                        if matching_group_count == 2 {
                            has_double = true;
                        }
                        matching_group_count = 1;
                    }
                    if p.to_digit(10) > c.to_digit(10) {
                        decreasing = true;
                        break;
                    }
                    previous = Some(c);
                }
            }
        }
        if (matching_group_count == 2 || has_double) && !decreasing {
            counter += 1;
        }
    }
    counter
}

fn main() {
    println!("Part 1: {}", solve_part1());
    println!("Part 2: {}", solve_part2());
}
