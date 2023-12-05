use std::{ops::Range, str::FromStr};

use super::Solution;
use async_trait::async_trait;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let (seeds, maps) = parse_input(input);
        let locations = seeds.into_iter().map(|s| {
            let mut value = s;
            for map in &maps {
                value = map.convert(value);
            }
            value
        });
        Answer::from(locations.min()).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let (seeds, maps) = parse_input(input);
        let locations = seeds
            .chunks(2)
            .into_iter()
            .flat_map(|s| vec![0; s[1]].into_iter().enumerate().map(|(i, _)| s[0] + i))
            .map(|s| {
                let mut value = s;
                for map in &maps {
                    value = map.convert(value);
                }
                value
            });
        Answer::from(locations.min()).into()
    }

    #[cfg(feature = "ui")]
    async fn get_shapes(
        &mut self,
        _input: String,
        _rect: egui::Rect,
    ) -> Option<Vec<ui_support::Shape>> {
        None
    }
}

fn parse_input(input: String) -> (Vec<usize>, Vec<ConversionMap>) {
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let maps = sections
        .map(|s| s.parse::<ConversionMap>().unwrap())
        .collect::<Vec<_>>();
    (seeds, maps)
}

#[derive(Debug)]
struct ConversionMap {
    #[allow(dead_code)]
    name: String,
    ranges: Vec<ConversionRange>,
}
impl ConversionMap {
    fn convert(&self, value: usize) -> usize {
        match self.ranges.iter().find(|r| r.from.contains(&value)) {
            Some(range) => range.to + (value - range.from.start),
            None => value,
        }
    }
}
impl FromStr for ConversionMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let name = lines.next().ok_or("Missing line")?;
        let ranges = lines
            .map(|r| r.parse::<ConversionRange>().unwrap())
            .collect();
        Ok(Self {
            name: name.into(),
            ranges,
        })
    }
}

#[derive(Debug)]
struct ConversionRange {
    from: Range<usize>,
    to: usize,
}
impl FromStr for ConversionRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<usize> = s
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Ok(Self {
            from: lines[1]..lines[1] + lines[2],
            to: lines[0],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(35))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(46))
        )
    }
}
