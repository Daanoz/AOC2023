use std::{str::FromStr, collections::HashMap};

use async_trait::async_trait;
use common::Answer;
use super::Solution;

#[derive(Default)]
pub struct Puzzle;

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, _input: String) -> Result<Answer, String> {
        let mut hands = parse_input_a(input);
        hands.sort();
        let total_winnings = hands.into_iter().enumerate().fold(0 as usize, |acc, (index, hand)| {
            acc + (hand.bid * (index + 1))
        });
        Answer::from(total_winnings).into()
    }

    async fn solve_b(&mut self, _input: String) -> Result<Answer, String> {
        let mut hands = parse_input_b(input);
        hands.sort();
        let total_winnings = hands.into_iter().enumerate().fold(0 as usize, |acc, (index, hand)| {
            acc + (hand.bid * (index + 1))
        });
        Answer::from(total_winnings).into()
    }

    #[cfg(feature = "ui")]
    async fn get_shapes(&mut self, _input: String, _rect: egui::Rect) -> Option<Vec<ui_support::Shape>> {
        None
    }
}

fn parse_input_a(input: String) -> Vec<Hand<PartA>> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Hand<PartA>>().unwrap())
        .collect::<Vec<Hand<PartA>>>()
}
fn parse_input_b(input: String) -> Vec<Hand<PartB>> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Hand<PartB>>().unwrap())
        .collect::<Vec<Hand<PartB>>>()
}

trait Part {}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PartA;
impl Part for PartA {}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PartB;
impl Part for PartB {}

#[derive(Debug, PartialEq, Eq)]
struct Hand<T> where T: Part {
    cards: Vec<u32>,
    hand_type: HandType,
    bid: usize,
    part: std::marker::PhantomData<T>,
}

impl<T> PartialOrd for Hand<T> where T: Part + std::cmp::PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) | None => {}
            ord => return ord,
        }
        match self.cards.partial_cmp(&other.cards) {
            Some(core::cmp::Ordering::Equal) | None => {}
            ord => return ord,
        }
        self.bid.partial_cmp(&other.bid)
    }
}

impl<T> Ord for Hand<T> where T: Part + Ord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Hand<PartA> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let bid = bid.parse::<usize>().unwrap();
        let cards = cards.chars().map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap(),
        }).collect::<Vec<u32>>();
        let card_count = cards.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });
        let hand_type = match card_count.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if card_count.len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            },
            2 => {
                if card_count.len() == 3 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            },
            _ => HandType::HighCard,
        };
        Ok(Hand { cards, bid, hand_type, part: std::marker::PhantomData })
    }
}

impl FromStr for Hand<PartB> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let bid = bid.parse::<usize>().unwrap();
        let cards = cards.chars().map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 0,
            'T' => 10,
            _ => c.to_digit(10).unwrap(),
        }).collect::<Vec<u32>>();
        let mut card_count = cards.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });
        let joker_count = *card_count.get(&0).unwrap_or(&0);
        card_count.remove(&0);
        let hand_type = match card_count.values().max().unwrap_or(&0) {
            5 => HandType::FiveOfAKind,
            4 => {
                if joker_count > 0 {
                    HandType::FiveOfAKind
                } else {
                    HandType::FourOfAKind
                }
            },
            3 => {
                match joker_count {
                    2 => HandType::FiveOfAKind,
                    1 => HandType::FourOfAKind,
                    _ => if card_count.len() == 2 {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfAKind
                    }
                }
            },
            2 => { 
                match joker_count {
                    3 => HandType::FiveOfAKind,
                    2 => HandType::FourOfAKind,
                    1 => {
                        if card_count.len() == 2 {
                            HandType::FullHouse
                        } else {
                            HandType::ThreeOfAKind
                        }
                    },
                    _ => if card_count.len() == 3 {
                        HandType::TwoPair
                    } else {
                        HandType::Pair
                    }
                }
            },
            1 => {
                match joker_count {
                    4 => HandType::FiveOfAKind,
                    3 => HandType::FourOfAKind,
                    2 => HandType::ThreeOfAKind,
                    1 => HandType::Pair,
                    _ => HandType::HighCard
                }
            },
            0 => HandType::FiveOfAKind,
            _ => panic!("Unexpected count")
        };
        Ok(Hand { cards, bid, hand_type, part: std::marker::PhantomData })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum HandType {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::Answer;
    use super::Solution;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(6440))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(5905))
        )
    }
}