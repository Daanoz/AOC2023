#[cfg(feature = "performance")]
use ahash::AHashMap as HashMap;
#[cfg(not(feature = "performance"))]
use std::collections::HashMap;
use std::{ops::RangeInclusive, str::FromStr};

use super::Solution;
use common::Answer;

#[derive(Default)]
pub struct Puzzle;

type RangeList = HashMap<char, RangeInclusive<u32>>;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let (workflows, parts) = parse_input(&input);
        let start_flow = workflows.get("in").unwrap();
        let sum = parts
            .into_iter()
            .filter(|part| start_flow.run(part, &workflows) == RuleTarget::Accept)
            .map(|part| part.sum())
            .sum::<u32>();
        Answer::from(sum).into()
    }

    fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let (workflows, _) = parse_input(&input);
        let ranges: RangeList = HashMap::from([
            ('x', 1..=4000),
            ('m', 1..=4000),
            ('a', 1..=4000),
            ('s', 1..=4000),
        ]);
        let start_flow = workflows.get("in").unwrap();
        Answer::from(start_flow.reduce(&ranges, &workflows)).into()
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

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .map(|workflow: Workflow| (workflow.name.clone(), workflow))
        .collect();
    let parts: Vec<Part> = parts
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect();
    (workflows, parts)
}

struct Workflow {
    name: String,
    rules: Vec<RuleType>,
}

impl Workflow {
    pub fn run(&self, part: &Part, map: &HashMap<String, Workflow>) -> RuleTarget {
        let result = self
            .rules
            .iter()
            .find_map(|rule| rule.evaluate(part))
            .expect("End condition not found");
        if let RuleTarget::Workflow(name) = &result {
            let workflow = map.get(name).unwrap();
            workflow.run(part, map)
        } else {
            result
        }
    }

    pub fn reduce(&self, ranges: &RangeList, map: &HashMap<String, Workflow>) -> usize {
        let mut ranges = ranges.clone();
        self.rules
            .iter()
            .fold(
                vec![],
                |mut list: Vec<(RuleTarget, RangeList)>, rule| match rule.reduce(&ranges) {
                    None => list,
                    Some((target, next_ranges, remaining_ranges)) => {
                        ranges = remaining_ranges;
                        list.push((target, next_ranges));
                        list
                    }
                },
            )
            .into_iter()
            .map(|(target, ranges)| match target {
                RuleTarget::Workflow(name) => {
                    let workflow = map.get(&name).unwrap();
                    workflow.reduce(&ranges, map)
                }
                RuleTarget::Accept => ranges
                    .values()
                    .map(|range| (range.end() - range.start() + 1) as usize)
                    .product(),
                RuleTarget::Reject => 0,
            })
            .sum()
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules) = s.split_once('{').unwrap();
        let rules = rules.strip_suffix('}').unwrap();
        let rules = rules.split(',').map(|r| r.parse().unwrap()).collect();

        Ok(Workflow {
            name: String::from(name),
            rules,
        })
    }
}

enum RuleType {
    Direct(RuleTarget),
    Gt(char, u32, RuleTarget),
    Lt(char, u32, RuleTarget),
}

impl RuleType {
    pub fn evaluate(&self, part: &Part) -> Option<RuleTarget> {
        match self {
            Self::Direct(target) => Some(target.clone()),
            Self::Gt(src, value, target) => {
                if part.get(src) > *value {
                    Some(target.clone())
                } else {
                    None
                }
            }
            Self::Lt(src, value, target) => {
                if part.get(src) < *value {
                    Some(target.clone())
                } else {
                    None
                }
            }
        }
    }

    pub fn reduce(&self, ranges: &RangeList) -> Option<(RuleTarget, RangeList, RangeList)> {
        match self {
            Self::Direct(target) => Some((target.clone(), ranges.clone(), ranges.clone())),
            Self::Gt(src, value, target) => {
                let mut next_range = ranges.clone();
                let mut remaining_range = ranges.clone();
                let current_range = ranges.get(src).unwrap();
                if current_range.end() < value {
                    return None;
                }
                next_range.insert(
                    *src,
                    (value + 1).max(*current_range.start())..=*current_range.end(),
                );
                remaining_range.insert(
                    *src,
                    *current_range.start()..=*value.max(current_range.start()),
                );
                Some((target.clone(), next_range, remaining_range))
            }
            Self::Lt(src, value, target) => {
                let mut next_range = ranges.clone();
                let mut remaining_range = ranges.clone();
                let current_range = ranges.get(src).unwrap();
                if current_range.start() > value {
                    return None;
                }
                next_range.insert(
                    *src,
                    *current_range.start()..=(value - 1).min(*current_range.end()),
                );
                remaining_range
                    .insert(*src, *value.min(current_range.end())..=*current_range.end());
                Some((target.clone(), next_range, remaining_range))
            }
        }
    }
}

impl FromStr for RuleType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(':');
        if parts.is_none() {
            return Ok(Self::Direct(s.parse().unwrap()));
        }
        let (condition, target) = parts.unwrap();
        let mut chars = condition.chars();
        let src = chars.next().unwrap();
        let operator = chars.next().unwrap();
        let value: u32 = chars.as_str().parse().unwrap();
        match operator {
            '>' => Ok(Self::Gt(src, value, target.parse().unwrap())),
            '<' => Ok(Self::Lt(src, value, target.parse().unwrap())),
            _ => panic!("Unknown operator {}", operator),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum RuleTarget {
    Accept,
    Reject,
    Workflow(String),
}

impl FromStr for RuleTarget {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RuleTarget::Accept),
            "R" => Ok(RuleTarget::Reject),
            _ => Ok(RuleTarget::Workflow(String::from(s))),
        }
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    pub fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
    pub fn get(&self, c: &char) -> u32 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Unknown part {}", c),
        }
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        for part in str[1..str.len() - 1].split(',') {
            let (key, value) = part.split_once('=').unwrap();
            let value: u32 = value.parse().unwrap();
            match key {
                "x" => x = value,
                "m" => m = value,
                "a" => a = value,
                "s" => s = value,
                _ => panic!("Unknown part {}", key),
            }
        }
        Ok(Part { x, m, a, s })
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)),
            Ok(Answer::from(19114))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)),
            Ok(Answer::from(167409079868000_isize))
        )
    }
}
