#[cfg(feature = "performance")]
use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::{Debug, Display},
    ops::RangeInclusive,
    rc::Rc,
    str::FromStr,
};
#[cfg(not(feature = "performance"))]
use std::{collections::HashMap, collections::HashSet};

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

type BlockRef = Rc<RefCell<Brick>>;
type BrickList = Vec<BlockRef>;
type ZMap = HashMap<usize, Vec<BlockRef>>;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let mut bricks = parse_input(input);
        let mut z_map = create_z_map(&bricks);
        stabilize_and_set_supports(&mut bricks, &mut z_map);
        let result = bricks
            .iter()
            .filter(|b| {
                if b.borrow().supporting.is_empty() {
                    return true;
                }
                return b
                    .borrow()
                    .supporting
                    .iter()
                    .all(|s| s.borrow().supported_by.len() > 1);
            })
            .count();
        Answer::from(result).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let mut bricks = parse_input(input);
        let mut z_map = create_z_map(&bricks);
        stabilize_and_set_supports(&mut bricks, &mut z_map);
        let result: usize = bricks
            .into_iter()
            .map(|b| {
                if b.borrow().supporting.is_empty() {
                    return 0;
                }
                count_drops(&b)
            })
            .sum();
        Answer::from(result).into()
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

fn parse_input(input: String) -> BrickList {
    input
        .lines()
        .enumerate()
        .map(|(c, line)| {
            let (left, right) = line.split_once('~').unwrap();
            Rc::new(RefCell::new(
                (c, left.parse().unwrap(), right.parse().unwrap()).into(),
            ))
        })
        .collect()
}

fn create_z_map(bricks: &BrickList) -> ZMap {
    bricks.iter().fold(HashMap::new(), |mut map: ZMap, brick| {
        let z_range = &brick.borrow().z_range;
        for i in z_range.clone() {
            map.entry(i).or_default().push(brick.clone());
        }
        map
    })
}

fn stabilize_and_set_supports(bricks: &mut BrickList, z_map: &mut ZMap) {
    while bricks.iter().any(|b| !b.borrow().is_static) {
        drop_bricks(bricks, z_map);
    }
    // compress z_map
    z_map.retain(|_, l| !l.is_empty());
    set_supports(bricks, z_map);
    set_supporting(bricks, z_map);
}

fn drop_bricks(bricks: &mut BrickList, z_map: &mut ZMap) {
    let mut dropped = Vec::new();
    let mut became_static = Vec::new();
    for brick_ref in bricks.iter_mut() {
        let brick = brick_ref.borrow();
        if brick.is_static {
            continue;
        }
        let lower_level = brick.z_range.start() - 1;
        let lower_blocks = z_map.get(&lower_level);
        let mut will_be_static: bool = false;
        let is_blocked = if let Some(lower_blocks) = lower_blocks {
            let touched_blocks = lower_blocks.iter().filter(|lb| {
                range_intersect(&lb.borrow().x_range, &brick.x_range)
                    && range_intersect(&lb.borrow().y_range, &brick.y_range)
            });
            will_be_static = touched_blocks.clone().any(|b| b.borrow().is_static);
            touched_blocks.count() > 0
        } else {
            false
        };
        if will_be_static {
            became_static.push(brick_ref.clone());
        } else if !is_blocked {
            dropped.push(brick_ref.clone());
            if lower_level == 1 {
                became_static.push(brick_ref.clone());
            }
        }
    }
    for brick_ref in dropped {
        let (from, to) = {
            let mut brick = brick_ref.borrow_mut();
            let z_range_before = brick.z_range.clone();
            brick.drop_brick();
            (*z_range_before.end(), *brick.z_range.start())
        };
        z_map
            .get_mut(&from)
            .unwrap()
            .retain(|b| !Rc::ptr_eq(b, &brick_ref));
        z_map.entry(to).or_default().push(brick_ref);
    }
    for brick_ref in became_static {
        brick_ref.borrow_mut().make_static();
    }
}

