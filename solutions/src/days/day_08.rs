use std::{collections::HashMap, str::FromStr};

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let (instructions, nodes) = parse_input(input);
        Answer::from(calculate_steps(&instructions, &nodes)).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let (instructions, nodes) = parse_input(input);
        let start: Vec<&Node> = nodes
            .iter()
            .filter(|(_, n)| n.is_start)
            .map(|(_, node)| node)
            .collect();
        Answer::from(calculate_smart_ghost_steps(&instructions, &nodes, start)).into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(
        &mut self,
        _input: String,
        _request: ui_support::DisplayRequest,
    ) -> Option<ui_support::DisplayResult> {
        None
    }
}

fn parse_input(input: String) -> (Vec<Direction>, HashMap<String, Node>) {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let instructions: Vec<Direction> = instructions
        .chars()
        .map(|c| c.to_string().parse::<Direction>().unwrap())
        .collect();
    let nodes: Vec<Node> = nodes.lines().map(|c| c.parse::<Node>().unwrap()).collect();
    let nodes = HashMap::from_iter(nodes.into_iter().map(|node| (node.name.clone(), node)));
    (instructions, nodes)
}

fn calculate_steps(instructions: &Vec<Direction>, nodes: &HashMap<String, Node>) -> usize {
    let mut location = "AAA";
    let mut steps = 0_usize;
    loop {
        if location == "ZZZ" {
            return steps;
        }
        let direction = instructions.get(steps % instructions.len()).unwrap();
        let node = nodes.get(location).unwrap();
        location = match direction {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        };
        steps += 1;
    }
}

fn calculate_ghost_steps<'a>(
    instructions: &'a Vec<Direction>,
    nodes: &'a HashMap<String, Node>,
    mut location: &'a Node,
) -> usize {
    let mut steps = 0_usize;
    loop {
        if location.is_end {
            return steps;
        }
        let direction = instructions.get(steps % instructions.len()).unwrap();
        location = match direction {
            Direction::Left => nodes.get(&location.left).unwrap(),
            Direction::Right => nodes.get(&location.right).unwrap(),
        };
        steps += 1;
    }
}

fn calculate_smart_ghost_steps(
    instructions: &Vec<Direction>,
    nodes: &HashMap<String, Node>,
    location: Vec<&Node>,
) -> usize {
    location
        .iter()
        .map(|n| calculate_ghost_steps(instructions, nodes, n))
        .fold(0, |prev, current| {
            if prev == 0 {
                current
            } else {
                get_lcm((prev, current))
            }
        })
}

/// Calculate the least common multiple of two numbers.
/// Based of https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor
fn get_lcm((left, right): (usize, usize)) -> usize {
    let gcd = get_gcd((left, right));
    left * right / gcd
}

/// Calculate the greatest common divisor of two numbers.
/// Based of https://en.wikipedia.org/wiki/Euclidean_algorithm
pub fn get_gcd((mut left, mut right): (usize, usize)) -> usize {
    if right > left {
        std::mem::swap(&mut left, &mut right);
    }
    loop {
        if right == 0 {
            return left;
        }
        left %= right;
        if left == 0 {
            return right;
        }
        right %= left;
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
    is_start: bool,
    is_end: bool,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, children) = s.split_once(" = (").unwrap();
        let (left, right) = children.trim_end_matches(')').split_once(", ").unwrap();
        let is_start = name.ends_with('A');
        let is_end = name.ends_with('Z');
        Ok(Node {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            is_start,
            is_end,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(2))
        );
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT2)),
            Ok(Answer::from(6))
        );
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT3)),
            Ok(Answer::from(6))
        )
    }
}
