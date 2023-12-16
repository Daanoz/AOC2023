use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        Answer::from(parse_input(input, false)).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        Answer::from(parse_input(input, true)).into()
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

fn parse_input(input: String, part_b: bool) -> usize {
    input
        .split("\n\n")
        .map(|s| parse_section(s, part_b))
        .sum::<usize>()
}

fn parse_section(input: &str, part_b: bool) -> usize {
    let base = read_as_number_list(input);
    if let Some(mirror) = find_mirror_slice(base, part_b) {
        return mirror * 100;
    }
    let flipped = read_as_number_list(&transpose_input(input));
    find_mirror_slice(flipped, part_b).expect("No mirror found")
}

/// Returns a list of numbers where each number represents a row of the input
fn read_as_number_list(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| {
            u32::from_str_radix(&l.to_string().replace('#', "1").replace('.', "0"), 2).unwrap()
        })
        .collect()
}

/// Transposes the input, so that the first row becomes the first column and so on
fn transpose_input(input: &str) -> String {
    let mut output = String::new();
    let map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for x in 0..map[0].len() {
        for row in &map {
            output.push(row[x]);
        }
        output.push('\n');
    }
    output
}

/// Returns the index of the mirror slice if found
/// For part b, the mirror slice must have exactly one bit difference
fn find_mirror_slice(values: Vec<u32>, part_b: bool) -> Option<usize> {
    'rows: for (index, _) in values.iter().enumerate().take(values.len() - 1) {
        let mirror = index + 1;
        let size = mirror.min(values.len() - index - 1);
        let before_slice = &values[mirror - size..mirror];
        let after_slice = &values[mirror..mirror + size];
        let mut bits_difference = 0;
        for (i, v) in before_slice.iter().enumerate() {
            if part_b {
                bits_difference +=
                    get_number_of_different_bits(*v, after_slice[(after_slice.len() - 1) - i]);
                if bits_difference > 1 {
                    continue 'rows;
                }
            } else if *v != after_slice[(after_slice.len() - 1) - i] {
                continue 'rows;
            }
        }
        if part_b && bits_difference != 1 {
            continue 'rows;
        }
        return Some(mirror);
    }
    None
}

fn get_number_of_different_bits(a: u32, b: u32) -> u32 {
    let mut count = 0;
    for i in 0..32 {
        if (a >> i) & 1 != (b >> i) & 1 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(405))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(400))
        )
    }
}

#[cfg(feature = "ui")]
fn build_shapes_for_ui(
    input: String,
    request: ui_support::DisplayRequest,
) -> ui_support::DisplayResult {
    use egui::{
        epaint::{CircleShape, Color32, Pos2, Rect, RectShape, Shape, Vec2},
        Stroke,
    };

    let section_index = request.result_index;

    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let section = sections[section_index % sections.len()];

    let mut shapes = section
        .lines()
        .enumerate()
        .into_iter()
        .flat_map(|(y, row)| {
            row.chars().enumerate().map(move |(x, c)| {
                if c == '#' {
                    Shape::Rect(RectShape::stroke(
                        Rect::from_min_size(
                            Pos2::new(x as f32 + 0.1, y as f32 + 0.1),
                            Vec2::new(0.8, 0.8),
                        ),
                        0.0,
                        Stroke::new(0.05, Color32::WHITE),
                    ))
                } else {
                    Shape::Circle(CircleShape::stroke(
                        Pos2::new(x as f32 + 0.5, y as f32 + 0.5),
                        0.1,
                        Stroke::new(0.05, Color32::WHITE),
                    ))
                }
            })
        })
        .collect::<Vec<Shape>>();

    let get_mirror_line = |section: &str, part_b: bool, color: Color32| {
        let mirror = parse_section(section, part_b);
        let points = if mirror < 100 {
            let start = Pos2::new(mirror as f32, 0.0);
            let end = Pos2::new(mirror as f32, section.lines().count() as f32);
            [start, end]
        } else {
            let start = Pos2::new(0.0, (mirror / 100) as f32);
            let end = Pos2::new(
                section.chars().position(|c| c == '\n').unwrap() as f32,
                (mirror / 100) as f32,
            );
            [start, end]
        };
        Shape::LineSegment {
            points,
            stroke: Stroke::new(0.1, color),
        }
    };

    shapes.push(get_mirror_line(section, false, Color32::GREEN));
    shapes.push(get_mirror_line(section, true, Color32::RED));

    ui_support::DisplayResult {
        result_count: Some(sections.len()),
        result_index: request.result_index,
        shapes: shapes.into_iter().map(|s| s.into()).collect(),
    }
}
