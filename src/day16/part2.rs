use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

use itertools::Itertools;
use ndarray::Array2;

use crate::day16::{Input, Output};

#[derive(PartialEq, Eq, Default, Debug)]
struct State {
    node: usize,
    time: usize,
    total: usize,
    seen: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

#[derive(PartialEq, Eq, Default, Debug)]
struct FinalState {
    total: usize,
    seen: usize,
}

impl PartialOrd for FinalState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

fn solve_for_target_valves(
    nodes: &HashMap<String, usize>,
    useful_nodes: &[(usize, &(String, (usize, Vec<usize>)))],
    nodes_vec: &[(String, (usize, Vec<usize>))],
    grid: &Array2<usize>,
    time_limit: usize,
) -> usize {
    let mut queue = BinaryHeap::new();
    let mut results = BinaryHeap::new();

    queue.push(State {
        node: nodes["AA"],
        ..Default::default()
    });

    while let Some(mut state) = queue.pop() {
        useful_nodes
            .iter()
            .filter(|(node, _)| state.seen & (1 << node) == 0 && *node != state.node)
            .for_each(|(node, (_, (rate, _)))| {
                let dist = grid[(state.node, *node)];
                let time = state.time + dist + 1;

                if time >= time_limit {
                    return;
                }

                let total = state.total + rate * (time_limit - time);
                let seen = state.seen | (1 << state.node);

                let new_state = State {
                    node: *node,
                    time,
                    total,
                    seen,
                };

                queue.push(new_state);
            });

        let seen = state.seen | (1 << state.node);

        results.push(FinalState {
            total: state.total,
            seen,
        });
    }

    if let Some(state) = results.peek() {
        state.total
    } else {
        0
    }
}

pub fn solve(input: &Input) -> Output {
    let start = Instant::now();

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

    let active_valves = useful_nodes
        .iter()
        .filter(|(_, (name, _))| name != "AA")
        .map(|(idx, _)| *idx)
        .collect::<Vec<_>>();

    let total_signature: usize = active_valves.iter().fold(0, |agg, idx| agg | (1 << *idx));

    let results = active_valves
        .iter()
        .powerset()
        .map(|valves| {
            let signature: usize = valves.iter().fold(0, |agg, idx| agg | (1 << **idx));
            let currently_useful_nodes = useful_nodes
                .iter()
                .filter(|(idx, (name, _))| signature & (1 << idx) != 0 || name == "AA")
                .cloned()
                .collect::<Vec<_>>();
            (
                signature,
                solve_for_target_valves(&nodes, &currently_useful_nodes, &nodes_vec, &grid, 26),
            )
        })
        .collect::<HashMap<_, _>>();

    if let Some((best, signature)) = results
        .iter()
        .map(|(signature, score)| (results[&(total_signature ^ *signature)] + score, signature))
        .max()
    {
        println!("took: {:?}", Instant::now() - start);
        println!("signature: {signature:x} {signature}");

        for (i, val) in nodes_vec.iter().enumerate() {
            if signature & (1 << i) != 0 {
                println!("{}", val.0);
            }
        }

        let other = total_signature ^ signature;

        println!("other signature: {other:x} {other}");
        for (i, val) in nodes_vec.iter().enumerate() {
            if other & (1 << i) != 0 {
                println!("{}", val.0);
            }
        }

        best
    } else {
        0
    }
    .into()
}
