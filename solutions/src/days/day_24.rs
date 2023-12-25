use std::collections::HashSet;

use super::Solution;
use common::Answer;

pub struct Puzzle {
    test_area: core::ops::Range<f64>,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            test_area: 200000000000000.0..400000000000000.0,
        }
    }
}

type Vec2<T> = (T, T);
type Vec3<T> = (T, T, T);
type Hailstone<T> = (Vec3<T>, Vec3<T>);

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let hail = parse_input::<f64>(&input);
        let mut count = 0;
        for (index, hail_a) in hail.iter().enumerate() {
            for hail_b in hail.iter().skip(index + 1) {
                if let Some(intersection) = intersection2d(hail_a, hail_b) {
                    if self.test_area.contains(&intersection.0)
                        && self.test_area.contains(&intersection.1)
                    {
                        count += 1;
                    }
                }
            }
        }
        Answer::from(count).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let hail = parse_input::<isize>(&input);
        let mut x_set: HashSet<isize> = HashSet::new();
        let mut y_set: HashSet<isize> = HashSet::new();
        let mut z_set: HashSet<isize> = HashSet::new();
        let get_set = |d1: isize, d2: isize, p1: isize, p2: isize| -> HashSet<isize> {
            let mut next_set = HashSet::new();
            if d1 == d2 && d1.abs() > 100 {
                let dif = p2 - p1;
                for v in -1000..1000 {
                    if v == d1 {
                        continue;
                    }
                    if dif % (v - d1) == 0 {
                        next_set.insert(v);
                    }
                }
            }
            next_set
        };

        for (index, a) in hail.iter().enumerate() {
            for b in hail.iter().skip(index + 1) {
                let ((x1, y1, z1), (dx1, dy1, dz1)) = a;
                let ((x2, y2, z2), (dx2, dy2, dz2)) = b;
                let next_x_set = get_set(*dx1, *dx2, *x1, *x2);
                let next_y_set = get_set(*dy1, *dy2, *y1, *y2);
                let next_z_set = get_set(*dz1, *dz2, *z1, *z2);

                if x_set.is_empty() {
                    x_set = next_x_set;
                } else if !next_x_set.is_empty() {
                    x_set = x_set.intersection(&next_x_set).cloned().collect();
                }
                if y_set.is_empty() {
                    y_set = next_y_set;
                } else if !next_y_set.is_empty() {
                    y_set = y_set.intersection(&next_y_set).cloned().collect();
                }
                if z_set.is_empty() {
                    z_set = next_z_set;
                } else if !next_z_set.is_empty() {
                    z_set = z_set.intersection(&next_z_set).cloned().collect();
                }
            }
        }
        assert!(x_set.len() == 1);
        assert!(y_set.len() == 1);
        assert!(z_set.len() == 1);

        let (dxr, dyr, dzr) = (
            x_set.iter().nth(0).unwrap(),
            y_set.iter().nth(0).unwrap(),
            z_set.iter().nth(0).unwrap(),
        );
        let ((x1, y1, z1), (dx1, dy1, dz1)) = hail[0];
        let ((x2, y2, _z2), (dx2, dy2, _dz2)) = hail[1];

        let ma = (dy1 - dyr) as f64 / (dx1 - dxr) as f64;
        let mb = (dy2 - dyr) as f64 / (dx2 - dxr) as f64;
        let ca = y1 as f64 - (ma * x1 as f64);
        let cb = y2 as f64 - (mb * x2 as f64);
        let x = ((cb - ca) / (ma - mb)).floor() as isize;
        let y = (ma * x as f64 + ca).floor() as isize;
        let t = ((x - x1) as f64 / (dx1 - dxr) as f64).floor() as isize;
        let z = z1 + (dz1 - dzr) * t;

        Answer::from(x + y + z).into()
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

fn intersection2d(a: &Hailstone<f64>, b: &Hailstone<f64>) -> Option<Vec2<f64>> {
    let ((x1, y1, _), (dx1, dy1, _)) = a;
    let ((x2, y2, _), (dx2, dy2, _)) = b;

    // Check if the lines are parallel (no intersection)
    let det = dx1 * dy2 - dy1 * dx2;
    if det.abs() < 1e-10 {
        return None;
    }

    // Calculate intersection coordinates
    let t = ((x2 - x1) * dy2 - (y2 - y1) * dx2) / det;
    let x = x1 + t * dx1;
    let y = y1 + t * dy1;

    let intersection = (x, y);

    // Check if the intersection point is in the same direction as both lines
    let dot_product = |v1: Vec2<_>, v2: Vec2<_>| -> f64 { v1.0 * v2.0 + v1.1 * v2.1 };
    let dot_product1 = dot_product((*dx1, *dy1), (intersection.0 - x1, intersection.1 - y1));
    let dot_product2 = dot_product((*dx2, *dy2), (intersection.0 - x2, intersection.1 - y2));

    if dot_product1 > 0.0 && dot_product2 > 0.0 {
        return Some(intersection);
    }
    None
}

fn parse_input<T>(input: &str) -> Vec<Hailstone<T>>
where
    T: std::str::FromStr + std::fmt::Debug + Copy,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| {
            let (point, direction) = line.split_once(" @ ").unwrap();
            let point: Vec<T> = point
                .split(", ")
                .map(|n| {
                    n.trim()
                        .parse::<T>()
                        .unwrap_or_else(|_| panic!("Failed to parse {}", n))
                })
                .collect();
            let direction: Vec<T> = direction
                .split(", ")
                .map(|n| {
                    n.trim()
                        .parse::<T>()
                        .unwrap_or_else(|_| panic!("Failed to parse {}", n))
                })
                .collect();
            (
                (point[0], point[1], point[2]),
                (direction[0], direction[1], direction[2]),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        puzzle.test_area = 7.0..27.0;
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(2))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(47))
        )
    }
}
