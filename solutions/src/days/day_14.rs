use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

type Grid = Vec<Vec<Cell>>;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let rocks = parse_input(input);
        let rocks = move_direction(rocks, &Direction::North);
        Answer::from(calculate_weight(&rocks)).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let mut rocks = parse_input(input);
        let mut weights: Vec<usize> = vec![];
        let cycles = 1_000_000_000;
        for i in 0..cycles {
            rocks = move_direction(rocks, &Direction::North);
            rocks = move_direction(rocks, &Direction::West);
            rocks = move_direction(rocks, &Direction::South);
            rocks = move_direction(rocks, &Direction::East);
            let weight = calculate_weight(&rocks);
            if weights.contains(&weight) && i > 10 {
                let index: usize = weights.iter().rposition(|&x| x == weight).unwrap();
                weights.push(weight);
                let cycle_size = (weights.len() - index) - 1;
                if weights.len() > cycle_size * 2 {
                    let slice1 = &weights[weights.len() - cycle_size..];
                    let slice2 =
                        &weights[weights.len() - (cycle_size * 2)..weights.len() - cycle_size];
                    if slice1 == slice2 {
                        let remaining_cycles = 1_000_000_000 - i - 1;
                        let cycle_index = remaining_cycles % cycle_size;
                        return Ok(Answer::from(
                            weights[(weights.len() - 1 - cycle_size) + cycle_index],
                        ));
                    }
                }
            } else {
                weights.push(weight);
            }
        }
        Answer::from(calculate_weight(&rocks)).into()
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

fn parse_input(input: String) -> Grid {
    input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|l| match l {
                    'O' => Cell::Boulder,
                    '#' => Cell::Rock,
                    '.' => Cell::Space,
                    _ => panic!("invalid input"),
                })
                .collect()
        })
        .collect()
}

fn calculate_weight(grid: &Grid) -> usize {
    let rows = grid.len();
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .map(|cell| match cell {
                    Cell::Boulder => rows - y,
                    Cell::Rock | Cell::Space => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn move_direction(grid: Grid, direction: &Direction) -> Grid {
    let mut grid = grid;
    let width = grid.get(0).unwrap().len();
    match direction {
        Direction::North | Direction::South => {
            for x in 0..width {
                let mut y = 0;
                let mut delta = 1_isize;
                let grid_range = 0..grid.len();
                if direction == &Direction::South {
                    y = grid.len() - 1;
                    delta = -1;
                };

                while grid_range.contains(&y) {
                    // only role rocks if current cell is empty
                    if grid[y][x] != Cell::Space {
                        y = (y as isize + delta) as usize;
                        continue;
                    }
                    let mut y2 = (y as isize + delta) as usize;
                    while grid_range.contains(&y2) {
                        match grid[y2][x] {
                            Cell::Space => {
                                y2 = (y2 as isize + delta) as usize;
                                continue;
                            }
                            Cell::Rock => {
                                y = y2;
                                break;
                            }
                            Cell::Boulder => {
                                grid[y][x] = Cell::Boulder;
                                grid[y2][x] = Cell::Space;
                                break;
                            }
                        }
                    }
                    y = (y as isize + delta) as usize;
                }
            }
        }
        Direction::West | Direction::East => {
            for grid_row in &mut grid {
                let mut x = 0;
                let mut delta = 1_isize;
                let grid_range = 0..width;
                if direction == &Direction::East {
                    x = width - 1;
                    delta = -1;
                };

                while grid_range.contains(&x) {
                    // only role rocks if current cell is empty
                    if grid_row[x] != Cell::Space {
                        x = (x as isize + delta) as usize;
                        continue;
                    }
                    let mut x2 = (x as isize + delta) as usize;
                    while grid_range.contains(&x2) {
                        match grid_row[x2] {
                            Cell::Space => {
                                x2 = (x2 as isize + delta) as usize;
                                continue;
                            }
                            Cell::Rock => {
                                x = x2;
                                break;
                            }
                            Cell::Boulder => {
                                grid_row[x] = Cell::Boulder;
                                grid_row[x2] = Cell::Space;
                                break;
                            }
                        }
                    }
                    x = (x as isize + delta) as usize;
                }
            }
        }
    }
    grid
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, PartialEq)]
enum Cell {
    Boulder,
    Rock,
    Space,
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(136))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(64))
        )
    }
}
