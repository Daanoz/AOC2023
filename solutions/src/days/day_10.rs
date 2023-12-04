use async_trait::async_trait;
use common::Answer;
use super::Solution;

#[derive(Default)]
pub struct Puzzle;

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, _input: String) -> Result<Answer, String> {
        Answer::from("").into()
    }

    async fn solve_b(&mut self, _input: String) -> Result<Answer, String> {
        Answer::from("").into()
    }

    #[cfg(feature = "ui")]
    async fn get_shapes(&mut self, _input: String, _rect: egui::Rect) -> Option<Vec<ui_support::Shape>> {
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
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(""))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(""))
        )
    }
}