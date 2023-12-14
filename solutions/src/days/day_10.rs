use std::{collections::HashMap, str::FromStr};

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

type Coord = (usize, usize);
type Grid = Vec<Vec<Cell>>;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let grid = parse_input(input);
        let path = find_path(&grid);
        Answer::from(path.len() / 2).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let grid = parse_input(input);
        let path = find_path(&grid);
        let enclosed = find_enclosed_ground_cells(&grid, &path);
        Answer::from(enclosed.len()).into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(&mut self, input: String) -> Option<Vec<ui_support::DisplayData>> {
        Some(build_shapes_for_ui(input))
    }
}

fn parse_input(input: String) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Cell::from_str(&c.to_string()).unwrap())
                .collect()
        })
        .collect()
}

fn find_path(grid: &Grid) -> Vec<Coord> {
    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, cell)| match cell {
                Cell::Start => Some((x, y)),
                _ => None,
            })
        })
        .expect("Start point");
    let start = start_pos;
    let mut prev = start;
    let mut current = Cell::find_first_step(grid, start);
    let mut path = vec![current];
    while current != start {
        let cell = &grid[current.1][current.0];
        let next = cell.make_step(prev, current);
        prev = current;
        current = next;
        path.push(next);
    }
    path
}

fn find_enclosed_ground_cells(grid: &Grid, path: &[Coord]) -> Vec<Coord> {
    let path_map = HashMap::from_iter(path.iter().map(|c| {
        let cell = &grid[c.1][c.0];
        if cell == &Cell::Start {
            (*c, Cell::replace_start(grid, *c))
        } else {
            (*c, grid[c.1][c.0].clone())
        }
    }));
    let enclosed_cells = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| get_enclosed(y, row, &path_map))
        .collect::<Vec<Coord>>();
    enclosed_cells
}

fn get_enclosed(y: usize, row: &[Cell], path: &HashMap<Coord, Cell>) -> Vec<Coord> {
    let mut border_count = 0;
    let mut corner_hit: Option<&Cell> = None;
    let mut coords: Vec<Coord> = vec![];
    for (x, cell) in row.iter().enumerate() {
        if let Some(path_cell) = path.get(&(x, y)) {
            match path_cell {
                Cell::Vertical => {
                    border_count += 1;
                }
                Cell::BendNE | Cell::BendSE => {
                    corner_hit = Some(cell);
                }
                Cell::BendNW => {
                    if corner_hit == Some(&Cell::BendSE) {
                        border_count += 1;
                    }
                    corner_hit = None;
                }
                Cell::BendSW => {
                    if corner_hit == Some(&Cell::BendNE) {
                        border_count += 1;
                    }
                    corner_hit = None;
                }
                _ => {}
            }
        } else if border_count % 2 == 1 {
            coords.push((x, y));
        }
    }
    coords
}

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Start,
    Horizontal,
    Vertical,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
    Ground,
}

impl Cell {
    fn replace_start(grid: &Grid, (s_x, s_y): Coord) -> Cell {
        let mut options = vec![
            Cell::Horizontal,
            Cell::Vertical,
            Cell::BendNE,
            Cell::BendNW,
            Cell::BendSE,
            Cell::BendSW,
        ];
        if s_x > 0 {
            match &grid[s_y][s_x - 1] {
                Cell::Horizontal | Cell::BendNE | Cell::BendSE => options
                    .retain(|c| *c == Cell::Horizontal || *c == Cell::BendNW || *c == Cell::BendSW),
                _ => (),
            };
        }
        if s_y > 0 {
            match &grid[s_y - 1][s_x] {
                Cell::Vertical | Cell::BendSW | Cell::BendSE => options
                    .retain(|c| *c == Cell::Vertical || *c == Cell::BendNW || *c == Cell::BendNE),
                _ => (),
            };
        }
        if s_y < grid.len() - 1 {
            match &grid[s_y + 1][s_x] {
                Cell::Vertical | Cell::BendNW | Cell::BendNE => options
                    .retain(|c| *c == Cell::Vertical || *c == Cell::BendSW || *c == Cell::BendSE),
                _ => (),
            };
        }
        if s_x < grid[0].len() - 1 {
            match &grid[s_y][s_x + 1] {
                Cell::Horizontal | Cell::BendNW | Cell::BendSW => options
                    .retain(|c| *c == Cell::Horizontal || *c == Cell::BendNE || *c == Cell::BendSE),
                _ => (),
            };
        }
        assert!(
            options.len() == 1,
            "Should have one valid option for start cell"
        );
        options[0].to_owned()
    }
    fn find_first_step(grid: &Grid, (s_x, s_y): Coord) -> Coord {
        let start_cell = Self::replace_start(grid, (s_x, s_y));
        match start_cell {
            Cell::Horizontal => (s_x + 1, s_y),
            Cell::BendNE | Cell::BendNW => (s_x, s_y - 1),
            Cell::BendSE | Cell::BendSW | Cell::Vertical => (s_x, s_y + 1),
            _ => panic!("Invalid start cell"),
        }
    }
    fn make_step(&self, (o_x, o_y): Coord, (c_x, c_y): Coord) -> Coord {
        match self {
            Cell::Start => panic!("Cannot step from start"),
            Cell::Horizontal => {
                assert_eq!(c_y, o_y);
                if c_x > o_x {
                    (c_x + 1, c_y)
                } else {
                    (c_x - 1, c_y)
                }
            }
            Cell::Vertical => {
                assert_eq!(c_x, o_x);
                if c_y > o_y {
                    (c_x, c_y + 1)
                } else {
                    (c_x, c_y - 1)
                }
            }
            Cell::BendNE => {
                assert!(c_x <= o_x);
                if o_y < c_y {
                    (c_x + 1, c_y)
                } else {
                    (c_x, c_y - 1)
                }
            }
            Cell::BendNW => {
                assert!(c_x >= o_x);
                if o_y < c_y {
                    (c_x - 1, c_y)
                } else {
                    (c_x, c_y - 1)
                }
            }
            Cell::BendSE => {
                assert!(c_x <= o_x);
                if o_y > c_y {
                    (c_x + 1, c_y)
                } else {
                    (c_x, c_y + 1)
                }
            }
            Cell::BendSW => {
                assert!(c_x >= o_x);
                if o_y > c_y {
                    (c_x - 1, c_y)
                } else {
                    (c_x, c_y + 1)
                }
            }
            Cell::Ground => panic!("Cannot step from ground"),
        }
    }
}

