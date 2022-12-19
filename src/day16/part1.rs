use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

use ndarray::Array2;

use crate::day16::{Input, Output};

#[derive(PartialEq, Eq, Default, Debug)]
struct State {
    node: usize,
    time: usize,
    total: usize,
    seen: Vec<usize>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total.cmp(&other.total)
    }
}

#[derive(PartialEq, Eq, Default, Debug)]
struct FinalState {
    total: usize,
    seen: Vec<usize>
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

pub fn solve(input: &Input) -> Output {
    let start = Instant::now();
    let nodes = input
        .iter()
        .enumerate()
        .map(|(idx, (name, _))| (name.clone(), idx))
        .collect::<HashMap<_, _>>();

    let mut nodes_vec: Vec<String> = vec!["".to_string(); input.len()];
    for (node, idx) in &nodes {
        nodes_vec[*idx] = node.clone();
    }

    let useful_nodes = input
        .iter()
        .filter(|(name, (rate, _))| *name == "AA" || *rate > 0)
        .collect::<Vec<_>>();

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

    // println!("{:?}", nodes);
    // println!("{}", grid);

    let mut queue = BinaryHeap::new();
    let mut results = BinaryHeap::new();

    queue.push(State {
        node: nodes["AA"],
        ..Default::default()
    });

    while !queue.is_empty() && queue.peek().unwrap().time < 30 {
        let mut state = queue.pop().unwrap();

        useful_nodes
            .iter()
            .filter(|(node, _)| !state.seen.contains(&nodes[*node]) 
                                          && nodes[*node] != state.node
                                          && state.time + grid[(state.node, nodes[*node])] < 30)
            .for_each(|(node_name, (rate, _))| {
                let node = nodes[*node_name];
                let dist = grid[(state.node, node)];
                let time = state.time + dist + 1;
                let total = state.total + rate * (30 - time);
                let mut seen = state.seen.clone();
                seen.push(state.node);

                // println!("trying {} to {node_name} @ {}", nodes_vec[state.node], time);

                let new_state = State {
                    node,
                    time,
                    total,
                    seen,
                };

                // println!("predicting {} at t=30", new_state.total);


                queue.push(new_state);
            });
        
        
        let mut seen = state.seen.clone();
        seen.push(state.node);

        results.push(FinalState {
            total: state.total,
            seen: seen
        });
    }
    
    println!("dur: {:?}", Instant::now() - start);

    println!("Total Candidates: {}", results.len());
    if let Some(state) = results.peek() {
        dbg!(state);
        let path = state.seen.iter().map(|node| nodes_vec[*node].clone()).reduce(|a, b| a + " -> " + &b).unwrap();
        println!("{path}");

        state.total.into()
    } else {
        0.into()
    }
}
