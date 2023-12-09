use std::str::FromStr;

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let cards = input.lines().map(|l| l.parse::<Card>().unwrap());
        Answer::from(cards.map(|c| c.score()).sum::<u32>()).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let cards = input
            .lines()
            .map(|l| l.parse::<Card>().unwrap())
            .collect::<Vec<_>>();
        let mut counts = cards.iter().map(|_| 1_u32).collect::<Vec<_>>();
        for (i, card) in cards.iter().enumerate() {
            let wins = card.winning_numbers();
            for i2 in (i + 1)..=i + (wins as usize) {
                counts[i2] += counts[i];
            }
        }
        Answer::from(counts.into_iter().sum::<u32>()).into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(&mut self, _input: String) -> Option<Vec<ui_support::DisplayData>> {
        None
    }
}

struct Card(Vec<u32>, Vec<u32>);

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once(": ").unwrap().1;
        let (winning, draw) = s.split_once(" | ").unwrap();
        let winning = winning
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let draw = draw
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Self(winning, draw))
    }
}

impl Card {
    fn winning_numbers(&self) -> u32 {
        self.1.iter().filter(|n| self.0.contains(n)).count() as u32
    }
    fn score(&self) -> u32 {
        let count = self.winning_numbers();
        if count == 0 {
            return 0;
        }
        u32::pow(2, count - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(13))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(30))
        )
    }
}
