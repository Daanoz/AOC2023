#[cfg(feature = "performance")]
use ahash::AHashMap as HashMap;
#[cfg(not(feature = "performance"))]
use std::collections::HashMap;

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let (symbols, numbers) = parse_input(input);
        let attached_numbers = get_attached_numbers(&symbols, &numbers);
        Answer::from(attached_numbers.map(|n| n.value).sum::<u32>()).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let (symbols, numbers) = parse_input(input);
        let gears: Vec<u32> = get_numbers_with_gears(&symbols, &numbers)
            .map(|(_, v)| (v.get(0).unwrap().value * v.get(1).unwrap().value))
            .collect();
        Answer::from(gears.iter().sum::<u32>()).into()
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

type Coord = (usize, usize);

#[derive(Debug)]
struct Number {
    coord: Coord,
    value: u32,
    digits: u32,
}

impl Number {
    fn new(digits: Vec<(Coord, u32)>) -> Self {
        let value = digits.iter().fold(0, |acc, (_, digit)| acc * 10 + digit);
        Self {
            coord: digits.first().unwrap().0,
            value,
            digits: digits.len() as u32,
        }
    }
    fn neighbor_coords(&self) -> Vec<Coord> {
        let (y, x) = self.coord;
        let mut list: Vec<Coord> = vec![];
        if y > 0 && x > 0 {
            // top left coord
            list.push((y - 1, x - 1));
        }
        if x > 0 {
            // left coord
            list.push((y, x - 1));
            list.push((y + 1, x - 1));
        }
        if y > 0 {
            // top coords
            for x in x..=x + self.digits as usize {
                list.push((y - 1, x));
            }
        }
        // bottom coords
        for x in x..=x + self.digits as usize {
            list.push((y + 1, x));
        }
        // right coord
        list.push((y, x + self.digits as usize));
        list
    }
}

fn parse_input(input: String) -> (HashMap<Coord, char>, Vec<Number>) {
    let symbols: HashMap<Coord, char> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| char != &'.' && !char.is_ascii_digit())
                .map(|(col, char)| ((row, col), char))
                .collect::<Vec<(Coord, char)>>()
        })
        .collect();
    let numbers: Vec<Number> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let (mut list, cur) = line.chars().enumerate().fold(
                (vec![], vec![]),
                |(mut list, mut current), (col, char)| {
                    if char.is_ascii_digit() {
                        current.push(((row, col), char.to_digit(10).unwrap()));
                    } else if !char.is_ascii_digit() && !current.is_empty() {
                        list.push(current);
                        current = vec![];
                    }
                    (list, current)
                },
            );
            if !cur.is_empty() {
                list.push(cur);
            }
            list.into_iter().map(Number::new).collect::<Vec<Number>>()
        })
        .collect();
    (symbols, numbers)
}

fn get_attached_numbers<'a>(
    symbols: &'a HashMap<Coord, char>,
    numbers: &'a [Number],
) -> impl Iterator<Item = &'a Number> {
    numbers.iter().filter(|n| {
        let neighbors = n.neighbor_coords();
        neighbors.iter().any(|c| symbols.contains_key(c))
    })
}

fn get_numbers_with_gears<'a>(
    symbols: &'a HashMap<Coord, char>,
    numbers: &'a [Number],
) -> impl Iterator<Item = (Coord, Vec<&'a Number>)> {
    numbers
        .iter()
        .filter_map(|n| {
            n.neighbor_coords()
                .iter()
                .find(|c| symbols.get(c).filter(|s| s == &&'*').is_some())
                .map(|c| (*c, n))
        })
        .fold(
            HashMap::new(),
            |mut map: HashMap<(usize, usize), Vec<&Number>>, (c, n)| {
                map.entry(c).or_default().push(n);
                map
            },
        )
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(4361))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(467835))
        )
    }
}

#[cfg(feature = "ui")]
fn build_shapes_for_ui(input: String) -> Vec<ui_support::DisplayData> {
    use egui::epaint::*;

    let mut shapes = vec![];
    let (symbols, numbers) = parse_input(input);
    shapes.extend(symbols.iter().enumerate().map(|(_, (coord, char))| {
        ui_support::DisplayData::text(
            Pos2::new(coord.1 as f32, coord.0 as f32),
            char.to_string(),
            1.0,
            Color32::YELLOW,
        )
    }));
    shapes.extend(numbers.iter().map(|n| {
        ui_support::DisplayData::text(
            Pos2::new(n.coord.1 as f32, n.coord.0 as f32),
            n.value.to_string(),
            1.0,
            Color32::BLUE,
        )
    }));
    shapes.extend(get_attached_numbers(&symbols, &numbers).map(|n| {
        ui_support::DisplayData::text(
            Pos2::new(n.coord.1 as f32, n.coord.0 as f32),
            n.value.to_string(),
            1.0,
            Color32::GREEN,
        )
    }));
    shapes.extend(
        get_numbers_with_gears(&symbols, &numbers).flat_map(|(gear, numbers)| {
            let mut s: Vec<ui_support::DisplayData> = numbers
                .iter()
                .map(|n| {
                    ui_support::DisplayData::text(
                        Pos2::new(n.coord.1 as f32, n.coord.0 as f32),
                        n.value.to_string(),
                        1.0,
                        Color32::RED,
                    )
                })
                .collect();
            s.push(ui_support::DisplayData::text(
                Pos2::new(gear.1 as f32, gear.0 as f32),
                "*".to_string(),
                1.0,
                Color32::RED,
            ));
            s
        }),
    );
    shapes
}
