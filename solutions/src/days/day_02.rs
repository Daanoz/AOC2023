use std::{ops::Add, str::FromStr};

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let games = parse_input(input);
        Answer::from(
            games
                .iter()
                .filter(|g| {
                    g.in_bounds(Set {
                        red: 12,
                        green: 13,
                        blue: 14,
                    })
                })
                .map(|g| g.id)
                .sum::<u32>(),
        )
        .into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let games = parse_input(input);
        Answer::from(games.iter().map(|g| g.set_power()).sum::<u32>()).into()
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

struct Game {
    id: u32,
    sets: Vec<Set>,
}
impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").unwrap();
        let (id, sets) = s.split_once(": ").unwrap();
        let id: u32 = id.parse().unwrap();
        let sets = sets
            .split("; ")
            .map(|set| {
                set.split(", ")
                    .map(|item| {
                        let (count, color) = item.split_once(' ').unwrap();
                        let count: u32 = count.parse().unwrap();
                        match color {
                            "red" => Set {
                                red: count,
                                ..Default::default()
                            },
                            "green" => Set {
                                green: count,
                                ..Default::default()
                            },
                            "blue" => Set {
                                blue: count,
                                ..Default::default()
                            },
                            _ => panic!("Unknown color {}", color),
                        }
                    })
                    .fold(Set::default(), |acc, f| acc + f)
            })
            .collect();
        Ok(Self { id, sets })
    }
}

impl Game {
    fn in_bounds(&self, bounds: Set) -> bool {
        self.sets.iter().all(|set| {
            set.red <= bounds.red && set.green <= bounds.green && set.blue <= bounds.blue
        })
    }

    fn min_cubes(&self) -> Set {
        self.sets.iter().fold(Set::default(), |min, set| Set {
            red: min.red.max(set.red),
            green: min.green.max(set.green),
            blue: min.blue.max(set.blue),
        })
    }

    fn set_power(&self) -> u32 {
        let min = self.min_cubes();
        min.red * min.green * min.blue
    }
}

#[derive(Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Add for Set {
    type Output = Set;

    fn add(self, rhs: Self) -> Self::Output {
        Set {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

fn parse_input(input: String) -> Vec<Game> {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(8))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(2286))
        )
    }
}

#[cfg(feature = "ui")]
fn build_shapes_for_ui(input: String) -> Vec<ui_support::DisplayData> {
    use egui::{
        epaint::{Color32, RectShape, Shape},
        Pos2, Rect, Rounding, Stroke,
    };
    let get_rect = |x, y, color, active: bool| {
        let pos = Rect::from_two_pos(Pos2::new(x + 0.2, y + 0.2), Pos2::new(x + 0.8, y + 0.8));
        Shape::Rect(if active {
            RectShape::filled(pos, Rounding::ZERO, color)
        } else {
            RectShape::stroke(pos, Rounding::ZERO, Stroke::new(0.05, color))
        })
    };

    let games = parse_input(input);
    games
        .into_iter()
        .enumerate()
        .flat_map(|(y, game)| {
            let min_cubes = game.min_cubes();
            let mut x = 0_f32;
            game.sets
                .into_iter()
                .flat_map(move |set| {
                    let mut cubes: Vec<Shape> = vec![];
                    for _ in 0..set.green {
                        cubes.push(get_rect(
                            x,
                            y as f32,
                            Color32::GREEN,
                            set.green == min_cubes.green,
                        ));
                        x += 1.0;
                    }
                    for _ in 0..set.blue {
                        cubes.push(get_rect(
                            x,
                            y as f32,
                            Color32::BLUE,
                            set.blue == min_cubes.blue,
                        ));
                        x += 1.0;
                    }
                    for _ in 0..set.red {
                        cubes.push(get_rect(
                            x,
                            y as f32,
                            Color32::RED,
                            set.red == min_cubes.red,
                        ));
                        x += 1.0;
                    }
                    cubes
                })
                .collect::<Vec<Shape>>()
        })
        .map(|s| s.into())
        .collect()
}
