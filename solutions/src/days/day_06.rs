use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let input = read_input(input);
        Answer::from(
            input
                .into_iter()
                .map(find_winning_strategies)
                .product::<usize>(),
        )
        .into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let input = read_input_with_fixed_kerning(input);
        Answer::from(
            input
                .into_iter()
                .map(find_winning_strategies)
                .product::<usize>(),
        )
        .into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(&mut self, _input: String) -> Option<Vec<ui_support::DisplayData>> {
        None
    }
}

fn read_input(input: String) -> Vec<(usize, usize)> {
    let (times, distances) = input.trim().split_once('\n').unwrap();
    let times: Vec<usize> = times
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = distances
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    times
        .into_iter()
        .zip(distances)
        .collect::<Vec<_>>()
}

fn read_input_with_fixed_kerning(input: String) -> Vec<(usize, usize)> {
    let (times, distances) = input.trim().split_once('\n').unwrap();
    let time = times
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    let distance = distances
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    vec![(time, distance)]
}

fn find_winning_strategies(game: (usize, usize)) -> usize {
    (0..game.0)
        .filter(|hold_time| {
            let speed = hold_time;
            let remaining_time = game.0 - hold_time;
            speed * remaining_time > game.1
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(288))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(71503))
        )
    }
}
