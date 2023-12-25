use std::collections::{HashMap, VecDeque, HashSet};


use super::Solution;
use common::Answer;
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct Puzzle;

impl Solution for Puzzle {
    fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let (graph, nodes, edges) = parse_input(&input);

        // let mut final_groups = Vec::new();
        // 'lop: loop {
        //     let mut groups = nodes.iter().map(|n| HashSet::from([n])).collect::<Vec<_>>();
        //     while groups.len() > 2 {
        //         let edge = edges[edges.len() - 1];
        //         let group_a = groups.iter().position(|g| g.contains(&edge.0)).unwrap();
        //         let group_b = groups.iter().position(|g| g.contains(&edge.1)).unwrap();
        //         if group_a != group_b {
        //             let group_b = groups.remove(group_b);
        //             groups.get_mut(group_a).unwrap().extend(group_b);
        //         }
        //         if edges.iter().filter(|(a, b)| {
        //             let group_a = groups.iter().position(|g| g.contains(&a)).unwrap();
        //             let group_b = groups.iter().position(|g| g.contains(&b)).unwrap();
        //             group_a != group_b
        //         }).count() < 4 {
        //             final_groups = groups;
        //             break 'lop;
        //         }
        //     }
        // }
        // println!("{:?}", final_groups.iter().map(|f| f.len()).collect::<Vec<_>>());
        // 614655
        let center = graph.len() / 2;
        let offset = graph.len() / 100;
        let acceptable = center-offset..center+offset;
        let mut path_counts: HashMap<String, i32> = HashMap::new();
        let mut groups = (0, 0);
        while !acceptable.contains(&groups.0) {
            groups = find_groups(&graph, &mut path_counts);
            println!("g1: {}, g2: {}", groups.0, groups.1);
        }
        println!("g1: {}, g2: {}", groups.0, groups.1);
        Answer::from(groups.0 * groups.1).into()
    }

    fn solve_b(&mut self, _input: String) -> Result<Answer, String> {
        Answer::from("Merry Christmas").into()
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

fn find_groups(graph: &HashMap<&str, Vec<&str>>, path_counts: &mut HashMap<String, i32>) -> (usize, usize) {
    let nodes: Vec<&str> = graph.keys().map(|&x| x).collect();

    let mut rand = rand::thread_rng();

    for _ in 0..30 {
        let mut chosen = nodes.choose_multiple(&mut rand, 2);
        let a = *chosen.next().unwrap();
        let b = *chosen.next().unwrap();

        get_path(&graph, a, b).iter().for_each(|&n| {
            let count = path_counts.get(n).unwrap_or(&0);
            path_counts.insert(n.to_string(), count + 1);
        })
    }

    let mut count_vecs: Vec<_> = path_counts.iter().collect();
    count_vecs.sort_by(|a, b| b.1.cmp(a.1));

    let suspicious: Vec<_> = count_vecs.iter().take(6).map(|x| x.0.clone()).collect();

    let a = suspicious[0].as_str();
    let b = suspicious[1].as_str();
    let c = suspicious[2].as_str();
    let d = suspicious[3].as_str();
    let e = suspicious[4].as_str();
    let f = suspicious[5].as_str();

    let black_list = [
        (a, b),
        (c, d), 
        (e, f)
    ];
    (count_nodes(&graph, a, &black_list), count_nodes(&graph, b, &black_list))
}

fn get_path<'a>(network: &HashMap<&'a str, Vec<&'a str>>, start: &'a str, end: &'a str) -> Vec<&'a str> {
    let mut queue = VecDeque::from([(start, vec![start])]);
    while let Some((node, visited)) = queue.pop_front() {
        if node == end {
            return visited;
        }
        if let Some(neighbors) = network.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    let mut next_visited = visited.clone();
                    next_visited.push(neighbor);
                    queue.push_back((neighbor, next_visited));
                }
            }
        }
    }
    panic!("No path found");
}

// fn path_from(&self, a: &'a str, b: &'a str) -> Vec<&str> {
//     let mut q = VecDeque::new();

//     q.push_back((a, vec![a]));

//     while let Some((curr, seen)) = q.pop_front() {
//         if curr == b {
//             return seen
//         }

//         let next = self.connections.get(curr).unwrap();

//         next.iter().filter(|&x| !seen.contains(x)).for_each(|&n| {
//             let mut nq = seen.clone();
//             nq.push(n);
//             q.push_back((n, nq))
//         })
//     }

//     todo!()

// }

fn count_nodes(network: &HashMap<&str, Vec<&str>>, node: &str, black_list: &[(&str, &str)]) -> usize {
    let mut visited = Vec::new();
    let mut queue = VecDeque::from([node]);
    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }
        visited.push(node);
        if let Some(neighbors) = network.get(node) {
            for neighbor in neighbors {
                if !black_list.contains(&(neighbor, node))&& !black_list.contains(&(node, neighbor)) {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    visited.len()
}

fn parse_input<'a>(input: &'a str) -> (HashMap<&'a str, Vec<&'a str>>, HashSet<&'a str>, Vec<(&'a str, &'a str)>) {
    let mut graph= HashMap::new();
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();
    input.lines().for_each(|line| {
        let (key, values) = line.split_once(": ").unwrap();
        nodes.insert(key);
        nodes.extend(values.split(' '));
        edges.extend(values.split(' ').map(|v| (key, v)));
        graph.entry(key).or_insert_with(Vec::new).extend(values.split(' ').map(|v| v));
        values.split(' ').for_each(|v| {
            graph.entry(v).or_insert_with(Vec::new).push(key);
        });
    });
    (graph, nodes, edges.into_iter().collect())
}   

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use super::Solution;
    use common::Answer;

    const TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

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
