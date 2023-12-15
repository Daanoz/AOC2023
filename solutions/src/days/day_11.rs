use super::Solution;
use common::Answer;

pub struct Puzzle {
    part_b_grow_size: usize,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            part_b_grow_size: 1000000,
        }
    }
}

type Coord = (usize, usize);

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let coords = parse_input(input, 2);
        let pairs = create_pairs(coords);
        Answer::from(pairs.iter().map(get_distance).sum::<usize>()).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let coords = parse_input(input, self.part_b_grow_size);
        let pairs = create_pairs(coords);
        Answer::from(pairs.iter().map(get_distance).sum::<usize>()).into()
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

fn parse_input(input: String, grow_size: usize) -> Vec<Coord> {
    let grow_size = grow_size - 1;
    let row_length = input.find('\n').unwrap();
    let rows: Vec<usize> = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| if !line.contains('#') { Some(y) } else { None })
        .collect();
    let cols: Vec<usize> = (0..row_length)
        .filter(|x| {
            !input
                .lines()
                .map(|l| l.chars().nth(*x).unwrap())
                .collect::<String>()
                .contains('#')
        })
        .collect();
    let coords = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        let row_count = rows.iter().filter(|r| **r < y).count();
                        let col_count = cols.iter().filter(|r| **r < x).count();
                        Some((x + (col_count * grow_size), y + (row_count * grow_size)))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Coord>>()
        })
        .collect::<Vec<Coord>>();
    coords
}

fn create_pairs(coords: Vec<Coord>) -> Vec<(Coord, Coord)> {
    let mut pairs = vec![];
    for (i, coord) in coords.iter().enumerate() {
        for other in coords.iter().skip(i + 1) {
            pairs.push((*coord, *other));
        }
    }
    pairs
}

fn get_distance(pair: &(Coord, Coord)) -> usize {
    let ((x1, y1), (x2, y2)) = pair;
    let x = (*x1 as isize - *x2 as isize).unsigned_abs();
    let y = (*y1 as isize - *y2 as isize).unsigned_abs();
    x + y
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(374))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        puzzle.part_b_grow_size = 10;
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(1030))
        )
    }
}
