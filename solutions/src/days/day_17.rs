#[cfg(feature = "performance")]
use ahash::AHashMap as HashMap;
#[cfg(not(feature = "performance"))]
use std::collections::HashMap;
use std::{cmp::Ordering, collections::BinaryHeap};

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

type Coord = (usize, usize);
type Grid = Vec<Vec<usize>>;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let grid = parse_input(input);
        let mut dijkstra = Dijkstra::new(&grid, 0, 3);
        Answer::from(dijkstra.find_shortest_path()).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let grid = parse_input(input);
        let mut dijkstra = Dijkstra::new(&grid, 4, 10);
        Answer::from(dijkstra.find_shortest_path()).into()
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

fn parse_input(input: String) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit") as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Grid>()
}

type VisitedCellKey = (Coord, Direction);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct QueueItem(Coord, Direction, usize);
impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
    }
}
impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Direction {
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}

struct Dijkstra<'a> {
    min_steps: usize,
    max_steps: usize,
    grid: &'a Grid,
    store_path: bool,
    path_map: HashMap<QueueItem, QueueItem>,
}

impl<'a> Dijkstra<'a> {
    fn new(grid: &'a Grid, min: usize, max: usize) -> Self {
        Self {
            min_steps: min,
            max_steps: max,
            grid,
            store_path: false,
            path_map: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn find_shortest_path(&mut self) -> usize {
        let start_cell = ((0, 0), Direction::Up);
        let start = QueueItem(start_cell.0, start_cell.1, 0);
        let mut visited: HashMap<VisitedCellKey, usize> = HashMap::from([(start_cell, 0)]);
        let mut queue: BinaryHeap<QueueItem> = BinaryHeap::from([start]);
        let exit = (self.grid[0].len() - 1, self.grid.len() - 1);

        let mut lowest_exit = usize::MAX;
        while !queue.is_empty() {
            let current = queue.pop().unwrap();
            if current.0 == exit {
                if current.2 < lowest_exit {
                    lowest_exit = current.2;
                }
                continue;
            }
            let neighbors = self.get_neighbors(current);
            for neighbor in neighbors {
                let visited_key = (neighbor.0, neighbor.1);
                if neighbor.2 < lowest_exit
                    && &neighbor.2 < visited.get(&visited_key).unwrap_or(&usize::MAX)
                {
                    visited.insert(visited_key, neighbor.2);
                    queue.push(neighbor);
                    if self.store_path {
                        self.path_map.insert(neighbor, current);
                    }
                }
            }
        }
        lowest_exit
    }

    fn get_neighbors(&self, coord: QueueItem) -> Vec<QueueItem> {
        let (x, y) = coord.0;
        let min_steps: usize = self.min_steps;
        let max_steps: usize = self.max_steps;
        let row = &self.grid[y];
        let dimensions = (self.grid[0].len() - 1, self.grid.len() - 1);
        let mut list: Vec<QueueItem> = vec![];

        for dir in [
            Direction::Down,
            Direction::Right,
            Direction::Up,
            Direction::Left,
        ] {
            if dir as u8 % 2 == coord.1 as u8 % 2 {
                // direction is either reversed or same as previous
                continue;
            }
            let mut heat_loss = coord.2;
            for delta in 1..=max_steps {
                match dir {
                    Direction::Left => {
                        if delta <= x {
                            let x2 = x - delta;
                            heat_loss += row[x2];
                            if delta >= min_steps {
                                list.push(QueueItem((x2, y), Direction::Left, heat_loss));
                            }
                        }
                    }
                    Direction::Up => {
                        if delta <= y {
                            let y2 = y - delta;
                            heat_loss += self.grid[y2][x];
                            if delta >= min_steps {
                                list.push(QueueItem((x, y2), Direction::Up, heat_loss));
                            }
                        }
                    }
                    Direction::Right => {
                        if x + delta <= dimensions.1 {
                            let x2 = x + delta;
                            heat_loss += row[x2];
                            if delta >= min_steps {
                                list.push(QueueItem((x2, y), Direction::Left, heat_loss));
                            }
                        }
                    }
                    Direction::Down => {
                        if y + delta <= dimensions.0 {
                            let y2 = y + delta;
                            heat_loss += self.grid[y2][x];
                            if delta >= min_steps {
                                list.push(QueueItem((x, y2), Direction::Down, heat_loss));
                            }
                        }
                    }
                }
            }
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(102))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(94))
        )
    }
}

#[cfg(feature = "ui")]
fn build_shapes_for_ui(input: String) -> Vec<ui_support::DisplayData> {
    use egui::epaint::{Color32, Rect, RectShape, Shape, Stroke};

    let grid = parse_input(input);
    let gradient = [
        Color32::from_rgb(26, 152, 80),
        Color32::from_rgb(102, 189, 99),
        Color32::from_rgb(166, 217, 106),
        Color32::from_rgb(217, 239, 139),
        Color32::from_rgb(255, 255, 191),
        Color32::from_rgb(254, 224, 139),
        Color32::from_rgb(253, 174, 97),
        Color32::from_rgb(244, 109, 67),
        Color32::from_rgb(215, 48, 39),
    ];

    let mut shapes: Vec<ui_support::DisplayData> =
        ui_support::render_grid(&grid, move |cell, pos| {
            vec![Some(
                Shape::Rect(RectShape::filled(
                    Rect::from_center_size(pos, (1.0, 1.0).into()),
                    0.0,
                    gradient[cell - 1],
                ))
                .into(),
            )]
        });

    let draw_path = |grid: &Grid, min: usize, max: usize, offset: f32, color: Color32| {
        let exit = (grid[0].len() - 1, grid.len() - 1);
        let mut dijkstra = Dijkstra::new(grid, min, max);
        dijkstra.store_path = true;
        let path_length = dijkstra.find_shortest_path();
        let from_top = QueueItem(exit, Direction::Down, path_length);
        let from_left = QueueItem(exit, Direction::Down, path_length);
        let mut prev_step = if dijkstra.path_map.contains_key(&from_top) {
            Some(&from_top)
        } else {
            Some(&from_left)
        };
        let mut path_shapes = vec![];
        while let Some(step) = prev_step {
            let next_step = dijkstra.path_map.get(step);
            if let Some(next) = next_step {
                path_shapes.push(Shape::LineSegment {
                    points: [
                        egui::Pos2::new(
                            step.0 .0 as f32 + 0.5 + offset,
                            step.0 .1 as f32 + 0.5 + offset,
                        ),
                        egui::Pos2::new(
                            next.0 .0 as f32 + 0.5 + offset,
                            next.0 .1 as f32 + 0.5 + offset,
                        ),
                    ],
                    stroke: Stroke::new(0.2, color),
                });
            }
            prev_step = next_step;
        }
        path_shapes
    };
    shapes.extend(
        draw_path(&grid, 0, 3, -0.2, Color32::BLUE)
            .into_iter()
            .map(|s| s.into()),
    );
    shapes.extend(
        draw_path(&grid, 4, 10, 0.2, Color32::DARK_BLUE)
            .into_iter()
            .map(|s| s.into()),
    );
    shapes
}
