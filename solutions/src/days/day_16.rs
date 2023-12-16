use std::collections::HashSet;

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

type Pos = (usize, usize);
type Dimensions = (usize, usize);
type Grid = Vec<Vec<Cell>>;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let grid = parse_input(&input);
        Answer::from(find_energized_tile_count(
            &grid,
            Beam {
                pos: (0, 0),
                dir: Direction::Right,
            },
        ))
        .into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let grid = parse_input(&input);
        let dimensions: Dimensions = (grid[0].len(), grid.len());
        let mut max = 0_usize;
        for y in 0..dimensions.1 {
            max = max.max(find_energized_tile_count(
                &grid,
                Beam {
                    pos: (0, y),
                    dir: Direction::Right,
                },
            ));
            max = max.max(find_energized_tile_count(
                &grid,
                Beam {
                    pos: (dimensions.0 - 1, y),
                    dir: Direction::Left,
                },
            ));
        }
        for x in 0..dimensions.0 {
            max = max.max(find_energized_tile_count(
                &grid,
                Beam {
                    pos: (x, 0),
                    dir: Direction::Down,
                },
            ));
            max = max.max(find_energized_tile_count(
                &grid,
                Beam {
                    pos: (x, dimensions.1 - 1),
                    dir: Direction::Up,
                },
            ));
        }
        Answer::from(max).into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(
        &mut self,
        input: String,
        _request: ui_support::DisplayRequest,
    ) -> Option<ui_support::DisplayResult> {
        Some(build_shapes_for_ui(input).into())
    }
}

fn find_energized_tile_count(grid: &Grid, start_beam: Beam) -> usize {
    run_beam(&grid, start_beam).len()
}

fn run_beam(grid: &Grid, start_beam: Beam) -> HashSet<Pos> {
    let dimensions: Dimensions = (grid[0].len(), grid.len());
    let mut visited: HashSet<(usize, usize, Direction)> =
        HashSet::from_iter(vec![(start_beam.pos.0, start_beam.pos.1, start_beam.dir)]);
    let mut beams = vec![start_beam];
    while !beams.is_empty() {
        beams = beams
            .into_iter()
            .flat_map(|b| b.mv(grid, &dimensions))
            .filter(|b| !visited.contains(&(b.pos.0, b.pos.1, b.dir)))
            .collect();
        visited.extend(beams.iter().map(|b| (b.pos.0, b.pos.1, b.dir)));
    }
    visited
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<Pos>>()
}

#[derive(Clone)]
struct Beam {
    pos: Pos,
    dir: Direction,
}
impl Beam {
    pub fn mv(&self, grid: &Grid, dimensions: &Dimensions) -> Vec<Beam> {
        let cell = &grid[self.pos.1][self.pos.0];
        let mut beams = vec![];
        match cell {
            Cell::VSplit if self.dir == Direction::Left || self.dir == Direction::Right => {
                if let Some(beam) = move_pos(&self.pos, &Direction::Up, dimensions) {
                    beams.push(beam)
                }
                if let Some(beam) = move_pos(&self.pos, &Direction::Down, dimensions) {
                    beams.push(beam)
                }
            }
            Cell::HSplit if self.dir == Direction::Up || self.dir == Direction::Down => {
                if let Some(beam) = move_pos(&self.pos, &Direction::Left, dimensions) {
                    beams.push(beam)
                }
                if let Some(beam) = move_pos(&self.pos, &Direction::Right, dimensions) {
                    beams.push(beam)
                }
            }
            Cell::DownRightMirror => match self.dir {
                Direction::Right => {
                    if let Some(beam) = move_pos(&self.pos, &Direction::Down, dimensions) {
                        beams.push(beam)
                    }
                }
                Direction::Down => {
                    if let Some(beam) = move_pos(&self.pos, &Direction::Right, dimensions) {
                        beams.push(beam)
                    }
                }
                Direction::Left => {
                    if let Some(beam) = move_pos(&self.pos, &Direction::Up, dimensions) {
                        beams.push(beam)
                    }
                }
                Direction::Up => {
                    if let Some(beam) = move_pos(&self.pos, &Direction::Left, dimensions) {
                        beams.push(beam)
                    }
                }
            },
            Cell::UpRightMirror => match self.dir {
                Direction::Right => {
                    if let Some(beam) = move_pos(&self.pos, &Direction::Up, dimensions) {
                        beams.push(beam)
                    }
                }
                Direction::Down => {
                    if let Some(beam) = move_pos(&self.pos, &Direction::Left, dimensions) {
                        beams.push(beam)
                    }
                }
                Direction::Left => {
                    if let Some(beam) = move_pos(&self.pos, &Direction::Down, dimensions) {
                        beams.push(beam)
                    }
                }
                Direction::Up => {
                    if let Some(beam) = move_pos(&self.pos, &Direction::Right, dimensions) {
                        beams.push(beam)
                    }
                }
            },
            _ => {
                if let Some(beam) = move_pos(&self.pos, &self.dir, dimensions) {
                    beams.push(beam)
                }
            }
        };
        beams
    }
}

