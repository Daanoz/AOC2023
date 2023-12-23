#[cfg(feature = "performance")]
use ahash::AHashMap as HashMap;
#[cfg(not(feature = "performance"))]
use std::collections::HashMap;

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        Answer::from(
            input
                .trim()
                .split(',')
                .map(holiday_ascii_string_helper)
                .sum::<usize>(),
        )
        .into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        Answer::from(holiday_ascii_string_helper_manual_arrangement_procedure(
            &input,
        ))
        .into()
    }

    #[cfg(feature = "ui")]
    fn get_shapes(
        &mut self,
        _input: String,
        _request: ui_support::DisplayRequest,
    ) -> Option<ui_support::DisplayResult> {
        None
    }
}

// Holiday ASCII String Helper Manual Arrangement Procedure
fn holiday_ascii_string_helper_manual_arrangement_procedure(input: &str) -> usize {
    let mut map: HashMap<usize, LensBox> = HashMap::new();
    input.trim().split(',').for_each(|step| {
        let (label, focal_length) = step.split_once(['=', '-']).unwrap();
        let operation = step.chars().nth(label.len()).unwrap();
        let box_number = holiday_ascii_string_helper(label);
        let lens_box = map.entry(box_number).or_default();
        match operation {
            '=' => lens_box.add_lens(Lens::new(focal_length.into(), label.into())),
            '-' => lens_box.remove_lens(label),
            _ => panic!("Invalid operation: {}", operation),
        }
    });
    map.into_iter()
        .map(|(box_number, lens_box)| lens_box.focus_power(box_number))
        .sum()
}

// Holiday ASCII String Helper algorithm (appendix 1A)
fn holiday_ascii_string_helper(input: &str) -> usize {
    input.chars().fold(0, |mut acc, c| {
        acc += (c as u8) as usize;
        acc *= 17;
        acc % 256
    })
}

#[derive(Default)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn focus_power(&self, box_number: usize) -> usize {
        self.lenses.iter().enumerate().fold(0, |acc, (pos, lens)| {
            acc + ((box_number + 1) * (pos + 1) * lens.focal_length)
        })
    }
    fn add_lens(&mut self, lens: Lens) {
        let pos = self.lenses.iter().position(|l| l.label == lens.label);
        if let Some(pos) = pos {
            self.lenses[pos] = lens;
        } else {
            self.lenses.push(lens);
        }
    }
    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|lens| lens.label != label);
    }
}

struct Lens {
    focal_length: usize,
    label: String,
}

impl Lens {
    fn new(focal_length: String, label: String) -> Self {
        Self {
            focal_length: focal_length.parse().unwrap(),
            label,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(1320))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(145))
        )
    }
}
