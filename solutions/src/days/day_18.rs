use std::{num::ParseIntError, str::FromStr};

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let dig_plans = parse_input(&input);
        let dig_border = dig(dig_plans);
        Answer::from(dig_area(&dig_border)).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let dig_plans = parse_input(&input);
        let dig_plans = dig_plans.into_iter().map(|plan| plan.alt_mode()).collect();
        let dig_border = dig(dig_plans);
        Answer::from(dig_area(&dig_border)).into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(
        &mut self,
        input: String,
        request: ui_support::DisplayRequest,
    ) -> Option<ui_support::DisplayResult> {
        Some(build_shapes_for_ui(input, request))
    }
}

fn parse_input(input: &str) -> Vec<DigPlan> {
    input
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

type DigBorder = Vec<(isize, isize)>;
fn dig(plans: Vec<DigPlan>) -> DigBorder {
    let mut current = (0_isize, 0_isize);
    let mut ground = Vec::from([current]);
    for plan in plans {
        match plan.direction {
            Direction::Down => {
                current = (current.0, current.1 + plan.distance);
                ground.push(current);
            }
            Direction::Up => {
                current = (current.0, current.1 - plan.distance);
                ground.push(current);
            }
            Direction::Left => {
                current = (current.0 - plan.distance, current.1);
                ground.push(current);
            }
            Direction::Right => {
                current = (current.0 + plan.distance, current.1);
                ground.push(current);
            }
        }
    }
    ground
}

// Shoelace formula
fn dig_area(dig_border: &DigBorder) -> usize {
    let mut border_length = 0;
    let (s1, s2) = dig_border
        .windows(2)
        .fold((0_isize, 0_isize), |(s1, s2), edges| {
            border_length +=
                ((edges[0].0 - edges[1].0).abs() + (edges[0].1 - edges[1].1).abs()).abs();
            let (c1, c2) = (edges[0], edges[1]);
            (s1 + c1.0 * c2.1, s2 + c1.1 * c2.0)
        });
    let area = ((s1 - s2).abs() + border_length) / 2;
    (area + 1) as usize
}

#[derive(Debug, Clone)]
struct DigPlan {
    direction: Direction,
    distance: isize,
    color: String,
}

impl DigPlan {
    fn alt_mode(self) -> Self {
        let distance = isize::from_str_radix(&self.color[0..5], 16).unwrap();
        let dir = self.color.chars().nth(5).unwrap();

        Self {
            direction: match dir {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Unknown direction: {}", dir),
            },
            distance,
            ..self
        }
    }
}

impl FromStr for DigPlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let direction = parts.next().ok_or("No direction")?;
        let distance = parts.next().ok_or("No distance")?;
        let color = parts
            .next()
            .ok_or("No color")?
            .strip_prefix("(#")
            .ok_or("No color prefix")?
            .strip_suffix(')')
            .ok_or("No color suffix")?;

        Ok(Self {
            direction: direction.parse()?,
            distance: distance.parse().map_err(|e: ParseIntError| e.to_string())?,
            color: color.into(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(format!("Unknown direction: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(62))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(952408144115_isize))
        )
    }
}

#[cfg(feature = "ui")]
fn build_shapes_for_ui(
    input: String,
    request: ui_support::DisplayRequest,
) -> ui_support::DisplayResult {
    use egui::epaint::{Color32, Shape, Stroke};

    let mut plans = parse_input(&input);
    let mut scale = 1.0;
    if request.result_index == 1 {
        plans = plans.into_iter().map(|plan| plan.alt_mode()).collect();
        scale = 0.0001; // scale down to avoid rendering issues
    }
    let dig = dig(plans);
    let ranges = dig
        .iter()
        .fold((0, 0, 0, 0), |(min_x, min_y, max_x, max_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
        });
    let offset: (f32, f32) = (ranges.0 as f32 - 0.5, ranges.1 as f32 - 0.5);
    let largest_range = (ranges.2 - ranges.0).max(ranges.3 - ranges.1) as f32 * scale;
    let stroke_width = largest_range / 1000.0;

    let shapes: Vec<ui_support::DisplayData> = dig
        .windows(2)
        .map(|coords: &[(isize, isize)]| {
            let (c1, c2) = (coords[0], coords[1]);
            Shape::LineSegment {
                points: [
                    egui::Pos2::new(
                        (c1.0 as f32 - offset.0) * scale,
                        (c1.1 as f32 - offset.1) * scale,
                    ),
                    egui::Pos2::new(
                        (c2.0 as f32 - offset.0) * scale,
                        (c2.1 as f32 - offset.1) * scale,
                    ),
                ],
                stroke: Stroke::new(stroke_width, Color32::RED),
            }
            .into()
        })
        .collect::<Vec<ui_support::DisplayData>>();
    let mut result: ui_support::DisplayResult = shapes.into();
    result.result_count = Some(2);
    result
}
