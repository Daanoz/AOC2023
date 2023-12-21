use std::collections::HashSet;

use super::Solution;
use common::Answer;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Puzzle {
    steps_a: usize,
    steps_b: usize,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            steps_a: 64,
            steps_b: 26501365,
        }
    }
}

type Grid = Vec<Vec<Cell>>;
type Coord = (usize, usize);

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let (grid, steps) = parse_input(input);
        Answer::from(run_steps(&grid, steps, self.steps_a).len()).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let (grid, mut steps) = parse_input(input);
        // Some assumptions:
        assert_eq!(grid.len(), grid[0].len(), "Grid should be square");
        assert_eq!(grid.len() % 2, 1, "Grid should be uneven");
        assert_eq!(
            steps.iter().nth(0).unwrap(),
            &((grid[0].len() / 2), (grid.len() / 2)),
            "Start should be center"
        );
        assert!(
            self.steps_b / grid.len() > 1,
            "Should require more than one grid"
        );

        // Basic scale of end result
        let size = grid.len();
        let total_width = (self.steps_b / size) - 1;

        // Determine number of full grids
        let even_grids = ((total_width + 1) / 2 * 2).pow(2);
        steps = run_steps_until_corner(&grid, steps, size * 2);
        let points_per_even_grid = steps.len();
        let odd_grids = (total_width / 2 * 2 + 1).pow(2);
        steps = make_step(&grid, &steps);
        let points_per_odd_grid = steps.len();

        let mut full_grid_steps =
            even_grids * points_per_even_grid + odd_grids * points_per_odd_grid;

        let start = 0;
        let center = size / 2;
        let end = size - 1;

        // straight edges
        let no_steps = size - 1;
        full_grid_steps += [
            (start, center),
            (end, center),
            (center, end),
            (center, start),
        ]
        .into_par_iter()
        .map(|start| run_steps(&grid, HashSet::from([start]), no_steps).len())
        .sum::<usize>();

        // all diagonal corners
        let diagonal_count = total_width + 1;
        let no_steps_small = size / 2 - 1;
        let no_steps_large = ((size * 3) / 2 - 1) - no_steps_small;
        full_grid_steps += [(start, end), (start, start), (end, start), (end, end)]
            .into_par_iter()
            .map(|start| {
                let small_diagonal_steps = run_steps(&grid, HashSet::from([start]), no_steps_small);
                let small_diagonal_step_count = small_diagonal_steps.len();
                let large_steps = run_steps(&grid, small_diagonal_steps, no_steps_large).len();
                diagonal_count * small_diagonal_step_count + large_steps * total_width
            })
            .sum::<usize>();

        Answer::from(full_grid_steps).into()
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

fn parse_input(input: String) -> (Grid, HashSet<Coord>) {
    let mut steps = HashSet::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        steps.insert((x, y));
                        return Cell::Garden;
                    }
                    Cell::from(c)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (grid, steps)
}

/// Not sure if this works on every input, but it saves ~100ms
fn run_steps_until_corner(grid: &Grid, mut steps: HashSet<Coord>, n: usize) -> HashSet<Coord> {
    for _ in 0..n {
        steps = make_step(grid, &steps);
        if steps.contains(&(0, 0)) {
            break;
        }
    }
    steps
}

fn run_steps(grid: &Grid, mut steps: HashSet<Coord>, n: usize) -> HashSet<Coord> {
    for _ in 0..n {
        steps = make_step(grid, &steps);
    }
    steps
}

fn make_step(grid: &Grid, steps: &HashSet<Coord>) -> HashSet<Coord> {
    let mut next = HashSet::new();
    for (x, y) in steps {
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (*x as isize + dx, *y as isize + dy);
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if ny >= grid.len() || nx >= grid[0].len() {
                continue;
            }
            if grid[ny][nx] == Cell::Garden {
                next.insert((nx, ny));
            }
        }
    }
    next
}

#[derive(PartialEq)]
enum Cell {
    Garden,
    Rock,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Garden,
            '#' => Cell::Rock,
            _ => panic!("Invalid cell"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        puzzle.steps_a = 6;
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(16))
        )
    }
}
