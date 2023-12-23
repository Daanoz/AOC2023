use std::collections::BTreeMap;

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

type Coord = (usize, usize);

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let path_finder = PathFinder::new(parse_input(&input));
        Answer::from(path_finder.find_longest_path()).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let input = input.replace(['>', 'v'], ".");
        let path_finder = PathFinder::new(parse_input(&input));
        Answer::from(path_finder.find_longest_path()).into()
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

struct PathFinder {
    grid: Vec<Vec<Cell>>,
}
impl PathFinder {
    fn new(grid: Vec<Vec<Cell>>) -> Self {
        Self { grid }
    }

    pub fn find_longest_path(&self) -> usize {
        let dim = (self.grid[0].len(), self.grid.len());
        let mut graph: BTreeMap<Coord, BTreeMap<Coord, usize>> = Default::default();

        // find all paths
        self.grid.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                let coord = (x, y);
                let deltas = match cell {
                    Cell::Space => vec![(-1, 0), (0, -1), (1, 0), (0, 1)],
                    Cell::SlopeRight => vec![(1, 0)],
                    Cell::SlopeDown => vec![(0, 1)],
                    _ => return,
                };
                deltas
                    .iter()
                    .filter_map(|d| {
                        let (nx, ny) = (x as isize + d.0, y as isize + d.1);
                        if nx < 0 || ny < 0 || nx >= dim.0 as isize || ny >= dim.1 as isize {
                            return None;
                        }
                        Some((nx as usize, ny as usize))
                    })
                    .filter(|(cx, cy)| self.grid[*cy][*cx] != Cell::Wall)
                    .for_each(|n| {
                        graph.entry(coord).or_default().insert(n, 1);
                    });
            })
        });

        // compress graph paths
        while let Some((&(x, y), _)) = graph.iter().find(|(_, n)| n.len() == 2) {
            // remove entry for current node which has 2 neighbors
            let neighbors = graph.remove(&(x, y)).unwrap();
            let mut neigbors_iter = neighbors.iter();
            // get two entries
            let first_n = neigbors_iter.next().unwrap();
            let second_n = neigbors_iter.next().unwrap();
            // replace first entry
            let current_neighbors = graph.get_mut(first_n.0).unwrap();
            current_neighbors.remove(&(x, y));
            current_neighbors.insert(*second_n.0, first_n.1 + second_n.1);
            // replace second entry
            let current_neighbors = graph.get_mut(second_n.0).unwrap();
            current_neighbors.remove(&(x, y));
            current_neighbors.insert(*first_n.0, first_n.1 + second_n.1);
        }
        let index_of = |c: &Coord| -> usize { graph.keys().position(|ip| ip == c).unwrap() };
        // compress to simple graph
        let compressed_graph: BTreeMap<usize, Vec<(usize, usize)>> = graph
            .keys()
            .enumerate()
            .map(|(i, (x, y))| {
                let neighbors = graph
                    .get(&(*x, *y))
                    .unwrap()
                    .iter()
                    .map(|(n, d)| (index_of(n), *d))
                    .collect();
                (i, neighbors)
            })
            .collect();
        let node_count = compressed_graph.len();
        Self::dfs(
            (index_of(&(1, 0)), 0),
            index_of(&(dim.0 - 2, dim.1 - 1)),
            &compressed_graph,
            &mut vec![false; node_count],
        )
    }

    fn dfs(
        current: (usize, usize),
        end: usize,
        graph: &BTreeMap<usize, Vec<(usize, usize)>>,
        path: &mut Vec<bool>,
    ) -> usize {
        if current.0 == end {
            return current.1;
        }
        let mut max = usize::MIN;
        for entry in graph.get(&current.0).unwrap() {
            let next = entry.0;
            if path[next] {
                continue;
            }
            path[next] = true;
            let result = Self::dfs((next, current.1 + entry.1), end, graph, path);
            if result > max {
                max = result;
            }
            path[next] = false;
        }
        max
    }
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

#[derive(PartialEq)]
enum Cell {
    Wall,
    Space,
    SlopeRight,
    SlopeDown,
}
impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Space,
            '>' => Cell::SlopeRight,
            'v' => Cell::SlopeDown,
            _ => panic!("Unknown cell type: {}", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(94))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(154))
        )
    }
}
