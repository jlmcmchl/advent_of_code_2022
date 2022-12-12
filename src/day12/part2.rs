use std::collections::{BinaryHeap, HashSet};

use ndarray::Array2;

use crate::day12::{Input, Output};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct HeapItem {
    steps: usize,
    distance: usize,
    coord: (usize, usize),
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self
            .steps
            .partial_cmp(&other.steps)
            .and_then(|ord| Some(ord.reverse()))
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self
            .distance
            .partial_cmp(&other.distance)
            .and_then(|ord| Some(ord.reverse()))
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.coord.partial_cmp(&other.coord)
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps
            .cmp(&other.steps)
            .reverse()
            .then(self.distance.cmp(&other.distance).reverse())
            .then(self.coord.cmp(&other.coord))
    }
}

fn distance(beg: &(usize, usize), end: &(usize, usize)) -> usize {
    end.0.abs_diff(beg.0) + end.1.abs_diff(beg.1)
}

fn adjacent((x, y): &(usize, usize), shape: &[usize]) -> Vec<(usize, usize)> {
    let sx = shape[0];
    let sy = shape[1];

    let mut ret = Vec::new();

    if *x > 0 {
        ret.push((x - 1, *y));
    }

    if x + 1 < sx {
        ret.push((x + 1, *y));
    }

    if *y > 0 {
        ret.push((*x, y - 1));
    }

    if y + 1 < sy {
        ret.push((*x, y + 1));
    }

    ret
}

pub fn solve_p1(input: &Input) -> Option<usize> {
    let (start_coord, _) = input
        .indexed_iter()
        .filter(|(d, val)| matches!(val, super::Node::Start))
        .next()
        .unwrap();

    let (end_coord, _) = input
        .indexed_iter()
        .filter(|(d, val)| matches!(val, super::Node::End))
        .next()
        .unwrap();

    let mut queue = BinaryHeap::new();

    queue.push(HeapItem {
        distance: distance(&start_coord, &end_coord),
        steps: 0,
        coord: start_coord,
    });

    let mut seen = HashSet::new();

    while queue.len() > 0 && queue.peek().unwrap().distance > 0 {
        let item = queue.pop().unwrap();

        if seen.contains(&item.coord) {
            continue;
        }

        seen.insert(item.coord);

        // println!("starting with {:?}", item);

        let nearby = adjacent(&item.coord, input.shape());

        nearby
            .iter()
            .filter(|coord| !seen.contains(*coord))
            .filter(|coord| {
                let prev = match input[item.coord] {
                    super::Node::Start => 0,
                    super::Node::Path(v) => v,
                    super::Node::End => b'z' - b'a',
                };
                let next = match input[**coord] {
                    super::Node::Start => 0,
                    super::Node::Path(v) => v,
                    super::Node::End => b'z' - b'a',
                };
                prev + 1 >= next
            })
            .map(|coord| HeapItem {
                distance: distance(coord, &end_coord),
                steps: item.steps + 1,
                coord: *coord,
            })
            // .inspect(|val| println!("moving to {:?}", val))
            .for_each(|next| queue.push(next));
    }

    if queue.len() > 0 {
        Some(queue.peek().unwrap().steps)
    } else {
        None
    }
}

pub fn solve(input: &Input) -> Output {
    let candidates: Vec<_> = input
        .indexed_iter()
        .filter(|(_, node)| matches!(node, super::Node::Start | super::Node::Path(0)))
        .map(|(coord, node)| coord)
        .collect();

    let mut alt_input = input.clone();

    let (start_coord, _) = input
        .indexed_iter()
        .filter(|(d, val)| matches!(val, super::Node::Start))
        .next()
        .unwrap();

    alt_input[start_coord] = super::Node::Path(0);

    candidates
        .iter()
        .filter_map(|candidate| {
            let mut inp = alt_input.clone();
            inp[*candidate] = super::Node::Start;

            solve_p1(&inp)
        })
        .min()
        .unwrap()
        .into()
}
