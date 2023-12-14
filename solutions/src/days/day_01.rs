use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        Answer::from(read_numbers(input)).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        Answer::from(read_str_numbers(input)).into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(&mut self, input: String) -> Option<Vec<ui_support::DisplayData>> {
        Some(build_shapes_for_ui(input))
    }
}

fn read_numbers(input: String) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

fn get_regex_pair() -> (regex::Regex, regex::Regex) {
    let start_regex =
        regex::Regex::new(r"^.*?(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let end_regex =
        regex::Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|[0-9]).*?$").unwrap();
    (start_regex, end_regex)
}

fn read_str_numbers(input: String) -> u32 {
    let (start_regex, end_regex) = get_regex_pair();
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
        _ => input.parse::<u32>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

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
            puzzle.solve_a(String::from(TEST_INPUT_A)),
            Ok(Answer::from(142))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT_B)),
            Ok(Answer::from(281))
        )
    }
}

#[cfg(feature = "ui")]
fn build_shapes_for_ui(input: String) -> Vec<ui_support::DisplayData> {
    let (start_regex, end_regex) = get_regex_pair();
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            let start = start_regex.captures(line).unwrap().get(1).unwrap();
            let end = end_regex.captures(line).unwrap().get(1).unwrap();
            let out = str_as_digit(start.as_str()) * 10 + str_as_digit(end.as_str());
            let start_range = start.range();
            let mut end_range = end.range();
            if start_range.end > end_range.start {
                end_range.start = start_range.end;
            }
            vec![
                ui_support::DisplayData::text(
                    egui::Pos2::new(0.0, 1.0 * (i as f32)),
                    line[0..start_range.start].to_string(),
                    1.0,
                    egui::Color32::WHITE,
                ),
                ui_support::DisplayData::text(
                    egui::Pos2::new(start_range.start as f32, 1.0 * (i as f32)),
                    line[start_range.clone()].to_string(),
                    1.0,
                    egui::Color32::RED,
                ),
                ui_support::DisplayData::text(
                    egui::Pos2::new(start_range.end as f32, 1.0 * (i as f32)),
                    line[start_range.end..end_range.start].to_string(),
                    1.0,
                    egui::Color32::WHITE,
                ),
                ui_support::DisplayData::text(
                    egui::Pos2::new(end_range.start as f32, 1.0 * (i as f32)),
                    line[end_range.clone()].to_string(),
                    1.0,
                    egui::Color32::DARK_RED,
                ),
                ui_support::DisplayData::text(
                    egui::Pos2::new(end_range.end as f32, 1.0 * (i as f32)),
                    line[end_range.end..].to_string(),
                    1.0,
                    egui::Color32::WHITE,
                ),
                ui_support::DisplayData::text(
                    egui::Pos2::new(line.len() as f32, 1.0 * (i as f32)),
                    format!(" = {}", out),
                    1.0,
                    egui::Color32::BLUE,
                ),
            ]
        })
        .collect::<Vec<ui_support::DisplayData>>()
}
