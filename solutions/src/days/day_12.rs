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
        Answer::from(parse_input(input)).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        Answer::from(parse_input_b(input)).into()
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

fn parse_input(input: String) -> usize {
    input
        .lines()
        .map(parse_line)
        .map(solve_line_part_a)
        .sum::<usize>()
}

fn parse_input_b(input: String) -> usize {
    input
        .lines()
        .map(parse_line)
        .map(solve_line_part_b)
        .sum::<usize>()
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let (springs, checksum) = line.split_once(' ').unwrap();
    let springs: Vec<char> = springs.chars().collect();
    let checksum: Vec<usize> = checksum.split(',').map(|s| s.parse().unwrap()).collect();
    (springs, checksum)
}

fn solve_line_part_a((springs, checksum): (Vec<char>, Vec<usize>)) -> usize {
    let mut memo_map: HashMap<(usize, usize), usize> = HashMap::new();
    find_possibilities(&springs, &checksum, 0, 0, &mut memo_map)
}

fn solve_line_part_b((springs, checksum): (Vec<char>, Vec<usize>)) -> usize {
    let mut memo_map: HashMap<(usize, usize), usize> = HashMap::new();
    // false-positive https://github.com/rust-lang/rust-clippy/issues/11958
    #[allow(clippy::useless_vec)]
    let springs = vec![springs; 5].join(&'?');
    let checksum = checksum.repeat(5);
    find_possibilities(&springs, &checksum, 0, 0, &mut memo_map)
}

fn find_possibilities(
    springs: &Vec<char>,
    checksum: &Vec<usize>,
    checksum_index: usize,
    spring_index: usize,
    memo_map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if memo_map.contains_key(&(checksum_index, spring_index)) {
        return memo_map[&(checksum_index, spring_index)];
    }

    if spring_index >= springs.len() {
        if checksum_index == checksum.len() {
            // end of the line, and all checksums are used
            return 1;
        }
        return 0;
    }

    if checksum_index >= checksum.len() {
        if springs[spring_index..].contains(&'#') {
            // There is still a spring left, but we have already used all the checksums
            return 0;
        }
        return 1;
    }

    let current_size = checksum[checksum_index];
    if spring_index + current_size > springs.len() {
        // we can't fit the upcoming block
        return 0;
    }

    // verify that we can fit the block, no '.'s in the block, no '#'s after the block
    let spring_can_fit_size = |springs: &Vec<char>, spring_index: usize, size: usize| -> bool {
        if spring_index + current_size < springs.len()
            && springs[spring_index + current_size] == '#'
        {
            return false;
        }
        if springs[spring_index..spring_index + size].contains(&'.') {
            return false;
        }
        true
    };

    let result = match springs[spring_index] {
        '.' => find_possibilities(
            springs,
            checksum,
            checksum_index,
            spring_index + 1,
            memo_map,
        ),
        '#' => {
            if !spring_can_fit_size(springs, spring_index, current_size) {
                return 0;
            }
            find_possibilities(
                springs,
                checksum,
                checksum_index + 1,
                spring_index + current_size + 1,
                memo_map,
            )
        }
        '?' => {
            let mut result = 0;
            if spring_can_fit_size(springs, spring_index, current_size) {
                // try to use '#'
                result += find_possibilities(
                    springs,
                    checksum,
                    checksum_index + 1,
                    spring_index + current_size + 1,
                    memo_map,
                );
            }
            // try to use '.'
            result
                + find_possibilities(
                    springs,
                    checksum,
                    checksum_index,
                    spring_index + 1,
                    memo_map,
                )
        }
        _ => panic!("invalid input"),
    };
    memo_map.insert((checksum_index, spring_index), result);
    result
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(21))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(525152))
        )
    }
}
