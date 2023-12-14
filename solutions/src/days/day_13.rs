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
    fn get_shapes(&mut self, _input: String) -> Option<Vec<ui_support::DisplayData>> {
        None
    }
}

fn parse_input(input: String, part_b: bool) -> usize {
    input.split("\n\n").map(|s| parse_section(s, part_b)).sum::<usize>()
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
        .map(|l|
            u32::from_str_radix(&l.to_string().replace("#", "1").replace(".", "0"), 2).unwrap()
        )
        .collect()
}

/// Transposes the input, so that the first row becomes the first column and so on
fn transpose_input(input: &str) -> String {
    let mut output = String::new();
    let map = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            output.push(map[y][x]);
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
                bits_difference += get_number_of_different_bits(*v, after_slice[(after_slice.len() - 1) - i]);
                if bits_difference > 1 {
                    continue 'rows;
                }
            } else {
                if *v != after_slice[(after_slice.len() - 1) - i] {
                    continue 'rows;
                }
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
