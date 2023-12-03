use std::collections::HashMap;

use super::Solution;
use async_trait::async_trait;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let (symbols, numbers) = parse_input(input);
        let attached_numbers = get_attached_numbers(&symbols, &numbers);
        Answer::from(attached_numbers.iter().map(|n| n.value).sum::<u32>()).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let (symbols, numbers) = parse_input(input);
        let gears: Vec<u32> = numbers
            .iter()
            .filter_map(|n| {
                n.neighbor_coords()
                    .iter()
                    .find(|c| symbols.get(c).filter(|s| s == &&'*').is_some())
                    .map(|c| (*c, n))
            })
            .fold(HashMap::new(), |mut map: HashMap<(usize, usize), Vec<&Number>>, (c, n)| {
                map.entry(c).or_default().push(n);
                map
            })
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| (v.get(0).unwrap().value * v.get(1).unwrap().value))
            .collect();
        Answer::from(gears.iter().sum::<u32>()).into()
    }

    #[cfg(feature = "ui")]
    async fn get_shapes(&mut self, input: String, _rect: egui::Rect) -> Option<Vec<egui::Shape>> {
        use egui::epaint::*;

        let mut shapes = vec![];
        let (symbols, numbers) = parse_input(input);
        shapes.extend(symbols.keys().map(|coord| {
            Shape::Rect(RectShape::filled(
                Rect::from_min_size(Pos2::new(coord.1 as f32, coord.0 as f32), Vec2::splat(1.0)),
                Rounding::ZERO,
                Color32::YELLOW,
            ))
        }));
        shapes.extend(numbers.iter().map(|n| {
            Shape::Rect(RectShape::filled(
                Rect::from_min_size(Pos2::new(n.coord.1 as f32, n.coord.0 as f32), Vec2::new(n.digits as f32, 1.0)),
                Rounding::ZERO,
                Color32::BLUE,
            ))
        }));
        let attached_numbers = get_attached_numbers(&symbols, &numbers);
        shapes.extend(attached_numbers.iter().map(|n| {
            Shape::Rect(RectShape::filled(
                Rect::from_min_size(Pos2::new(n.coord.1 as f32, n.coord.0 as f32), Vec2::new(n.digits as f32, 1.0)),
                Rounding::ZERO,
                Color32::GREEN,
            ))
        }));
        Some(shapes)
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
            list
                .into_iter()
                .map(Number::new)
                .collect::<Vec<Number>>()
        })
        .collect();
    (symbols, numbers)
}

fn get_attached_numbers<'a>(
    symbols: &HashMap<Coord, char>,
    numbers: &'a [Number],
) -> Vec<&'a Number> {
    numbers
        .iter()
        .filter(|n| {
            let neighbors = n.neighbor_coords();
            neighbors.iter().any(|c| symbols.contains_key(c))
        })
        .collect()
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
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(4361))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(467835))
        )
    }
}
