use common::Answer;
use super::Solution;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, _input: String) -> Result<Answer, String> {
        Answer::from("").into()
    }

    fn solve_b(&mut self, _input: String) -> Result<Answer, String> {
        Answer::from("").into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(&mut self, _input: String) -> Option<Vec<ui_support::Shape>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::Answer;
    use super::Solution;

    const TEST_INPUT: &str = "";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(""))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(""))
        )
    }
}