impl FromStr for Cell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Cell::Start),
            "-" => Ok(Cell::Horizontal),
            "|" => Ok(Cell::Vertical),
            "L" => Ok(Cell::BendNE),
            "J" => Ok(Cell::BendNW),
            "F" => Ok(Cell::BendSE),
            "7" => Ok(Cell::BendSW),
            "." => Ok(Cell::Ground),
            _ => Err(format!("Invalid cell: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const TEST_INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(4))
        );
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT2)),
            Ok(Answer::from(8))
        )
    }

    const TEST_INPUT3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const TEST_INPUT4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT3)),
            Ok(Answer::from(8))
        );
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT4)),
            Ok(Answer::from(10))
        )
    }
}

#[cfg(feature = "ui")]
fn build_shapes_for_ui(input: String) -> Vec<ui_support::DisplayData> {
    use egui::epaint::{CircleShape, Color32, PathShape, Shape, Stroke};

    let grid = parse_input(input);
    let path = find_path(&grid);
    let enclosed = find_enclosed_ground_cells(&grid, &path);
    let no_path_color = Color32::from_rgb(255, 0, 0);
    let path_color = Color32::from_rgb(0, 255, 0);
    let enclosed_color = Color32::from_rgb(0, 255, 255);
    grid.into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .flat_map(|(x, cell)| {
                    let stroke_color = if path.contains(&(x, y)) {
                        path_color
                    } else if enclosed.contains(&(x, y)) {
                        enclosed_color
                    } else {
                        no_path_color
                    };
                    match cell {
                        Cell::Start => Some(Shape::Circle(CircleShape {
                            center: egui::Pos2::new(x as f32 + 0.5, y as f32 + 0.5),
                            radius: 0.5,
                            fill: Color32::from_rgb(0, 255, 0),
                            stroke: Stroke::new(0.1, Color32::from_rgb(0, 0, 0)),
                        })),
                        Cell::Horizontal => Some(Shape::LineSegment {
                            points: [
                                egui::Pos2::new(x as f32, y as f32 + 0.5),
                                egui::Pos2::new(x as f32 + 1.0, y as f32 + 0.5),
                            ],
                            stroke: Stroke::new(0.1, stroke_color),
                        }),
                        Cell::Vertical => Some(Shape::LineSegment {
                            points: [
                                egui::Pos2::new(x as f32 + 0.5, y as f32),
                                egui::Pos2::new(x as f32 + 0.5, y as f32 + 1.0),
                            ],
                            stroke: Stroke::new(0.1, stroke_color),
                        }),
                        Cell::BendNE => Some(Shape::Path(PathShape {
                            points: vec![
                                egui::Pos2::new(x as f32 + 0.5, y as f32),
                                egui::Pos2::new(x as f32 + 0.5, y as f32 + 0.5),
                                egui::Pos2::new(x as f32 + 1.0, y as f32 + 0.5),
                            ],
                            closed: false,
                            fill: Color32::TRANSPARENT,
                            stroke: Stroke::new(0.1, stroke_color),
                        })),
                        Cell::BendNW => Some(Shape::Path(PathShape {
                            points: vec![
                                egui::Pos2::new(x as f32 + 0.5, y as f32),
                                egui::Pos2::new(x as f32 + 0.5, y as f32 + 0.5),
                                egui::Pos2::new(x as f32 + 0.0, y as f32 + 0.5),
                            ],
                            closed: false,
                            fill: Color32::TRANSPARENT,
                            stroke: Stroke::new(0.1, stroke_color),
                        })),
                        Cell::BendSE => Some(Shape::Path(PathShape {
                            points: vec![
                                egui::Pos2::new(x as f32 + 0.5, y as f32 + 1.0),
                                egui::Pos2::new(x as f32 + 0.5, y as f32 + 0.5),
                                egui::Pos2::new(x as f32 + 1.0, y as f32 + 0.5),
                            ],
                            closed: false,
                            fill: Color32::TRANSPARENT,
                            stroke: Stroke::new(0.1, stroke_color),
                        })),
                        Cell::BendSW => Some(Shape::Path(PathShape {
                            points: vec![
                                egui::Pos2::new(x as f32 + 0.5, y as f32 + 1.0),
                                egui::Pos2::new(x as f32 + 0.5, y as f32 + 0.5),
                                egui::Pos2::new(x as f32 + 0.0, y as f32 + 0.5),
                            ],
                            closed: false,
                            fill: Color32::TRANSPARENT,
                            stroke: Stroke::new(0.1, stroke_color),
                        })),
                        Cell::Ground => Some(Shape::Circle(CircleShape {
                            center: egui::Pos2::new(x as f32 + 0.5, y as f32 + 0.5),
                            radius: 0.5,
                            fill: stroke_color,
                            stroke: Stroke::new(0.1, Color32::from_rgb(0, 0, 0)),
                        })),
                    }
                })
                .collect::<Vec<Shape>>()
        })
        .map(|s| s.into())
        .collect()
}
