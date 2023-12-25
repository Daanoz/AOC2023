#[cfg(feature = "performance")]
use ahash::{AHashMap as HashMap, AHashSet as HashSet};
#[cfg(not(feature = "performance"))]
use std::collections::{HashMap, HashSet};

use super::Solution;
use common::Answer;
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct Puzzle;

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let graph = parse_input(&input);
        let group_size = find_group_size(&graph);
        Answer::from(group_size * (graph.len() - group_size)).into()
    }

    fn solve_b(&mut self, _input: String) -> Result<Answer, String> {
        Answer::from("Merry Christmas").into()
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

fn find_group_size(graph: &Graph) -> usize {
    let nodes: Vec<&str> = graph.keys().copied().collect();

    let mut rand = rand::thread_rng();
    let mut path_traveled: HashSet<(&str, &str)> = HashSet::new();
    loop {
        let mut start_and_exit = nodes.choose_multiple(&mut rand, 2);
        let (start, exit) = (
            start_and_exit.next().unwrap(),
            start_and_exit.next().unwrap(),
        );
        path_traveled.clear();
        let mut cuts = 0;
        while find_path(graph, &mut path_traveled, &mut HashSet::new(), start, exit) {
            cuts += 1;
        }
        if cuts == 3 {
            let mut visited: HashSet<&str> = HashSet::new();
            find_isolated_group(graph, &path_traveled, &mut visited, start);
            if (graph.len() - visited.len()) != 0 && (graph.len() != visited.len()) {
                break visited.len();
            } else {
                println!("Randomness failed, trying again")
            }
        }
    }
}

fn find_path<'a>(
    graph: &Graph<'a>,
    path_traveled: &mut HashSet<(&'a str, &'a str)>,
    visited: &mut HashSet<&'a str>,
    start: &'a str,
    exit: &'a str,
) -> bool {
    if start == exit {
        return true;
    }
    visited.insert(start);
    for &neighbor in graph.get(&start).unwrap_or(&HashSet::new()) {
        if !visited.contains(&neighbor)
            && !path_traveled.contains(&(start, neighbor))
            && find_path(graph, path_traveled, visited, neighbor, exit)
        {
            if path_traveled.contains(&(neighbor, start)) {
                path_traveled.remove(&(neighbor, start));
            } else {
                path_traveled.insert((start, neighbor));
            }
            return true;
        }
    }
    false
}

fn find_isolated_group<'a>(
    graph: &Graph<'a>,
    path_traveled: &HashSet<(&'a str, &'a str)>,
    visited: &mut HashSet<&'a str>,
    node: &'a str,
) {
    visited.insert(node);
    for &neighbor in graph.get(&node).unwrap_or(&HashSet::new()) {
        if !visited.contains(&neighbor) && !path_traveled.contains(&(node, neighbor)) {
            find_isolated_group(graph, path_traveled, visited, neighbor);
        }
    }
}

fn parse_input(input: &str) -> Graph {
    let mut graph = HashMap::new();
    input.lines().for_each(|line| {
        let (key, values) = line.split_once(": ").unwrap();
        graph
            .entry(key)
            .or_insert_with(HashSet::new)
            .extend(values.split(' '));
        values.split(' ').for_each(|v| {
            graph.entry(v).or_insert_with(HashSet::new).insert(key);
        });
    });
    graph
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(""))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(""))
        )
    }
}
