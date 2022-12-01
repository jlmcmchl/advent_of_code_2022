use std::collections::BinaryHeap;

use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut elves = BinaryHeap::new();

    for elf in input {
        elves.push(*elf);
    }

    Output::USize((0..3).map(|i| elves.pop().unwrap()).sum())
}