fn set_supports(bricks: &mut BrickList, z_map: &mut ZMap) {
    for brick_ref in bricks.iter_mut() {
        let lower_blocks = {
            let brick = brick_ref.borrow();
            let lower_level = brick.z_range.start() - 1;
            z_map
                .get(&lower_level)
                .map(|lower_blocks| {
                    lower_blocks
                        .iter()
                        .filter(|lb| {
                            range_intersect(&lb.borrow().x_range, &brick.x_range)
                                && range_intersect(&lb.borrow().y_range, &brick.y_range)
                        })
                        .cloned()
                        .collect::<Vec<BlockRef>>()
                })
                .unwrap_or_default()
        };
        brick_ref.borrow_mut().set_supports(lower_blocks);
    }
}
fn set_supporting(bricks: &mut BrickList, z_map: &mut ZMap) {
    for brick_ref in bricks.iter_mut() {
        let upper_blocks = {
            let brick = brick_ref.borrow();
            let upper_level = brick.z_range.end() + 1;
            z_map
                .get(&upper_level)
                .map(|lower_blocks| {
                    lower_blocks
                        .iter()
                        .filter(|lb| {
                            range_intersect(&lb.borrow().x_range, &brick.x_range)
                                && range_intersect(&lb.borrow().y_range, &brick.y_range)
                        })
                        .cloned()
                        .collect::<Vec<BlockRef>>()
                })
                .unwrap_or_default()
        };
        brick_ref.borrow_mut().set_supporting(upper_blocks);
    }
}

fn count_drops(brick: &BlockRef) -> usize {
    let mut queue = VecDeque::from([brick.clone()]);
    let mut falling: HashSet<_> = HashSet::from([brick.as_ptr() as usize]);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for top in &current.borrow().supporting {
            let is_falling = top
                .borrow()
                .supported_by
                .iter()
                .all(|s| falling.contains(&(s.as_ptr() as usize)));
            if is_falling {
                falling.insert(top.as_ptr() as usize);
                queue.push_back(top.clone());
            }
        }
    }
    falling.len() - 1
}

fn range_intersect(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    if a.end() < b.start() || a.start() > b.end() {
        false
    } else {
        !matches!(
            (
                a.start().partial_cmp(b.start()),
                a.end().partial_cmp(b.end()),
            ),
            (None, _) | (_, None)
        )
    }
}

struct Brick {
    id: usize,
    c1: Coord,
    c2: Coord,
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
    z_range: RangeInclusive<usize>,
    is_static: bool,
    supported_by: Vec<BlockRef>,
    supporting: Vec<BlockRef>,
}

impl From<(usize, Coord, Coord)> for Brick {
    fn from((id, c1, c2): (usize, Coord, Coord)) -> Self {
        Self {
            id,
            is_static: c1.z == 1 || c2.z == 1,
            x_range: c1.x.min(c2.x)..=c1.x.max(c2.x),
            y_range: c1.y.min(c2.y)..=c1.y.max(c2.y),
            z_range: c1.z.min(c2.z)..=c1.z.max(c2.z),
            c1,
            c2,
            supported_by: vec![],
            supporting: vec![],
        }
    }
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}~{}",
            char::from_u32(65 + self.id as u32).unwrap(),
            self.c1,
            self.c2
        )
    }
}

impl Brick {
    fn drop_brick(&mut self) {
        self.c1.z -= 1;
        self.c2.z -= 1;
        self.z_range = self.c1.z.min(self.c2.z)..=self.c1.z.max(self.c2.z);
    }
    fn make_static(&mut self) {
        self.is_static = true;
    }
    fn set_supports(&mut self, supports: Vec<BlockRef>) {
        self.supported_by = supports;
    }
    fn set_supporting(&mut self, supports: Vec<BlockRef>) {
        self.supporting = supports;
    }
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Coord {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(',').collect::<Vec<_>>();
        Ok(Self {
            x: s[0].parse()?,
            y: s[1].parse()?,
            z: s[2].parse()?,
        })
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(5))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(7))
        )
    }
}
