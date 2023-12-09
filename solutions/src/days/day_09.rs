use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let history = parse_input(input);
        let total = history
            .into_iter()
            .map(|v| v.last().unwrap() + extrapolate(&v).1)
            .sum::<isize>();
        Answer::from(total).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let history = parse_input(input);
        let total = history
            .into_iter()
            .map(|v: Vec<isize>| v.first().unwrap() - extrapolate(&v).0)
            .sum::<isize>();
        Answer::from(total).into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(&mut self, _input: String, _rect: egui::Rect) -> Option<Vec<ui_support::Shape>> {
        None
    }
}

fn parse_input(input: String) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|d| d.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect()
}

fn extrapolate(history: &[isize]) -> (isize, isize) {
    if history.iter().all(|n| n == &0_isize) {
        return (0, 0);
    }
    let deltas: Vec<isize> = history.windows(2).map(|w| w[1] - w[0]).collect();
    let extrapolated = extrapolate(&deltas);
    (
        deltas.first().unwrap() - extrapolated.0,
        deltas.last().unwrap() + extrapolated.1
    )
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(114))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(2))
        )
    }
}
