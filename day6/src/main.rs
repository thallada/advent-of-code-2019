use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::result;

use petgraph::graph::NodeIndex;
use petgraph::{Direction, Graph};

const INPUT: &str = "input/input.txt";

type Result<T> = result::Result<T, Box<dyn Error>>;

struct OrbitMap {
    graph: Graph<String, ()>,
    map: HashMap<String, NodeIndex>,
}

fn read_orbit_map(filename: &str) -> Result<OrbitMap> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut graph = Graph::<String, ()>::new();
    let mut map: HashMap<String, NodeIndex> = HashMap::new();

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

        let mass_index = match map.get(&mass_name) {
            None => {
                let index = graph.add_node(mass_name.clone());
                map.insert(mass_name, index);
                index
            }
            Some(index) => *index,
        };
        let orbiter_index = match map.get(&orbiter_name) {
            None => {
                let index = graph.add_node(orbiter_name.clone());
                map.insert(orbiter_name, index);
                index
            }
            Some(index) => *index,
        };
        graph.update_edge(orbiter_index, mass_index, ());
    }

    Ok(OrbitMap { graph, map })
}

fn get_orbit_count(orbit_map: &OrbitMap, orbiter: NodeIndex) -> u32 {
    for neighbor in orbit_map
        .graph
        .neighbors_directed(orbiter, Direction::Outgoing)
    {
        return 1 + get_orbit_count(orbit_map, neighbor);
    }
    return 0;
}

fn get_orbit_count_checksum(orbit_map: &OrbitMap) -> u32 {
    let mut checksum = 0;

    for orbiter in orbit_map.map.keys() {
        let node = orbit_map.map.get(orbiter).expect("Incomplete orbit map");
        checksum += get_orbit_count(&orbit_map, *node);
    }

    checksum
}

fn solve_part1() -> Result<u32> {
    let orbit_map = read_orbit_map(INPUT)?;
    Ok(get_orbit_count_checksum(&orbit_map))
}

fn solve_part2() -> Result<i32> {
    let orbit_map = read_orbit_map(INPUT)?;
    let you = orbit_map.map.get("YOU").expect("YOU not found in orbit map");
    let san = orbit_map.map.get("SAN").expect("SAN not found in orbit map");
    // let mut bfs = Bfs::new(&orbit_map.graph, *you);
    // that BFS doesn't tell me the edges from node for each iteration. guess I'll roll my own
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
        let orbit_map = read_orbit_map(TEST_INPUT).unwrap();
        assert_eq!(
            format!("{:?}", orbit_map.graph),
            "Graph { \
             Ty: \"Directed\", \
             node_count: 12, \
             edge_count: 11, \
             edges: (1, 0), (2, 1), (3, 2), (4, 3), (5, 4), (6, 1), (7, 6), \
             (8, 3), (9, 4), (10, 9), (11, 10), \
             node weights: {\
             0: \"COM\", \
             1: \"B\", \
             2: \"C\", \
             3: \"D\", \
             4: \"E\", \
             5: \"F\", \
             6: \"G\", \
             7: \"H\", \
             8: \"I\", \
             9: \"J\", \
             10: \"K\", \
             11: \"L\"\
             } \
             }",
        )
    }

    #[test]
    fn gets_orbit_count_checksum() {
        let orbit_map = read_orbit_map(TEST_INPUT).unwrap();
        assert_eq!(get_orbit_count_checksum(&orbit_map), 42)
    }
}
