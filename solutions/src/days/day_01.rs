use async_trait::async_trait;
use common::Answer;
use super::Solution;

#[derive(Default)]
pub struct Puzzle;

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        Answer::from(read_numbers(input)).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        Answer::from(read_str_numbers(input)).into()
    }

    #[cfg(feature = "ui")]
    async fn get_shapes(&mut self, _input: String, _rect: egui::Rect) -> Option<Vec<ui_support::Shape>> {
        None
    }
}

fn read_numbers(input: String) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

fn read_str_numbers(input: String) -> u32 {
    let start_regex = regex::Regex::new(r"^.*?(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let end_regex = regex::Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|[0-9]).*?$").unwrap();

    input
        .trim()
        .lines()
        .map(|line| {
            let start = start_regex.captures(line).unwrap().get(1).unwrap().as_str();
            let end = end_regex.captures(line).unwrap().get(1).unwrap().as_str();
            str_as_digit(start) * 10 + str_as_digit(end)
        })
        .sum()
}

fn str_as_digit(input: &str) -> u32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => input.parse::<u32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::Answer;
use super::Solution;

    const TEST_INPUT_A: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const TEST_INPUT_B: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT_A)).await,
            Ok(Answer::from(142))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT_B)).await,
            Ok(Answer::from(281))
        )
    }
}