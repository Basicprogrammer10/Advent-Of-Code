use std::collections::HashMap;

use common::{Answer, Solution};
use itertools::Itertools;

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> &'static str {
        "Haunted Wasteland"
    }

    fn part_a(&self, input: &str) -> Answer {
        let map = parse(input);

        let mut i = 0;
        let mut pos = "AAA".to_string();
        loop {
            let direction = map.instructions[i % map.instructions.len()];
            i += 1;

            let (left, right) = map.nodes.get(&pos).unwrap();
            pos = match direction {
                Direction::Left => left.to_owned(),
                Direction::Right => right.to_owned(),
            };

            if pos == "ZZZ" {
                break;
            }
        }

        i.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let map = parse(input);

        let mut pos = Vec::new();
        for (id, _) in &map.nodes {
            if id.ends_with("A") {
                pos.push(id.as_str());
            }
        }

        let mut cycles = Vec::new();
        for mut pos in pos {
            let mut cycle_len = 0;
            let mut i = 0;
            loop {
                let instruction = map.instructions[i % map.instructions.len()];
                i += 1;

                let (left, right) = map.nodes.get(pos).unwrap();
                pos = match instruction {
                    Direction::Left => left.as_str(),
                    Direction::Right => right.as_str(),
                };

                cycle_len += 1;
                if pos.ends_with("Z") {
                    cycles.push(cycle_len);
                    break;
                }
            }
        }

        cycles.iter().fold(1, |x, &acc| lcm(x, acc)).into()
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

fn parse(input: &str) -> Map {
    let (instructions, node_list) = input.split_once("\n\n").unwrap();
    let instructions = instructions
        .chars()
        .map(|char| match char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction: {}", char),
        })
        .collect();

    let mut nodes = HashMap::new();
    for node in node_list.lines() {
        let (id, children) = node.split_once(" = ").unwrap();
        let id = id.parse().unwrap();
        let children = children
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ")
            .map(|child| child.parse().unwrap())
            .collect_tuple()
            .unwrap();
        nodes.insert(id, children);
    }

    Map {
        instructions,
        nodes,
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day08;

    const CASE_A: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    const CASE_B: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day08.part_a(CASE_A), 6.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day08.part_b(CASE_B), 6.into());
    }
}
