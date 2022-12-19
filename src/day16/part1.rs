use std::collections::{BinaryHeap, HashMap};

use ndarray::Array2;

use crate::day16::{Input, Output};

#[derive(PartialEq, Eq, Default, Debug)]
struct State {
    node: usize,
    time: usize,
    total: usize,
    rate: usize,
    target: usize,
    remaining: usize,
    seen: Vec<usize>,
}

impl State {
    fn total_by(&self, time: usize) -> usize {
        if self.time >= time {
            self.total
        } else {
            self.total + (time - self.time) * self.rate
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_by(30).cmp(&other.total_by(30))
    }
}

#[derive(PartialEq, Eq, Default, Debug)]
struct FinalState {
    total: usize,
    seen: Vec<usize>,
}

impl PartialOrd for FinalState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total.partial_cmp(&other.total)
    }
}

impl Ord for FinalState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total.cmp(&other.total)
    }
}

fn get_shortest_distances(input: &Input, nodes: &HashMap<String, usize>) -> Array2<usize> {
    let mut grid = Array2::<usize>::from_shape_simple_fn((input.len(), input.len()), || usize::MAX);

    for (node, (_, edges)) in input {
        for edge in edges {
            grid[(nodes[node], nodes[edge])] = 1;
        }

        grid[(nodes[node], nodes[node])] = 0;
    }

    for k in 0..nodes.len() {
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if grid[(i, k)] == usize::MAX || grid[(k, j)] == usize::MAX {
                    continue;
                }
                if grid[(i, j)] > grid[(i, k)] + grid[(k, j)] {
                    grid[(i, j)] = grid[(i, k)] + grid[(k, j)];
                }
            }
        }
    }

    grid
}

fn prefix_match(map: &[(String, (usize, Vec<usize>))], full: &[&str], prefix: &[usize]) -> bool {
    prefix
        .iter()
        .enumerate()
        .all(|(idx, node)| map[*node].0 == full[idx])
}

pub fn solve(input: &Input) -> Output {
    let full_result = vec!["AA", "CA", "JF", "LE", "FP", "YH", "UX", "AR", "DM"];
    let nodes = input
        .iter()
        .enumerate()
        .map(|(idx, (name, _))| (name.clone(), idx))
        .collect::<HashMap<_, _>>();

    let mut nodes_vec: Vec<(String, (usize, Vec<usize>))> = vec![Default::default(); input.len()];
    for (node, idx) in &nodes {
        let tunnels = input[node].1.iter().map(|node| nodes[node]).collect();
        nodes_vec[*idx] = (node.clone(), (input[node].0, tunnels));
    }

    let useful_nodes = nodes_vec
        .iter()
        .enumerate()
        .filter(|(idx, (name, (rate, _)))| *name == "AA" || *rate > 0)
        .collect::<Vec<_>>();

    let grid = get_shortest_distances(input, &nodes);

    // println!("{:?}", nodes);
    // println!("{}", grid);

    let (_, edges) = &input["AA"];
    let mut queue: BinaryHeap<_> = nodes
        .iter()
        .filter(|(node, _)| *node != "AA")
        .map(|(node, idx)| State {
            node: nodes["AA"],
            time: 0,
            total: 0,
            rate: 0,
            target: *idx,
            remaining: grid[(nodes["AA"], *idx)],
            seen: Vec::new(),
        })
        .collect();

    let mut results = BinaryHeap::new();

    while !queue.is_empty() && queue.peek().unwrap().time < 30 {
        let mut state = queue.pop().unwrap();

        state.total += state.rate;
        state.time += 1;

        if state.time == 30 {
            // what if we just wait here?
            let mut seen = state.seen.clone();
            seen.push(state.node);

            results.push(FinalState {
                total: state.total,
                seen,
            });
        } else if state.remaining == 0 {
            // if state.node == nodes["AA"] && state.target == nodes["CA"] {
            //     println!("found the right path");
            // }
            state.rate += nodes_vec[state.target].1 .0;
            state.seen.push(state.node);
            state.node = state.target;

            // if prefix_match(&nodes_vec, &full_result, &state.seen) {
            //     let path = state
            //         .seen
            //         .iter()
            //         .map(|node| nodes_vec[*node].0.clone())
            //         .reduce(|a, b| a + " -> " + &b)
            //         .unwrap();
            //     println!(
            //         "{} @ {} w/ total {}, rate {}, seen: {path}",
            //         nodes_vec[state.node].0, state.time, state.total, state.rate
            //     );
            // }

            // attempt to traverse to other nodes
            useful_nodes
                .iter()
                .filter(|(node, _)| {
                    !state.seen.contains(node)
                        && *node != state.node
                        && state.time + grid[(state.node, *node)] < 30
                })
                .for_each(|(node, (node_name, (rate, _)))| {
                    let dist = grid[(state.node, *node)];

                    // println!(
                    //     "trying {} to {node_name} @ {}",
                    //     nodes_vec[state.node].0,
                    //     state.time + dist
                    // );

                    let new_state = State {
                        node: state.node,
                        time: state.time,
                        rate: state.rate,
                        total: state.total,
                        remaining: dist,
                        target: *node,
                        seen: state.seen.clone(),
                    };

                    // println!("predicting {} at t=30", new_state.total);

                    queue.push(new_state);
                });

            // what if we just wait here?
            let final_total = state.total + (30 - state.time) * state.rate;
            let mut seen = state.seen.clone();
            seen.push(state.node);

            if final_total == 1720 {
                println!("found correct value after {} results", results.len());
            }

            results.push(FinalState {
                total: state.total + (30 - state.time) * state.rate,
                seen,
            });
        } else {
            state.remaining -= 1;
            queue.push(state);
        }
    }

    println!("Total Candidates: {}", results.len());
    if let Some(state) = results.peek() {
        dbg!(state);
        let path = state
            .seen
            .iter()
            .map(|node| nodes_vec[*node].0.clone())
            .reduce(|a, b| a + " -> " + &b)
            .unwrap();
        println!("{path}");

        state.total.into()
    } else {
        0.into()
    }
}
