use std::collections::HashSet;

use super::Solution;
use common::Answer;

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

        // round up
        let even_grids = ((total_width + 1) / 2 * 2).pow(2);
        steps = run_steps(&grid, steps, size * 2);
        let points_per_even_grid = steps.len();

        // round down
        let odd_grids = (total_width / 2 * 2 + 1).pow(2);
        steps = make_step(&grid, &steps);
        let points_per_odd_grid = steps.len();

        let mut full_grid_steps =
            even_grids * points_per_even_grid + odd_grids * points_per_odd_grid;

        // straight edges
        let no_steps = size - 1;
        let right_steps = run_steps(&grid, HashSet::from([(0, size / 2)]), no_steps).len();
        let left_steps = run_steps(&grid, HashSet::from([(size - 1, size / 2)]), no_steps).len();
        let top_steps = run_steps(&grid, HashSet::from([(size / 2, size - 1)]), no_steps).len();
        let bottom_steps = run_steps(&grid, HashSet::from([(size / 2, 0)]), no_steps).len();
        full_grid_steps += right_steps + left_steps + top_steps + bottom_steps;

        // all diagonal corners
        let diagonal_count = total_width + 1;
        let no_steps = size / 2 - 1;
        let str_steps = run_steps(&grid, HashSet::from([(0, size - 1)]), no_steps);
        let sbr_steps = run_steps(&grid, HashSet::from([(0, 0)]), no_steps);
        let sbl_steps = run_steps(&grid, HashSet::from([(size - 1, 0)]), no_steps);
        let stl_steps = run_steps(&grid, HashSet::from([(size - 1, size - 1)]), no_steps);
        full_grid_steps += (str_steps.len() + sbr_steps.len() + sbl_steps.len() + stl_steps.len()) * diagonal_count;
        // other diagonal corners, continue from previous
        let no_steps = ((size * 3) / 2 - 1) - no_steps;
        let ltr_steps = run_steps(&grid, str_steps, no_steps).len();
        let lbr_steps = run_steps(&grid, sbr_steps, no_steps).len();
        let lbl_steps = run_steps(&grid, sbl_steps, no_steps).len();
        let ltl_steps = run_steps(&grid, stl_steps, no_steps).len();
        full_grid_steps += (ltr_steps + lbr_steps + lbl_steps + ltl_steps) * total_width;

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
