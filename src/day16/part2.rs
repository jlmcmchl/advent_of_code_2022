use std::collections::{BinaryHeap, HashMap};

use itertools::{Itertools, MultiProduct};
use ndarray::Array2;

use crate::day16::{Input, Output};

#[derive(PartialEq, Eq, Default, Debug, Clone, Copy)]
struct Crawler {
    node: usize,
    target: usize,
    remaining: usize,
}

#[derive(PartialEq, Eq, Default, Debug)]
struct State {
    you: Crawler,
    elephant: Crawler,
    time: usize,
    total: usize,
    rate: usize,
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
        (self.total_by(26) - self.total).cmp(&(other.total_by(26) - other.total))
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

pub fn solve(input: &Input) -> Output {
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

    let valves = useful_nodes.iter().filter(|(_, (node, _))| *node != "AA");

    let mut queue: BinaryHeap<_> = valves
        .combinations(2)
        .flat_map(|pair| {
            [
                State {
                    you: Crawler { node: nodes["AA"], target: pair[0].0, remaining: grid[(nodes["AA"], pair[0].0)] },
                    elephant: Crawler { node: nodes["AA"], target: pair[1].0, remaining: grid[(nodes["AA"], pair[1].0)] },
                    ..Default::default()
                },
                State {
                    you: Crawler { node: nodes["AA"], target: pair[1].0, remaining: grid[(nodes["AA"], pair[1].0)] },
                    elephant: Crawler { node: nodes["AA"], target: pair[0].0, remaining: grid[(nodes["AA"], pair[0].0)] },
                    ..Default::default()
                },
            ]
        })
        .collect();

    let mut results = BinaryHeap::new();

    while !queue.is_empty() && queue.peek().unwrap().time < 30 {
        let mut state = queue.pop().unwrap();

        state.total += state.rate;
        state.time += 1;

        if state.time == 26 {
            // done

            let mut seen = state.seen.clone();
            seen.push(state.you.node);
            seen.push(state.elephant.node);

            results.push(FinalState {
                total: state.total,
                seen,
            });
        } else if state.you.remaining == 0 && state.elephant.remaining == 0 {
            // special case
            state.rate += nodes_vec[state.you.target].1 .0;
            state.rate += nodes_vec[state.elephant.target].1 .0;
            state.seen.push(state.you.node);
            state.seen.push(state.elephant.node);
            state.you.node = state.you.target;
            state.elephant.node = state.elephant.target;

            let all_targets = useful_nodes
                .iter()
                .filter(|(node, _)| {
                    !state.seen.contains(node)
                        && *node != state.you.node
                        && *node != state.elephant.node
                        && *node != state.you.target
                        && *node != state.elephant.target
                })
                .collect::<Vec<_>>();

            all_targets.iter().combinations(2).for_each(|pair| {
                queue.push(State {
                    you: Crawler { node: state.you.node, target: pair[0].0, remaining: grid[(state.you.node, pair[0].0)] },
                    elephant: Crawler { node: state.elephant.node, target: pair[1].0, remaining: grid[(state.elephant.node, pair[1].0)]},
                    time: state.time,
                    rate: state.rate,
                    total: state.total,
                    seen: state.seen.clone(),
                });
                queue.push(State {
                    you: Crawler { node: state.you.node, target: pair[1].0, remaining: grid[(state.you.node, pair[1].0)] },
                    elephant: Crawler { node: state.elephant.node, target: pair[0].0, remaining: grid[(state.elephant.node, pair[0].0)]},
                    time: state.time,
                    rate: state.rate,
                    total: state.total,
                    seen: state.seen.clone(),
                });
            });
        } else if state.you.remaining == 0 {
            state.rate += nodes_vec[state.you.target].1 .0;
            state.seen.push(state.you.node);
            state.you.node = state.you.target;

            // attempt to traverse to other nodes
            useful_nodes
                .iter()
                .filter(|(node, _)| {
                    !state.seen.contains(node)
                        && *node != state.you.node
                        && *node != state.elephant.node
                        && *node != state.elephant.target
                })
                .for_each(|(node, (node_name, (rate, _)))| {
                    let dist = grid[(state.you.node, *node)];

                    // if state.node.1 == nodes["BB"] && *node == nodes["CC"] {
                    //     dbg!(&state);
                    // }

                    // println!(
                    //     "trying {} to {node_name} @ {}",
                    //     nodes_vec[state.node].0,
                    //     state.time + dist
                    // );

                    let new_state = State {
                        you: Crawler {node: state.you.node, target: *node, remaining: dist},
                        elephant: state.elephant,
                        time: state.time,
                        rate: state.rate,
                        total: state.total,
                        seen: state.seen.clone(),
                    };

                    // println!("predicting {} at t=30", new_state.total);

                    queue.push(new_state);
                });

            // what if we just wait here?
            let final_total = state.total + (26 - state.time) * state.rate;
            let mut seen = state.seen.clone();
            seen.push(state.elephant.node);

            results.push(FinalState {
                total: state.total + (26 - state.time) * state.rate,
                seen,
            });

            state.you.remaining = 99;
            queue.push(state);
        } else if state.elephant.remaining == 0 {
            state.rate += nodes_vec[state.elephant.target].1 .0;
            state.seen.push(state.elephant.node);
            state.elephant.node = state.elephant.target;

            // attempt to traverse to other nodes
            useful_nodes
                .iter()
                .filter(|(node, _)| {
                    !state.seen.contains(node)
                        && *node != state.you.node
                        && *node != state.elephant.node
                        && *node != state.you.target
                })
                .for_each(|(node, (node_name, (rate, _)))| {
                    let dist = grid[(state.elephant.node, *node)];

                    // if state.node.1 == nodes["BB"] && *node == nodes["CC"] {
                    //     dbg!(&state);
                    // }

                    // println!(
                    //     "trying {} to {node_name} @ {}",
                    //     nodes_vec[state.node].0,
                    //     state.time + dist
                    // );

                    let new_state = State {
                        you: state.you,
                        elephant: Crawler {node: state.you.node, target: *node, remaining: dist},
                        time: state.time,
                        rate: state.rate,
                        total: state.total,
                        seen: state.seen.clone(),
                    };

                    // println!("predicting {} at t=30", new_state.total);

                    queue.push(new_state);
                });

            // what if we just wait here?
            let final_total = state.total + (26 - state.time) * state.rate;
            let mut seen = state.seen.clone();
            seen.push(state.you.node);

            results.push(FinalState {
                total: state.total + (26 - state.time) * state.rate,
                seen,
            });

            state.elephant.remaining = 99;
            queue.push(state);
        } else {
            state.you.remaining -= 1;
            state.elephant.remaining -= 1;
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
