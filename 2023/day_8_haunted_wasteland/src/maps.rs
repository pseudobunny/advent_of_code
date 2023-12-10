use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};
use num::integer::lcm;

use crate::parsing::get_node_components;

type Network = HashMap<String, Node>;

struct Node {
    left: String,
    right: String,
}

enum Direction {
    Left,
    Right,
}

fn instructions_to_directions(instructions: &str) -> Result<Vec<Direction>> {
    instructions
        .chars()
        .map(|c| match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow!(format!("{} is not a valid direction", c))),
        })
        .collect()
}

pub fn construct_map(lines: &[String]) -> Result<Map> {
    let filtered_lines: Vec<&str> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let instructions = filtered_lines[0];
    let directions = instructions_to_directions(instructions)?;

    let network_result: Result<Network> = filtered_lines[1..]
        .iter()
        .map(|&(mut line)| {
            let (node_id, (left, right)) = get_node_components(&mut line)?;

            Ok((
                node_id.to_string(),
                Node {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            ))
        })
        .collect();

    let network = network_result?;

    Ok(Map {
        directions,
        network,
    })
}

pub struct Map {
    directions: Vec<Direction>,
    network: Network,
}

impl Map {
    fn next_node<'s>(&'s self, node_id: &str, step: usize) -> Result<&'s str> {
        let direction = &self.directions[step % self.directions.len()];

        let current_node = self
            .network
            .get(node_id)
            .ok_or(anyhow!("Could not find node {node_id} in network"))?;

        Ok(match direction {
            Direction::Left => &current_node.left,
            Direction::Right => &current_node.right,
        })
    }

    fn steps_to_end(&self, start: &str) -> Result<usize> {
        let mut steps = 0;
        let mut current_node_id = start;

        while !ends_in(current_node_id, 'Z') {
            current_node_id = self.next_node(current_node_id, steps)?;
            steps += 1;
        }

        Ok(steps)
    }
}

fn ends_in(node_id: &str, ending_char: char) -> bool {
    node_id.chars().last().unwrap_or(' ') == ending_char
}

pub fn part_one_total_steps(map: &Map) -> Result<usize> {
    map.steps_to_end("AAA")
}

pub fn part_two_total_steps(map: &Map) -> Result<usize> {
    let route_steps = map
        .network
        .keys()
        .filter(|node_id| ends_in(node_id, 'A'))
        .map(|node_id| map.steps_to_end(&node_id))
        .collect::<Result<Vec<usize>>>()?;

    Ok(route_steps.iter().fold(1, |acc, &steps| lcm(acc, steps)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "
    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    ";

    const TEST_INPUT_2: &str = "
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    ";

    const TEST_INPUT_3: &str = "
    LR

    AAA = (AAB, XXX)
    AAB = (XXX, AAZ)
    AAZ = (AAB, XXX)
    BBA = (BBB, XXX)
    BBB = (BBC, BBC)
    BBC = (BBZ, BBZ)
    BBZ = (BBB, BBB)
    XXX = (XXX, XXX)
    ";

    fn lines_1() -> Vec<String> {
        TEST_INPUT_1.split("\n").map(|s| s.to_string()).collect()
    }

    fn lines_2() -> Vec<String> {
        TEST_INPUT_2.split("\n").map(|s| s.to_string()).collect()
    }

    fn lines_3() -> Vec<String> {
        TEST_INPUT_3.split("\n").map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part_one() {
        let map_1 = construct_map(&lines_1()).unwrap();
        let map_2 = construct_map(&lines_2()).unwrap();

        assert_eq!(part_one_total_steps(&map_1).unwrap(), 2);
        assert_eq!(part_one_total_steps(&map_2).unwrap(), 6);
    }

    #[test]
    fn test_part_two() {
        let map_3 = construct_map(&lines_3()).unwrap();

        assert_eq!(part_two_total_steps(&map_3).unwrap(), 6);
    }
}