fn move_pos(pos: &Pos, dir: &Direction, dimensions: &Dimensions) -> Option<Beam> {
    match dir {
        Direction::Right if pos.0 < dimensions.0 - 1 => Some(Beam {
            pos: (pos.0 + 1, pos.1),
            dir: *dir,
        }),
        Direction::Down if pos.1 < dimensions.1 - 1 => Some(Beam {
            pos: (pos.0, pos.1 + 1),
            dir: *dir,
        }),
        Direction::Left if pos.0 > 0 => Some(Beam {
            pos: (pos.0 - 1, pos.1),
            dir: *dir,
        }),
        Direction::Up if pos.1 > 0 => Some(Beam {
            pos: (pos.0, pos.1 - 1),
            dir: *dir,
        }),
        _ => None,
    }
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Cell::VSplit,
                    '-' => Cell::HSplit,
                    '\\' => Cell::DownRightMirror,
                    '/' => Cell::UpRightMirror,
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect()
}

#[derive(Debug)]
enum Cell {
    VSplit,
    HSplit,
    DownRightMirror,
    UpRightMirror,
    Empty,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(46))
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

#[cfg(feature = "ui")]
fn build_shapes_for_ui(input: String) -> Vec<ui_support::DisplayData> {
    use egui::epaint::{CircleShape, Color32, Shape, Stroke};

    let grid = parse_input(&input);
    let energized = run_beam(
        &grid,
        Beam {
            pos: (0, 0),
            dir: Direction::Right,
        },
    );
    let mirror_color = Color32::from_rgb(255, 0, 0);
    let path_color = Color32::from_rgba_premultiplied(0, 255, 0, 75);
    grid.into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .flat_map(|(x, cell)| {
                    let mut shapes = vec![match cell {
                        Cell::HSplit => Some(Shape::LineSegment {
                            points: [
                                egui::Pos2::new(x as f32, y as f32 + 0.5),
                                egui::Pos2::new(x as f32 + 1.0, y as f32 + 0.5),
                            ],
                            stroke: Stroke::new(0.1, mirror_color),
                        }),
                        Cell::VSplit => Some(Shape::LineSegment {
                            points: [
                                egui::Pos2::new(x as f32 + 0.5, y as f32),
                                egui::Pos2::new(x as f32 + 0.5, y as f32 + 1.0),
                            ],
                            stroke: Stroke::new(0.1, mirror_color),
                        }),
                        Cell::DownRightMirror => Some(Shape::LineSegment {
                            points: [
                                egui::Pos2::new(x as f32, y as f32),
                                egui::Pos2::new(x as f32 + 1.0, y as f32 + 1.0),
                            ],
                            stroke: Stroke::new(0.1, mirror_color),
                        }),
                        Cell::UpRightMirror => Some(Shape::LineSegment {
                            points: [
                                egui::Pos2::new(x as f32, y as f32 + 1.0),
                                egui::Pos2::new(x as f32 + 1.0, y as f32),
                            ],
                            stroke: Stroke::new(0.1, mirror_color),
                        }),
                        _ => None,
                    }];
                    if energized.contains(&(x, y)) {
                        shapes.push(Some(Shape::Circle(CircleShape::filled(
                            egui::Pos2::new(x as f32 + 0.5, y as f32 + 0.5),
                            0.2,
                            path_color,
                        ))));
                    }
                    shapes
                })
                .filter_map(|s| s)
                .collect::<Vec<Shape>>()
        })
        .map(|s| s.into())
        .collect()
}
